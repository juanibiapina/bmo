#![allow(dead_code)]

use std::path::{Path, PathBuf};
pub mod fakes;
pub mod server;

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::Duration;

use wait_timeout::ChildExt;

pub struct Bmo {
    home: tempfile::TempDir,
    applications: tempfile::TempDir,
    environment: Vec<(String, String)>,
    bare: bool,
}

impl Bmo {
    pub fn new() -> Self {
        Self {
            home: tempfile::tempdir().expect("temporary bmo home"),
            applications: tempfile::tempdir().expect("temporary applications folder"),
            environment: Vec::new(),
            bare: false,
        }
    }

    pub fn with(mut self, name: &str, value: &str) -> Self {
        self.environment.push((name.into(), value.into()));
        self
    }

    pub fn launched_like_a_mac_app(mut self) -> Self {
        self.bare = true;
        self
    }

    pub fn shows_green(&self, name: &str, work: &str) {
        self.app_with(name, &green_when(work));
        let shot = self.run(&["shot", name]);
        assert!(shot.success, "{}", shot.output());
        assert_eq!(center_color(&shot.stdout), GREEN, "{}", shot.output());
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
        let mut command = Command::new(env!("CARGO_BIN_EXE_bmo"));
        if self.bare {
            command.env_clear().env("HOME", std::env::var("HOME").unwrap());
        }
        let mut child = command
            .args(arguments)
            .envs(self.environment.iter().cloned())
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

pub fn quits_after_writing(proof: &Path) -> String {
    format!(
        r#"
import {{ View, div }} from "gpui-kit";
import * as fs from "fs/promises";
import process from "process";

export default class App extends View {{
  init(_props, cx) {{
    cx.spawn(async () => {{
      await fs.writeFile("{}", "ran");
      process.exit(0);
    }});
  }}

  render() {{
    return div();
  }}
}}
"#,
        proof.display()
    )
}

pub fn green_when(work: &str) -> String {
    format!(
        r##"
import {{ View, div }} from "gpui-kit";
import * as fs from "fs/promises";
import process from "process";
import * as bmo from "bmo";

export default class App extends View {{
  init(_props, cx) {{
    this.color = "#888888";
    cx.spawn(async (cx) => {{
      try {{
        const passed = await (async () => {{ {work} }})();
        this.color = passed ? "#00ff00" : "#ff0000";
      }} catch (error) {{
        console.log(String(error));
        this.color = "#ff0000";
      }}
      cx.notify();
    }});
  }}

  render() {{
    return div().size_full().bg(this.color);
  }}
}}
"##
    )
}

pub const GREEN: [u8; 3] = [0, 255, 0];

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
