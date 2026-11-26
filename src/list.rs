use crate::app::App;
use crate::home::Home;

pub fn list(home: &Home) -> Result<String, String> {
    let folder = home.root().join("apps");
    let mut names: Vec<String> = std::fs::read_dir(&folder)
        .map(|entries| {
            entries
                .flatten()
                .filter(|entry| entry.path().join("main.js").is_file())
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    if names.is_empty() {
        return Ok(format!("no apps yet in {}", folder.display()));
    }
    let lines: Vec<String> = names
        .iter()
        .filter_map(|name| App::named(home, name).ok())
        .map(|app| {
            let installed = home.applications().join(format!("{}.app", app.title())).is_dir();
            format!(
                "{:<16} {:<24} {}",
                app.name(),
                app.id(),
                if installed { "installed" } else { "" }
            )
            .trim_end()
            .to_string()
        })
        .collect();
    Ok(lines.join("\n"))
}
