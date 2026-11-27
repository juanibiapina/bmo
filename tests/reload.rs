mod support;

use support::{Bmo, painted};

#[test]
fn a_reload_of_source_that_fails_the_check_is_refused() {
    let bmo = Bmo::new();
    let app = bmo.app_with("red", &painted("#ff0000"));
    std::fs::write(app.join("main.js"), "this is not javascript (").unwrap();

    let reloaded = bmo.run(&["reload", "red"]);

    assert!(!reloaded.success, "{}", reloaded.output());
    assert!(reloaded.stderr.contains("the running app was left as it is"), "{}", reloaded.stderr);
}

#[test]
fn a_reload_says_when_the_app_is_not_running() {
    let bmo = Bmo::new();
    bmo.app_with("red", &painted("#ff0000"));

    let reloaded = bmo.run(&["reload", "red"]);

    assert!(reloaded.success, "{}", reloaded.output());
    assert!(reloaded.stdout.contains("red is not running"), "{}", reloaded.stdout);
}

#[test]
fn an_app_runs_from_its_source_with_no_copy() {
    let bmo = Bmo::new();
    bmo.app_with("red", &painted("#ff0000"));

    bmo.run(&["reload", "red"]);
    bmo.run(&["shot", "red"]);

    assert!(!bmo.home().join("live").exists(), "bmo made a second copy of the code");
}
