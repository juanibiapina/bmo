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
        let mut characters = self.name.chars();
        match characters.next() {
            Some(first) => first.to_uppercase().chain(characters).collect(),
            None => String::new(),
        }
    }
}

fn is_valid(name: &str) -> bool {
    name.starts_with(|first: char| first.is_ascii_lowercase())
        && name.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}
