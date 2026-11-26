use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct Reply {
    pub status: u16,
    pub body: Vec<u8>,
}

pub fn serve(reply: impl Fn(&str) -> Reply + Send + 'static) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a free port");
    let address = listener.local_addr().expect("the port");
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            answer(stream, &reply);
        }
    });
    format!("http://{address}")
}

fn answer(mut stream: TcpStream, reply: &impl Fn(&str) -> Reply) {
    let request = read_request(&mut stream);
    let reply = reply(&request);
    let head = format!(
        "HTTP/1.1 {} Status\r\nX-Test: yes\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        reply.status,
        reply.body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(&reply.body);
}

fn read_request(stream: &mut TcpStream) -> String {
    let mut bytes = Vec::new();
    let mut chunk = [0u8; 1024];
    loop {
        let text = String::from_utf8_lossy(&bytes).into_owned();
        if let Some((head, body)) = text.split_once("\r\n\r\n") {
            let wanted = head
                .lines()
                .find_map(|line| line.to_lowercase().strip_prefix("content-length:").map(|length| length.trim().parse::<usize>().unwrap_or(0)))
                .unwrap_or(0);
            if body.len() >= wanted {
                return text;
            }
        }
        match stream.read(&mut chunk) {
            Ok(0) | Err(_) => return text,
            Ok(read) => bytes.extend_from_slice(&chunk[..read]),
        }
    }
}
