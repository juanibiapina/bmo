mod support;

use support::Bmo;

fn screen(bmo: &Bmo, keys: &str) -> Vec<u8> {
    let shot = match keys {
        "" => bmo.run(&["shot", "fresh", "--dark"]),
        keys => bmo.run(&["shot", "fresh", "--dark", "--keys", keys]),
    };
    assert!(shot.success, "{}", shot.output());
    assert!(!shot.stderr.contains("ERROR"), "{}", shot.stderr);
    image::open(shot.stdout.trim()).expect("the screenshot").to_rgb8().into_raw()
}

#[test]
fn a_new_app_shows_its_keys_in_the_footer() {
    let bmo = Bmo::new();
    bmo.run(&["new", "fresh"]);

    let checked = bmo.run(&["check", "fresh", "--print-spec"]);

    assert!(checked.success, "{}", checked.output());
    for hint in ["next", "all keys", "quit"] {
        assert!(checked.stdout.contains(hint), "{hint} is missing: {}", checked.stdout);
    }
}

#[test]
fn a_new_app_moves_with_j_and_k() {
    let bmo = Bmo::new();
    bmo.run(&["new", "fresh"]);

    let start = screen(&bmo, "");

    assert_ne!(screen(&bmo, "j"), start);
    assert_eq!(screen(&bmo, "j k"), start);
    assert_eq!(screen(&bmo, "down up"), start);
}

#[test]
fn a_new_app_opens_and_closes_its_help_sheet() {
    let bmo = Bmo::new();
    bmo.run(&["new", "fresh"]);

    let start = screen(&bmo, "");
    let help = screen(&bmo, "?");

    assert_ne!(help, start);
    assert_eq!(screen(&bmo, "? escape"), start);
    assert_ne!(screen(&bmo, "? / q u i t"), help);
    assert_eq!(screen(&bmo, "? / q u i t escape"), help);
}
