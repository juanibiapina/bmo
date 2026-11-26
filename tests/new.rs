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
fn a_new_app_is_listed_under_its_own_id() {
    let bmo = Bmo::new();
    bmo.run(&["new", "hn"]);

    let listed = bmo.run(&["list"]);

    assert!(listed.stdout.contains("bmo.hn"), "{}", listed.output());
}

#[test]
fn a_name_that_is_not_a_plain_word_is_refused() {
    let bmo = Bmo::new();

    for name in ["../escape", "Has Space", "", "UPPER", "-dash"] {
        let created = bmo.run(&["new", name]);
        assert!(!created.success, "{name} was accepted");
        assert!(
            created.stderr.contains("lowercase letters, digits and dashes"),
            "{}",
            created.stderr
        );
    }
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
