use std::path::PathBuf;

use crate::home::Home;

pub struct App {
    name: String,
    directory: PathBuf,
}

impl App {
    pub fn named(home: &Home, name: &str) -> Result<Self, String> {
        if !is_valid(name) {
            return Err(format!(
                "`{name}` is not a valid name: use lowercase letters, digits and dashes, starting with a letter"
            ));
        }
        Ok(Self {
            name: name.to_string(),
            directory: home.app(name),
        })
    }

    /// The same app, with its code read from another folder.
    pub fn at(&self, directory: PathBuf) -> Self {
        Self {
            name: self.name.clone(),
            directory,
        }
    }

    pub fn require(&self) -> Result<(), String> {
        if self.directory.is_dir() {
            return Ok(());
        }
        Err(format!(
            "no app named {}: {} does not exist",
            self.name,
            self.directory.display()
        ))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn directory(&self) -> &PathBuf {
        &self.directory
    }

    pub fn id(&self) -> String {
        format!("bmo.{}", self.name)
    }

    pub fn title(&self) -> String {
        self.manifest_name().unwrap_or_else(|| {
            let mut characters = self.name.chars();
            match characters.next() {
                Some(first) => first.to_uppercase().chain(characters).collect(),
                None => String::new(),
            }
        })
    }

    fn manifest_name(&self) -> Option<String> {
        let text = std::fs::read_to_string(self.directory.join("gpui-shell.json")).ok()?;
        let manifest: serde_json::Value = serde_json::from_str(&text).ok()?;
        manifest["name"].as_str().map(str::to_string)
    }
}

fn is_valid(name: &str) -> bool {
    name.starts_with(|first: char| first.is_ascii_lowercase())
        && name.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}
