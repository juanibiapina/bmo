use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

const PLIST: &str = include_str!("../template/Info.plist");

pub struct Bundle {
    pub title: String,
    pub id: String,
    pub home: PathBuf,
    pub arguments: Vec<String>,
}

impl Bundle {
    pub fn write_into(&self, folder: &Path) -> Result<PathBuf, String> {
        let path = folder.join(format!("{}.app", self.title));
        self.write(&path)
            .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
        Ok(path)
    }

    fn write(&self, path: &Path) -> io::Result<()> {
        let contents = path.join("Contents");
        let executables = contents.join("MacOS");
        std::fs::create_dir_all(&executables)?;
        std::fs::write(
            contents.join("Info.plist"),
            PLIST
                .replace("{{id}}", &escaped(&self.id))
                .replace("{{title}}", &escaped(&self.title)),
        )?;
        let launcher = executables.join(&self.title);
        std::fs::write(&launcher, self.launcher()?)?;
        std::fs::set_permissions(&launcher, std::fs::Permissions::from_mode(0o755))
    }

    fn launcher(&self) -> io::Result<String> {
        let bmo = std::env::current_exe()?;
        let mut command = format!(
            "BMO_HOME={} exec {}",
            quoted(&self.home.display().to_string()),
            quoted(&bmo.display().to_string())
        );
        for argument in &self.arguments {
            command.push(' ');
            command.push_str(&quoted(argument));
        }
        Ok(format!("#!/bin/sh\n{command}\n"))
    }
}

fn escaped(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}
