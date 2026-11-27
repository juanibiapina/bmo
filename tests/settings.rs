mod support;

use support::{Bmo, painted};

fn check_with(settings: &str) -> support::Run {
    let bmo = Bmo::new();
    let app = bmo.app_with("red", &painted("#ff0000"));
    std::fs::write(app.join("bmo.json"), settings).expect("the settings");
    bmo.run(&["check", "red"])
}

#[test]
fn an_app_can_ask_for_a_window_without_a_title_bar() {
    let checked = check_with(r#"{ "title_bar": false }"#);

    assert!(checked.success, "{}", checked.output());
}

#[test]
fn a_setting_bmo_does_not_know_fails_the_check_and_names_the_ones_it_does() {
    let checked = check_with(r#"{ "titlebar": false }"#);

    assert!(!checked.success, "{}", checked.output());
    assert!(checked.stderr.contains("bmo.json has no setting `titlebar`"), "{}", checked.stderr);
    assert!(checked.stderr.contains("title_bar"), "{}", checked.stderr);
}

#[test]
fn a_setting_of_the_wrong_kind_fails_the_check() {
    let checked = check_with(r#"{ "title_bar": "no" }"#);

    assert!(!checked.success, "{}", checked.output());
    assert!(checked.stderr.contains("`title_bar` in bmo.json must be true or false"), "{}", checked.stderr);
}
