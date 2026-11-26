use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

pub fn open(bundle: &Path, log: &Path) -> Result<(), String> {
    run(Command::new("open")
        .arg("-g")
        .arg("--stdout")
        .arg(log)
        .arg("--stderr")
        .arg(log)
        .arg(bundle))
}

pub fn process(bundle_id: &str) -> Result<Option<String>, String> {
    let output = Command::new("lsappinfo")
        .args(["info", "-only", "pid", "-app", bundle_id])
        .output()
        .map_err(|error| format!("cannot run lsappinfo: {error}"))?;
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.split("pid").nth(1).and_then(|rest| {
        let digits: String = rest
            .chars()
            .skip_while(|character| !character.is_ascii_digit())
            .take_while(char::is_ascii_digit)
            .collect();
        (!digits.is_empty()).then_some(digits)
    }))
}

pub fn wait_until(
    bundle_id: &str,
    limit: Duration,
    wanted: impl Fn(&Option<String>) -> bool,
) -> Result<Option<String>, String> {
    let started = Instant::now();
    loop {
        let found = process(bundle_id)?;
        if wanted(&found) || started.elapsed() > limit {
            return Ok(found);
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

pub fn quit(process: &str) -> Result<(), String> {
    run(Command::new("kill").arg(process))
}

pub fn move_to_workspace(bundle_id: &str, workspace: &str, limit: Duration) -> bool {
    let started = Instant::now();
    while started.elapsed() < limit {
        if let Some(window) = window(bundle_id) {
            return run(Command::new("aerospace").args([
                "move-node-to-workspace",
                "--window-id",
                &window,
                workspace,
            ]))
            .is_ok();
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    false
}

fn window(bundle_id: &str) -> Option<String> {
    let output = Command::new("aerospace")
        .args(["list-windows", "--monitor", "all", "--app-bundle-id", bundle_id])
        .args(["--format", "%{window-id}"])
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .map(str::to_string)
}

fn run(command: &mut Command) -> Result<(), String> {
    let program = command.get_program().to_string_lossy().into_owned();
    let output = command
        .output()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    Err(format!(
        "{program} failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}
