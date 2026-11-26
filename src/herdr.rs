use std::time::Duration;

use serde_json::{Value, json};

use crate::exec;

pub struct NewSession {
    pub folder: Option<String>,
    pub task: Option<String>,
    pub name: Option<String>,
    pub agent: String,
}

pub fn sessions() -> Result<Value, String> {
    let listed = parsed(&call("hc", &["ls", "--json"])?)?;
    Ok(listed.get("terminals").cloned().unwrap_or_else(|| json!([])))
}

pub fn focused() -> Result<Option<String>, String> {
    let status = parsed(&call("herdr-app", &["status"])?)?;
    Ok(status["agent"].as_str().map(str::to_string))
}

pub fn send(address: &str, text: &str) -> Result<(), String> {
    call("hc", &["send", address, text]).map(drop)
}

pub fn show(address: &str) -> Result<(), String> {
    call("herdr-app", &["do", "agent.focus", "--terminal", address])?;
    call("open", &["-b", "dev.herdr.app"]).map(drop)
}

pub fn start(session: NewSession) -> Result<(), String> {
    let mut space = json!({ "machine": "", "argv": [session.agent] });
    for (name, value) in [
        ("folder", session.folder),
        ("task", session.task),
        ("name", session.name),
    ] {
        if let Some(value) = value {
            space[name] = json!(value);
        }
    }
    call("herdr-app", &["do", "space.new", &space.to_string()]).map(drop)
}

fn call(program: &str, arguments: &[&str]) -> Result<String, String> {
    let output = exec::run(exec::Request {
        program: program.into(),
        arguments: arguments.iter().map(|argument| argument.to_string()).collect(),
        folder: None,
        input: None,
        limit: Duration::from_secs(30),
    })
    .map_err(|error| format!("Herdr is not installed or did not answer: {error}"))?;
    if output.code != 0 {
        return Err(format!(
            "{program} {} failed: {}",
            arguments.first().unwrap_or(&""),
            output.stderr.trim()
        ));
    }
    Ok(output.stdout)
}

fn parsed(text: &str) -> Result<Value, String> {
    serde_json::from_str(text).map_err(|error| format!("Herdr answered with something unexpected: {error}"))
}
