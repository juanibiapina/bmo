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
    let bundle = home.applications().join(format!("{}.app", app.title()));
    write(&bundle, app).map_err(|error| format!("cannot write {}: {error}", bundle.display()))?;
    Ok(format!("installed {}", bundle.display()))
}

fn write(bundle: &Path, app: &App) -> io::Result<()> {
    let contents = bundle.join("Contents");
    let executables = contents.join("MacOS");
    std::fs::create_dir_all(&executables)?;
    std::fs::write(
        contents.join("Info.plist"),
        PLIST
            .replace("{{id}}", &app.id())
            .replace("{{title}}", &app.title()),
    )?;
    let launcher = executables.join(app.title());
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

fn quoted(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', "'\\''"))
}
