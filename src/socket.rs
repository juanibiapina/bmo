use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

const CONNECT_LIMIT: Duration = Duration::from_secs(10);

struct Connection {
    stream: TcpStream,
    incoming: async_channel::Receiver<String>,
}

fn open() -> std::sync::MutexGuard<'static, HashMap<u64, Connection>> {
    static OPEN: OnceLock<Mutex<HashMap<u64, Connection>>> = OnceLock::new();
    OPEN.get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub fn connect(host: &str, port: u16) -> Result<u64, String> {
    let problem = |error: &dyn std::fmt::Display| format!("cannot connect to {host}:{port}: {error}");
    let addresses = (host, port).to_socket_addrs().map_err(|error| problem(&error))?;
    let mut last = problem(&"no address");
    for address in addresses {
        match TcpStream::connect_timeout(&address, CONNECT_LIMIT) {
            Ok(stream) => return keep(stream).map_err(|error| problem(&error)),
            Err(error) => last = problem(&error),
        }
    }
    Err(last)
}

fn keep(stream: TcpStream) -> std::io::Result<u64> {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let (sender, incoming) = async_channel::unbounded();
    let reader = stream.try_clone()?;
    std::thread::spawn(move || relay(reader, sender));
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    open().insert(id, Connection { stream, incoming });
    Ok(id)
}

/// Passes on what arrives as text, holding back a character cut in half by
/// the network until the rest of it comes.
fn relay(mut stream: TcpStream, sender: async_channel::Sender<String>) {
    let mut waiting = Vec::new();
    let mut chunk = [0u8; 64 * 1024];
    loop {
        let read = match stream.read(&mut chunk) {
            Ok(0) | Err(_) => return,
            Ok(read) => read,
        };
        waiting.extend_from_slice(&chunk[..read]);
        let whole = match std::str::from_utf8(&waiting) {
            Ok(_) => waiting.len(),
            Err(error) if error.error_len().is_none() => error.valid_up_to(),
            Err(_) => waiting.len(),
        };
        if whole == 0 {
            continue;
        }
        let text = String::from_utf8_lossy(&waiting[..whole]).into_owned();
        waiting.drain(..whole);
        if sender.send_blocking(text).is_err() {
            return;
        }
    }
}

pub async fn read(id: u64) -> Result<Option<String>, String> {
    let incoming = open().get(&id).ok_or_else(|| unknown(id))?.incoming.clone();
    Ok(incoming.recv().await.ok())
}

pub fn write(id: u64, text: &str) -> Result<(), String> {
    let mut stream = open()
        .get(&id)
        .ok_or_else(|| unknown(id))?
        .stream
        .try_clone()
        .map_err(|error| error.to_string())?;
    stream
        .write_all(text.as_bytes())
        .map_err(|error| format!("cannot write to the connection: {error}"))
}

pub fn close(id: u64) {
    if let Some(connection) = open().remove(&id) {
        let _ = connection.stream.shutdown(Shutdown::Both);
    }
}

fn unknown(id: u64) -> String {
    format!("{id} is not an open connection")
}
