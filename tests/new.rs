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
