#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::io::Read;
use std::process::{Command, Stdio};
use std::time::Duration;

use wait_timeout::ChildExt;

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

    pub fn app_with(&self, name: &str, main: &str) -> PathBuf {
        let created = self.run(&["new", name]);
        assert!(created.success, "{}", created.output());
        std::fs::write(self.app(name).join("main.js"), main).expect("write the app");
        self.app(name)
    }

    pub fn installed(&self, bundle: &str) -> PathBuf {
        self.applications.path().join(bundle)
    }

    pub fn run(&self, arguments: &[&str]) -> Run {
        let mut child = Command::new(env!("CARGO_BIN_EXE_bmo"))
            .args(arguments)
            .env("BMO_HOME", self.home())
            .env("BMO_APPLICATIONS", self.applications.path())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("bmo starts");
        let status = match child.wait_timeout(Duration::from_secs(20)).expect("bmo waits") {
            Some(status) => status,
            None => {
                child.kill().expect("bmo stops");
                child.wait().expect("bmo stops");
                panic!("bmo {} did not finish in 20 seconds", arguments.join(" "));
            }
        };
        let mut stdout = String::new();
        let mut stderr = String::new();
        child.stdout.take().unwrap().read_to_string(&mut stdout).unwrap();
        child.stderr.take().unwrap().read_to_string(&mut stderr).unwrap();
        Run {
            success: status.success(),
            stdout,
            stderr,
        }
    }
}

pub fn painted(color: &str) -> String {
    format!(
        r#"
import {{ View, div }} from "gpui-kit";

export default class App extends View {{
  render() {{
    return div().size_full().bg("{color}");
  }}
}}
"#
    )
}

pub fn center_color(image: &str) -> [u8; 3] {
    let image = image::open(image.trim()).expect("the screenshot opens").to_rgb8();
    image.get_pixel(image.width() / 2, image.height() / 2).0
}

pub struct Run {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl Run {
    pub fn output(&self) -> String {
        format!("{}{}", self.stdout, self.stderr)
    }
}
