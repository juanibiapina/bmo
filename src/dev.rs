use std::path::PathBuf;
use std::time::Duration;

use crate::app::App;
use crate::bundle::Bundle;
use crate::desktop;
use crate::home::Home;

const WORKSPACE: &str = "sandbox";

pub fn start(home: &Home, app: &App) -> Result<String, String> {
    app.require()?;
    let folder = folder(home);
    let bundle = Bundle {
        title: format!("{} Dev", app.title()),
        id: id(app),
        home: home.root().clone(),
        arguments: vec!["run".into(), app.name().into(), "--watch".into()],
    }
    .write_into(&folder)?;
    let log = folder.join(format!("{}.log", app.name()));
    desktop::open(&bundle, &log)?;
    desktop::wait_until(&id(app), Duration::from_secs(10), Option::is_some)?.ok_or(format!(
        "{} did not start; see {}",
        app.name(),
        log.display()
    ))?;
    let place = if desktop::move_to_workspace(&id(app), WORKSPACE, Duration::from_secs(3)) {
        format!(" on the {WORKSPACE} workspace")
    } else {
        String::new()
    };
    Ok(format!(
        "{} is running{place} and reloads on save\nlog: {}",
        app.name(),
        log.display()
    ))
}

pub fn stop(app: &App) -> Result<String, String> {
    let process = desktop::process(&id(app))?.ok_or(format!(
        "{name} is not running: start it with bmo dev {name}",
        name = app.name()
    ))?;
    desktop::quit(&process)?;
    desktop::wait_until(&id(app), Duration::from_secs(5), Option::is_none)?;
    Ok(format!("stopped {}", app.name()))
}

fn id(app: &App) -> String {
    format!("{}.dev", app.id())
}

fn folder(home: &Home) -> PathBuf {
    home.cache().join("dev")
}
