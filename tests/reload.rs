mod support;

use support::{Bmo, painted};

#[test]
fn a_reload_hands_the_checked_source_to_the_live_copy() {
    let bmo = Bmo::new();
    let app = bmo.app_with("red", &painted("#ff0000"));
    let live = bmo.home().join("live/red");
    assert!(bmo.run(&["reload", "red"]).success);
    assert!(std::fs::read_to_string(live.join("main.js")).unwrap().contains("#ff0000"));

    std::fs::write(app.join("main.js"), painted("#00ff00")).unwrap();
    std::fs::create_dir_all(app.join("parts")).unwrap();
    std::fs::write(app.join("parts/extra.js"), "export const extra = 1;\n").unwrap();
    std::fs::remove_file(app.join("keys.js")).unwrap();
    let reloaded = bmo.run(&["reload", "red"]);

    assert!(reloaded.success, "{}", reloaded.output());
    assert!(std::fs::read_to_string(live.join("main.js")).unwrap().contains("#00ff00"));
    assert!(live.join("parts/extra.js").is_file());
    assert!(!live.join("keys.js").exists(), "a file removed from the source stayed in the live copy");
}

#[test]
fn a_reload_of_source_that_fails_the_check_changes_nothing() {
    let bmo = Bmo::new();
    let app = bmo.app_with("red", &painted("#ff0000"));
    bmo.run(&["reload", "red"]);

    std::fs::write(app.join("main.js"), "this is not javascript (").unwrap();
    let reloaded = bmo.run(&["reload", "red"]);

    assert!(!reloaded.success, "{}", reloaded.output());
    assert!(reloaded.stderr.contains("the running app was left as it is"), "{}", reloaded.stderr);
    let live = std::fs::read_to_string(bmo.home().join("live/red/main.js")).unwrap();
    assert!(live.contains("#ff0000"), "{live}");
}

#[test]
fn a_reload_says_when_the_app_is_not_running() {
    let bmo = Bmo::new();
    bmo.app_with("red", &painted("#ff0000"));

    let reloaded = bmo.run(&["reload", "red"]);

    assert!(reloaded.success, "{}", reloaded.output());
    assert!(reloaded.stdout.contains("red is not running"), "{}", reloaded.stdout);
}
