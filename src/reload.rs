use std::process::Command;

use crate::app::App;
use crate::desktop;
use crate::home::Home;
use crate::live;

pub fn reload(home: &Home, app: &App) -> Result<String, String> {
    app.require()?;
    let name = app.name();
    check(name)?;
    live::sync(app.directory(), &home.live(name))
        .map_err(|error| format!("cannot update the live copy of {name}: {error}"))?;
    let Some(process) = desktop::process(&app.id())? else {
        return Ok(format!("{name} is not running; it starts with the new code"));
    };
    if !desktop::arguments(&process).contains("--live") {
        return Err(format!(
            "{name} is running, but it was installed by an older bmo and cannot reload: run bmo install {name}, then quit and reopen it once"
        ));
    }
    Ok(format!("reloaded {name}"))
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
