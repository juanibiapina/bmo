use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::app::App;
use crate::home::Home;

const PLIST: &str = include_str!("../template/Info.plist");

pub fn install(home: &Home, app: &App) -> Result<String, String> {
    if !app.directory().is_dir() {
        return Err(format!(
            "no app named {}: {} does not exist",
            app.name(),
            app.directory().display()
        ));
    }
    let title = displayed_name(app)?;
    let bundle = home.applications().join(format!("{title}.app"));
    write(&bundle, app, &title).map_err(|error| format!("cannot write {}: {error}", bundle.display()))?;
    Ok(format!("installed {}", bundle.display()))
}

fn displayed_name(app: &App) -> Result<String, String> {
    let path = app.directory().join("gpui-shell.json");
    let problem = |error: String| format!("cannot read {}: {error}", path.display());
    let text = std::fs::read_to_string(&path).map_err(|error| problem(error.to_string()))?;
    let manifest: serde_json::Value =
        serde_json::from_str(&text).map_err(|error| problem(error.to_string()))?;
    Ok(manifest["name"]
        .as_str()
        .map(str::to_string)
        .unwrap_or_else(|| app.title()))
}

fn write(bundle: &Path, app: &App, title: &str) -> io::Result<()> {
    let contents = bundle.join("Contents");
    let executables = contents.join("MacOS");
    std::fs::create_dir_all(&executables)?;
    std::fs::write(
        contents.join("Info.plist"),
        PLIST
            .replace("{{id}}", &app.id())
            .replace("{{title}}", &escaped(title)),
    )?;
    let launcher = executables.join(title);
    std::fs::write(&launcher, launcher_script(app)?)?;
    std::fs::set_permissions(&launcher, std::fs::Permissions::from_mode(0o755))
}

fn launcher_script(app: &App) -> io::Result<String> {
    let bmo = std::env::current_exe()?;
    Ok(format!(
        "#!/bin/sh\nexec {} run --dir {} --packaged\n",
        quoted(&bmo),
        quoted(app.directory())
    ))
}

fn escaped(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn quoted(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', "'\\''"))
}
