use std::process::{Command, Stdio};
use std::sync::OnceLock;

const MARK: &str = "bmo-environment-starts-here";

pub fn of_the_user() -> &'static [(String, String)] {
    static FOUND: OnceLock<Vec<(String, String)>> = OnceLock::new();
    FOUND.get_or_init(|| {
        if std::env::var_os("TERM").is_some() {
            return Vec::new();
        }
        from_login_shell().unwrap_or_default()
    })
}

fn from_login_shell() -> Option<Vec<(String, String)>> {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".into());
    let output = Command::new(shell)
        .args(["-l", "-i", "-c", &format!("printf '%s\\0' {MARK}; env -0")])
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let (_, variables) = text.split_once(&format!("{MARK}\0"))?;
    Some(
        variables
            .split('\0')
            .filter_map(|variable| variable.split_once('='))
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect(),
    )
}
