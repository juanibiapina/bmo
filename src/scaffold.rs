use std::io;

use crate::app::App;

const MAIN: &str = include_str!("../template/main.js");
const KEYS: &str = include_str!("../template/keys.js");
const MANIFEST: &str = include_str!("../template/gpui-shell.json");
const SETTINGS: &str = include_str!("../template/bmo.json");
const IGNORED: &str = include_str!("../template/gitignore");

pub fn create(app: &App) -> Result<String, String> {
    let directory = app.directory();
    if directory.exists() {
        return Err(format!(
            "{} already exists at {}",
            app.name(),
            directory.display()
        ));
    }
    write(app).map_err(|error| format!("cannot create {}: {error}", directory.display()))?;
    Ok(format!("created {}", directory.display()))
}

fn write(app: &App) -> io::Result<()> {
    std::fs::create_dir_all(app.directory())?;
    std::fs::write(app.directory().join("gpui-shell.json"), fill(MANIFEST, app))?;
    std::fs::write(app.directory().join("bmo.json"), SETTINGS)?;
    std::fs::write(app.directory().join("keys.js"), KEYS)?;
    std::fs::write(app.directory().join(".gitignore"), IGNORED)?;
    std::fs::write(app.directory().join("main.js"), fill(MAIN, app))
}

fn fill(template: &str, app: &App) -> String {
    template
        .replace("{{id}}", &app.id())
        .replace("{{title}}", &app.title())
}
