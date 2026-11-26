mod support;

use support::Bmo;

#[test]
fn a_new_app_passes_the_check() {
    let bmo = Bmo::new();

    let created = bmo.run(&["new", "hn"]);
    assert!(created.success, "{}", created.output());

    let checked = bmo.run(&["check", "hn"]);
    assert!(checked.success, "{}", checked.output());
    assert!(checked.output().contains("check passed"), "{}", checked.output());
}

#[test]
fn an_existing_app_is_not_overwritten() {
    let bmo = Bmo::new();
    bmo.run(&["new", "hn"]);
    std::fs::write(bmo.app("hn").join("main.js"), "my work").expect("edit the app");

    let again = bmo.run(&["new", "hn"]);

    assert!(!again.success);
    assert!(again.stderr.contains("hn already exists"), "{}", again.stderr);
    let main = std::fs::read_to_string(bmo.app("hn").join("main.js")).expect("read the app");
    assert_eq!(main, "my work");
}
