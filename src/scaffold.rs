use crate::home::Home;

const MAIN: &str = include_str!("../template/main.js");

pub fn create(home: &Home, name: Option<&str>) -> Result<String, String> {
    let name = name.ok_or("new needs a name: bmo new <name>")?;
    let directory = home.app(name);
    std::fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    std::fs::write(directory.join("gpui-shell.json"), manifest(name))
        .map_err(|error| error.to_string())?;
    std::fs::write(directory.join("main.js"), MAIN.replace("{{title}}", &title(name)))
        .map_err(|error| error.to_string())?;
    Ok(format!("created {}", directory.display()))
}

fn manifest(name: &str) -> String {
    format!(
        "{{\n  \"id\": \"bmo.{name}\",\n  \"name\": \"{}\",\n  \"version\": \"0.1.0\",\n  \"entry\": \"main.js\",\n  \"capabilities\": {{\n    \"storage\": true\n  }}\n}}\n",
        title(name)
    )
}

fn title(name: &str) -> String {
    let mut characters = name.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect(),
        None => String::new(),
    }
}
