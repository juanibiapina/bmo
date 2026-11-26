use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

pub struct Window {
    pub id: String,
    pub process: String,
}

pub fn window(bundle_id: &str) -> Result<Option<Window>, String> {
    let output = Command::new("aerospace")
        .args(["list-windows", "--monitor", "all", "--app-bundle-id", bundle_id])
        .args(["--format", "%{window-id} %{app-pid}"])
        .output()
        .map_err(|error| format!("cannot run aerospace, which bmo uses to find windows: {error}"))?;
    let text = String::from_utf8_lossy(&output.stdout);
    let mut fields = text.split_whitespace();
    Ok(match (fields.next(), fields.next()) {
        (Some(id), Some(process)) => Some(Window {
            id: id.to_string(),
            process: process.to_string(),
        }),
        _ => None,
    })
}

pub fn wait_until(
    bundle_id: &str,
    limit: Duration,
    wanted: impl Fn(&Option<Window>) -> bool,
) -> Result<Option<Window>, String> {
    let started = Instant::now();
    loop {
        let found = window(bundle_id)?;
        if wanted(&found) || started.elapsed() > limit {
            return Ok(found);
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

pub fn open(bundle: &Path, log: &Path) -> Result<(), String> {
    run(Command::new("open")
        .arg("-g")
        .arg("--stdout")
        .arg(log)
        .arg("--stderr")
        .arg(log)
        .arg(bundle))
}

pub fn move_to_workspace(window: &Window, workspace: &str) -> Result<(), String> {
    run(Command::new("aerospace").args([
        "move-node-to-workspace",
        "--window-id",
        &window.id,
        workspace,
    ]))
}

pub fn quit(window: &Window) -> Result<(), String> {
    run(Command::new("kill").arg(&window.process))
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
