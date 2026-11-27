use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::time::Duration;

const ANSWER_LIMIT: Duration = Duration::from_secs(30);
const DONE: &str = "done";

/// A request that reached a running app. Answer it once.
pub struct Request(UnixStream);

impl Request {
    pub fn answer(mut self, outcome: Result<(), String>) {
        // One line: the caller reads up to the first line break.
        let line = match outcome {
            Ok(()) => DONE.to_string(),
            Err(message) => message.replace('\n', "\u{1f}"),
        };
        let _ = writeln!(self.0, "{line}");
    }
}

/// Lets `bmo reload` reach this running app, through a socket in bmo's home.
/// A socket left by a copy that has gone is replaced.
pub fn listen(socket: &Path) -> std::io::Result<async_channel::Receiver<Request>> {
    if let Some(folder) = socket.parent() {
        std::fs::create_dir_all(folder)?;
    }
    let _ = std::fs::remove_file(socket);
    let listener = UnixListener::bind(socket)?;
    let (sender, requests) = async_channel::unbounded();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            if sender.send_blocking(Request(stream)).is_err() {
                return;
            }
        }
    });
    Ok(requests)
}

/// Asks the app behind the socket to reload. None when no app is listening.
pub fn ask_to_reload(socket: &Path) -> Option<Result<(), String>> {
    let stream = UnixStream::connect(socket).ok()?;
    let _ = stream.set_read_timeout(Some(ANSWER_LIMIT));
    let mut answer = String::new();
    if BufReader::new(stream).read_line(&mut answer).is_err() || answer.is_empty() {
        return Some(Err("the running app did not answer".into()));
    }
    let answer = answer.trim_end().replace('\u{1f}', "\n");
    Some(if answer == DONE { Ok(()) } else { Err(answer) })
}
