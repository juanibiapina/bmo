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
        arguments: vec![
            "run".into(),
            "--dir".into(),
            app.directory().display().to_string(),
            "--watch".into(),
            "--dev".into(),
        ],
    }
    .write_into(&folder)?;
    let log = folder.join(format!("{}.log", app.name()));
    desktop::open(&bundle, &log)?;
    let window = desktop::wait_until(&id(app), Duration::from_secs(10), Option::is_some)?
        .ok_or(format!(
            "{} did not open a window; see {}",
            app.name(),
            log.display()
        ))?;
    desktop::move_to_workspace(&window, WORKSPACE)?;
    Ok(format!(
        "{} is running on the {WORKSPACE} workspace and reloads on save\nlog: {}",
        app.name(),
        log.display()
    ))
}

pub fn stop(app: &App) -> Result<String, String> {
    desktop::quit(&running(app)?)?;
    desktop::wait_until(&id(app), Duration::from_secs(5), Option::is_none)?;
    Ok(format!("stopped {}", app.name()))
}

fn running(app: &App) -> Result<desktop::Window, String> {
    desktop::window(&id(app))?.ok_or(format!(
        "{name} is not running: start it with bmo dev {name}",
        name = app.name()
    ))
}

fn id(app: &App) -> String {
    format!("{}.dev", app.id())
}

fn folder(home: &Home) -> PathBuf {
    home.cache().join("dev")
}
