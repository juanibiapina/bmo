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
