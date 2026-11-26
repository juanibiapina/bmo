mod support;

use support::Bmo;

#[test]
#[ignore = "opens a window on the desktop"]
fn a_running_app_can_be_captured_and_stopped() {
    let bmo = Bmo::new();
    bmo.run(&["new", "shottest"]);

    let started = bmo.run(&["dev", "shottest"]);
    assert!(started.success, "{}", started.output());

    let shot = bmo.run(&["shot", "shottest"]);
    assert!(shot.success, "{}", shot.output());
    let image = std::fs::read(shot.stdout.trim()).expect("the screenshot");
    assert!(image.starts_with(b"\x89PNG"), "not a PNG");

    let stopped = bmo.run(&["stop", "shottest"]);
    assert!(stopped.success, "{}", stopped.output());

    let after = bmo.run(&["shot", "shottest"]);
    assert!(!after.success);
    assert!(after.stderr.contains("shottest is not running"), "{}", after.stderr);
}
