use std::io::Read;
use std::path::Path;
use std::time::Duration;

pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub limit: Duration,
}

pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

pub fn fetch(request: Request) -> Result<Response, String> {
    let mut call = ureq::request(&request.method, &request.url).timeout(request.limit);
    for (name, value) in &request.headers {
        call = call.set(name, value);
    }
    let answered = match request.body {
        Some(body) => call.send_string(&body),
        None => call.call(),
    };
    let response = match answered {
        Ok(response) | Err(ureq::Error::Status(_, response)) => response,
        Err(error) => return Err(format!("cannot reach {}: {error}", request.url)),
    };
    let status = response.status();
    let headers = response
        .headers_names()
        .into_iter()
        .filter_map(|name| {
            let value = response.header(&name)?.to_string();
            Some((name.to_lowercase(), value))
        })
        .collect();
    let mut body = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut body)
        .map_err(|error| format!("cannot read the response from {}: {error}", request.url))?;
    Ok(Response {
        status,
        headers,
        body,
    })
}

pub fn save(response: &Response, file: &Path) -> Result<(), String> {
    let written = || {
        if let Some(folder) = file.parent() {
            std::fs::create_dir_all(folder)?;
        }
        std::fs::write(file, &response.body)
    };
    written().map_err(|error| format!("cannot write {}: {error}", file.display()))
}
