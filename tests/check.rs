mod support;

use support::{Bmo, painted};

#[test]
fn a_broken_app_fails_the_check_with_its_error() {
    let bmo = Bmo::new();
    bmo.app_with("broken", "import { nothing } from \"gpui-kit\";\nexport default 1;\n");

    let checked = bmo.run(&["check", "broken"]);

    assert!(!checked.success);
    assert!(checked.output().contains("nothing"), "{}", checked.output());
}

#[test]
fn the_check_can_print_what_the_app_draws() {
    let bmo = Bmo::new();
    bmo.app_with("red", &painted("#ff0000"));

    let checked = bmo.run(&["check", "red", "--print-spec"]);

    assert!(checked.success, "{}", checked.output());
    assert!(checked.stdout.contains("ff0000"), "{}", checked.stdout);
}

#[test]
fn checking_an_app_that_does_not_exist_says_so() {
    let bmo = Bmo::new();

    let checked = bmo.run(&["check", "ghost"]);

    assert!(!checked.success);
    assert!(checked.stderr.contains("no app named ghost"), "{}", checked.stderr);
}
