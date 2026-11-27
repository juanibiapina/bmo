mod support;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use support::Bmo;

/// Answers each line with "pong: <line>", greets with "héllo", and hangs up
/// when it reads "bye".
fn line_server() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a free port");
    let port = listener.local_addr().expect("the port").port();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            std::thread::spawn(move || {
                let mut writer = stream.try_clone().expect("the stream twice");
                let _ = writer.write_all("héllo\n".as_bytes());
                for line in BufReader::new(stream).lines().map_while(Result::ok) {
                    if line == "bye" {
                        return;
                    }
                    let _ = writer.write_all(format!("pong: {line}\n").as_bytes());
                }
            });
        }
    });
    port
}

#[test]
fn an_app_talks_over_a_socket_without_asking() {
    let port = line_server();

    Bmo::new().shows_green(
        "talker",
        &format!(
            r#"
            const socket = await bmo.connect("127.0.0.1", {port});
            const greeting = await bmo.read(socket);
            await bmo.write(socket, "ping\n");
            const answer = await bmo.read(socket);
            await bmo.write(socket, "bye\n");
            const ended = await bmo.read(socket);
            bmo.close(socket);
            return greeting === "héllo\n" && answer === "pong: ping\n" && ended === null;
            "#
        ),
    );
}

#[test]
fn a_socket_nobody_answers_is_refused_with_a_reason() {
    let port = {
        let listener = TcpListener::bind("127.0.0.1:0").expect("a free port");
        listener.local_addr().expect("the port").port()
    };

    Bmo::new().shows_green(
        "alone",
        &format!(
            r#"
            try {{
              await bmo.connect("127.0.0.1", {port});
              return false;
            }} catch (error) {{
              return String(error).includes("cannot connect to 127.0.0.1:{port}");
            }}
            "#
        ),
    );
}

#[test]
fn a_closed_socket_cannot_be_used() {
    let port = line_server();

    Bmo::new().shows_green(
        "closer",
        &format!(
            r#"
            const socket = await bmo.connect("127.0.0.1", {port});
            bmo.close(socket);
            try {{
              await bmo.write(socket, "ping\n");
              return false;
            }} catch (error) {{
              return String(error).includes("is not an open connection");
            }}
            "#
        ),
    );
}
