mod support;

use support::Bmo;
use support::server::{Reply, serve};

#[test]
fn fetch_sends_the_request_and_gives_the_response() {
    let url = serve(|request| Reply {
        status: 201,
        body: request.as_bytes().to_vec(),
    });

    Bmo::new().shows_green(
        "poster",
        &format!(
            r#"
            const response = await bmo.fetch("{url}/things?page=2", {{
              method: "POST",
              headers: {{ Cookie: "d=secret", Authorization: "Bearer token" }},
              body: "the body",
            }});
            const sent = response.body.toLowerCase();
            return response.status === 201 && response.ok
              && response.headers["x-test"] === "yes"
              && sent.startsWith("post /things?page=2 ")
              && sent.includes("cookie: d=secret")
              && sent.includes("authorization: bearer token")
              && sent.endsWith("the body");
            "#
        ),
    );
}

#[test]
fn fetch_resolves_when_the_server_refuses() {
    let url = serve(|_| Reply {
        status: 404,
        body: b"nothing here".to_vec(),
    });

    Bmo::new().shows_green(
        "refused",
        &format!(
            r#"
            const response = await bmo.fetch("{url}/missing");
            return response.status === 404 && !response.ok && response.body === "nothing here";
            "#
        ),
    );
}

#[test]
fn fetch_rejects_when_the_server_cannot_be_reached() {
    Bmo::new().shows_green(
        "unreachable",
        r#"
        try {
          await bmo.fetch("http://127.0.0.1:9/");
          return false;
        } catch (error) {
          return String(error).includes("cannot reach http://127.0.0.1:9/");
        }
        "#,
    );
}

#[test]
fn download_saves_the_bytes_to_a_file() {
    let url = serve(|_| Reply {
        status: 200,
        body: vec![0x89, b'P', b'N', b'G', 0xff, 0x00, 0xfe],
    });
    let bmo = Bmo::new();
    let saved = bmo.home().join("nested/folder/image.png");

    bmo.shows_green(
        "saver",
        &format!(
            r#"
            const result = await bmo.download("{url}/image.png", "{}");
            return result.status === 200 && result.ok;
            "#,
            saved.display()
        ),
    );

    assert_eq!(std::fs::read(saved).expect("the file"), vec![0x89, b'P', b'N', b'G', 0xff, 0x00, 0xfe]);
}
