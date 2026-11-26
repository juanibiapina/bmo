use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub struct Fakes {
    folder: tempfile::TempDir,
}

impl Fakes {
    pub fn new() -> Self {
        Self {
            folder: tempfile::tempdir().expect("a folder for fake programs"),
        }
    }

    pub fn program(self, name: &str, prints: &str) -> Self {
        let file = self.folder.path().join(name);
        let calls = self.calls(name);
        let script = format!(
            "#!/bin/sh\nfor argument in \"$@\"; do printf '%s\\n' \"$argument\" >> '{}'; done\ncat <<'PRINTED'\n{prints}\nPRINTED\n",
            calls.display()
        );
        std::fs::write(&file, script).expect("the fake program");
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o755)).expect("the fake program");
        self
    }

    pub fn path(&self) -> String {
        format!("{}:/usr/bin:/bin", self.folder.path().display())
    }

    pub fn called_with(&self, name: &str) -> Vec<String> {
        std::fs::read_to_string(self.calls(name))
            .unwrap_or_default()
            .lines()
            .map(str::to_string)
            .collect()
    }

    fn calls(&self, name: &str) -> PathBuf {
        Path::new(self.folder.path()).join(format!("{name}.calls"))
    }
}
