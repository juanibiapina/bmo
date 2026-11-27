use std::path::Path;

use serde_json::Value;

/// What an app asks of bmo itself, from the `bmo.json` beside its code. The
/// runtime's manifest refuses keys it does not know, so these live apart.
pub struct Settings {
    pub title_bar: bool,
}

impl Settings {
    pub fn read(directory: &Path) -> Result<Self, String> {
        let mut settings = Self { title_bar: true };
        let Ok(text) = std::fs::read_to_string(directory.join("bmo.json")) else {
            return Ok(settings);
        };
        let parsed: Value =
            serde_json::from_str(&text).map_err(|error| format!("bmo.json is not JSON: {error}"))?;
        let fields = parsed.as_object().ok_or("bmo.json must be an object")?;
        for (name, value) in fields {
            match name.as_str() {
                "title_bar" => {
                    settings.title_bar = value
                        .as_bool()
                        .ok_or("`title_bar` in bmo.json must be true or false")?;
                }
                _ => {
                    return Err(format!(
                        "bmo.json has no setting `{name}`; it takes `title_bar` (true or false)"
                    ));
                }
            }
        }
        Ok(settings)
    }
}
