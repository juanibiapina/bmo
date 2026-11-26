mod support;

use support::Bmo;

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
