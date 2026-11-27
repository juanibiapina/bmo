mod support;

use support::{Bmo, quits_after_writing};

#[test]
#[ignore = "opens a window on the desktop"]
fn a_running_app_works_without_grants_and_can_quit() {
    let bmo = Bmo::new();
    let proof = bmo.home().join("proof.txt");
    bmo.app_with("runner", &quits_after_writing(&proof));

    let ran = bmo.run(&["run", "runner"]);

    assert!(ran.success, "{}", ran.output());
    assert_eq!(std::fs::read_to_string(proof).expect("the proof"), "ran");
}

#[test]
#[ignore = "opens a window on the desktop"]
fn an_installed_app_runs_from_its_launcher() {
    let bmo = Bmo::new();
    let proof = bmo.home().join("proof.txt");
    bmo.app_with("launched", &quits_after_writing(&proof));
    bmo.run(&["install", "launched"]);

    let status = std::process::Command::new(bmo.installed("Launched.app/Contents/MacOS/Launched"))
        .env_remove("BMO_HOME")
        .status()
        .expect("the launcher starts");

    assert!(status.success());
    assert_eq!(std::fs::read_to_string(proof).expect("the proof"), "ran");
}

#[test]
#[ignore = "opens a window on the desktop"]
fn a_running_app_can_be_stopped() {
    let bmo = Bmo::new();
    bmo.run(&["new", "devtest"]);

    let started = bmo.run(&["dev", "devtest"]);
    assert!(started.success, "{}", started.output());

    let stopped = bmo.run(&["stop", "devtest"]);
    assert!(stopped.success, "{}", stopped.output());

    let again = bmo.run(&["stop", "devtest"]);
    assert!(!again.success);
    assert!(again.stderr.contains("devtest is not running"), "{}", again.stderr);
}

fn writes_at_start(proof: &std::path::Path, text: &str) -> String {
    format!(
        r#"
import {{ View, div }} from "gpui-kit";
import * as fs from "fs/promises";

export default class App extends View {{
  init(_props, cx) {{
    cx.spawn(async () => {{
      await fs.writeFile("{}", "{text}");
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

fn becomes(proof: &std::path::Path, text: &str) -> bool {
    let started = std::time::Instant::now();
    while started.elapsed() < std::time::Duration::from_secs(10) {
        if std::fs::read_to_string(proof).is_ok_and(|found| found == text) {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    false
}

#[test]
#[ignore = "opens a window on the desktop"]
fn a_running_app_takes_new_code_on_reload_and_not_before() {
    let bmo = Bmo::new();
    let proof = bmo.home().join("proof.txt");
    let app = bmo.app_with("living", &writes_at_start(&proof, "one"));
    let mut running = std::process::Command::new(env!("CARGO_BIN_EXE_bmo"))
        .args(["run", "living"])
        .env("BMO_HOME", bmo.home())
        .env("BMO_SKILLS", bmo.skills())
        .spawn()
        .expect("the app starts");
    let started = becomes(&proof, "one");

    std::fs::write(app.join("main.js"), writes_at_start(&proof, "two")).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(2));
    let untouched = std::fs::read_to_string(&proof).unwrap_or_default();
    let reloaded = bmo.run(&["reload", "living"]);
    let updated = becomes(&proof, "two");
    running.kill().expect("the app stops");
    running.wait().expect("the app stops");

    assert!(started, "the app never started");
    assert_eq!(untouched, "one", "an edit reached the running app before the reload");
    assert!(reloaded.success, "{}", reloaded.output());
    assert!(reloaded.stdout.contains("reloaded living"), "{}", reloaded.output());
    assert!(updated, "the running app did not take the new code");
}

#[test]
#[ignore = "opens a window on the desktop"]
fn a_reload_that_fails_in_the_running_app_says_why() {
    let bmo = Bmo::new();
    let proof = bmo.home().join("proof.txt");
    let app = bmo.app_with("fragile", &writes_at_start(&proof, "one"));
    let mut running = std::process::Command::new(env!("CARGO_BIN_EXE_bmo"))
        .args(["run", "fragile"])
        .env("BMO_HOME", bmo.home())
        .env("BMO_SKILLS", bmo.skills())
        .spawn()
        .expect("the app starts");
    let started = becomes(&proof, "one");

    // Passes the check, which draws a fresh copy, but throws when the
    // running app builds its view again.
    let throws_on_second_start = writes_at_start(&proof, "two").replace(
        "init(_props, cx) {",
        "init(_props, cx) { if (globalThis.started) throw new Error(\"second start\"); globalThis.started = true;",
    );
    std::fs::write(app.join("main.js"), &throws_on_second_start).unwrap();
    let first = bmo.run(&["reload", "fragile"]);
    let second = bmo.run(&["reload", "fragile"]);
    running.kill().expect("the app stops");
    running.wait().expect("the app stops");

    assert!(started, "the app never started");
    assert!(first.success, "{}", first.output());
    assert!(!second.success, "{}", second.output());
    assert!(second.stderr.contains("second start"), "{}", second.stderr);
}
