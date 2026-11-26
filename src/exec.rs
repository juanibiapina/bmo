use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

use wait_timeout::ChildExt;

pub struct Request {
    pub program: String,
    pub arguments: Vec<String>,
    pub folder: Option<PathBuf>,
    pub input: Option<String>,
    pub limit: Duration,
}

pub struct Output {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub fn run(request: Request) -> Result<Output, String> {
    let program = &request.program;
    let mut command = Command::new(program);
    command
        .args(&request.arguments)
        .stdin(if request.input.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(folder) = &request.folder {
        command.current_dir(folder);
    }
    let mut child = command
        .spawn()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    if let (Some(input), Some(mut stdin)) = (request.input, child.stdin.take()) {
        std::thread::spawn(move || {
            let _ = stdin.write_all(input.as_bytes());
        });
    }
    let stdout = collect(child.stdout.take());
    let stderr = collect(child.stderr.take());
    let status = child
        .wait_timeout(request.limit)
        .map_err(|error| format!("cannot wait for {program}: {error}"))?;
    let Some(status) = status else {
        let _ = child.kill();
        let _ = child.wait();
        return Err(format!(
            "{program} did not finish in {} ms",
            request.limit.as_millis()
        ));
    };
    Ok(Output {
        code: status.code().unwrap_or(-1),
        stdout: stdout.join().unwrap_or_default(),
        stderr: stderr.join().unwrap_or_default(),
    })
}

fn collect(stream: Option<impl Read + Send + 'static>) -> std::thread::JoinHandle<String> {
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        if let Some(mut stream) = stream {
            let _ = stream.read_to_end(&mut bytes);
        }
        String::from_utf8_lossy(&bytes).into_owned()
    })
}
