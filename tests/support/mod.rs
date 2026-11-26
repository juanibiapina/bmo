use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub struct Bmo {
    home: tempfile::TempDir,
    applications: tempfile::TempDir,
}

impl Bmo {
    pub fn new() -> Self {
        Self {
            home: tempfile::tempdir().expect("temporary bmo home"),
            applications: tempfile::tempdir().expect("temporary applications folder"),
        }
    }

    pub fn home(&self) -> &Path {
        self.home.path()
    }

    pub fn app(&self, name: &str) -> PathBuf {
        self.home().join("apps").join(name)
    }

    pub fn installed(&self, bundle: &str) -> PathBuf {
        self.applications.path().join(bundle)
    }

    pub fn run(&self, arguments: &[&str]) -> Run {
        let output = Command::new(env!("CARGO_BIN_EXE_bmo"))
            .args(arguments)
            .env("BMO_HOME", self.home())
            .env("BMO_APPLICATIONS", self.applications.path())
            .output()
            .expect("bmo runs");
        Run::from(output)
    }
}

pub struct Run {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl From<Output> for Run {
    fn from(output: Output) -> Self {
        Self {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

impl Run {
    pub fn output(&self) -> String {
        format!("{}{}", self.stdout, self.stderr)
    }
}
