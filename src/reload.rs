use std::process::Command;

use crate::app::App;
use crate::home::Home;
use crate::remote;

pub fn reload(home: &Home, app: &App) -> Result<String, String> {
    app.require()?;
    let name = app.name();
    check(name)?;
    match remote::ask_to_reload(&home.socket(name)) {
        None => Ok(format!("{name} is not running; it starts with the new code")),
        Some(Ok(())) => Ok(format!("reloaded {name}")),
        Some(Err(message)) => Err(format!(
            "{message}\n{name} could not reload; its window still runs the code from before"
        )),
    }
}

/// New code reaches a running window only after it passes the check.
fn check(name: &str) -> Result<(), String> {
    let bmo = std::env::current_exe().map_err(|error| format!("cannot find bmo: {error}"))?;
    let checked = Command::new(bmo)
        .args(["check", name])
        .output()
        .map_err(|error| format!("cannot run the check: {error}"))?;
    if checked.status.success() {
        return Ok(());
    }
    Err(format!(
        "{}\n{name} failed its check; the running app was left as it is",
        String::from_utf8_lossy(&checked.stderr).trim()
    ))
}
