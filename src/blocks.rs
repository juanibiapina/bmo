use std::time::Duration;

use gpui_shell::gpui::WindowAppearance;
use gpui_shell::{HostArguments, HostError, HostModule, HostObject, HostValue};

use crate::exec;
use crate::http;
use crate::theme::{self, Appearance};

const DECLARATIONS: &str = include_str!("blocks.d.ts");

pub fn export() -> Result<(), HostError> {
    gpui_shell::export_module(
        HostModule::new("bmo")
            .function("theme", |_: &HostArguments| Ok(colors()))
            .async_function("exec", |arguments: &HostArguments| {
                let request = exec_request(arguments)?;
                Ok(async move { exec::run(request).map(exec_output).map_err(HostError::new) })
            })
            .async_function("fetch", |arguments: &HostArguments| {
                let request = http_request(arguments, 1)?;
                Ok(async move {
                    http::fetch(request)
                        .map(|response| {
                            let body = String::from_utf8_lossy(&response.body).into_owned();
                            http_response(&response).field("body", body).into()
                        })
                        .map_err(HostError::new)
                })
            })
            .async_function("download", |arguments: &HostArguments| {
                let request = http_request(arguments, 2)?;
                let file = std::path::PathBuf::from(arguments.string(1)?);
                Ok(async move {
                    let response = http::fetch(request).map_err(HostError::new)?;
                    if (200..300).contains(&response.status) {
                        http::save(&response, &file).map_err(HostError::new)?;
                    }
                    Ok(http_response(&response).into())
                })
            })
            .declarations(DECLARATIONS),
    )
}

fn colors() -> HostValue {
    theme::colors(appearance())
        .into_iter()
        .fold(HostObject::new(), |object, (name, color)| object.field(name, color))
        .into()
}

fn appearance() -> Appearance {
    match gpui_shell::with_current_app(|app| app.window_appearance()) {
        Some(WindowAppearance::Light | WindowAppearance::VibrantLight) => Appearance::Light,
        _ => Appearance::Dark,
    }
}

fn exec_request(arguments: &HostArguments) -> Result<exec::Request, HostError> {
    let options = arguments.get(2);
    let text = |name: &str| options.and_then(|options| options.get(name)).and_then(HostValue::as_str);
    Ok(exec::Request {
        program: arguments.string(0)?.to_string(),
        arguments: strings(arguments.get(1), "exec's arguments")?,
        folder: text("cwd").map(Into::into),
        input: text("input").map(str::to_string),
        limit: Duration::from_millis(
            options
                .and_then(|options| options.get("timeout"))
                .and_then(HostValue::as_number)
                .unwrap_or(30_000.) as u64,
        ),
    })
}

fn exec_output(output: exec::Output) -> HostValue {
    HostObject::new()
        .field("code", output.code)
        .field("stdout", output.stdout)
        .field("stderr", output.stderr)
        .into()
}

fn http_request(arguments: &HostArguments, options: usize) -> Result<http::Request, HostError> {
    let options = arguments.get(options);
    let option = |name: &str| options.and_then(|options| options.get(name));
    let headers = option("headers")
        .and_then(HostValue::as_object)
        .unwrap_or_default()
        .iter()
        .filter_map(|(name, value)| Some((name.clone(), value.as_str()?.to_string())))
        .collect();
    Ok(http::Request {
        url: arguments.string(0)?.to_string(),
        method: option("method")
            .and_then(HostValue::as_str)
            .unwrap_or("GET")
            .to_uppercase(),
        headers,
        body: option("body").and_then(HostValue::as_str).map(str::to_string),
        limit: Duration::from_millis(
            option("timeout").and_then(HostValue::as_number).unwrap_or(30_000.) as u64,
        ),
    })
}

fn http_response(response: &http::Response) -> HostObject {
    let headers = response
        .headers
        .iter()
        .fold(HostObject::new(), |headers, (name, value)| headers.field(name, value.as_str()));
    HostObject::new()
        .field("status", response.status)
        .field("ok", (200..300).contains(&response.status))
        .field("headers", headers)
}

fn strings(value: Option<&HostValue>, what: &str) -> Result<Vec<String>, HostError> {
    let Some(value) = value.filter(|value| !value.is_null()) else {
        return Ok(Vec::new());
    };
    value
        .as_array()
        .and_then(|items| {
            items
                .iter()
                .map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .ok_or_else(|| HostError::new(format!("{what} must be a list of strings")))
}
