mod support;

use support::{Bmo, center_color, painted};

#[test]
fn a_screenshot_shows_what_the_app_draws() {
    let bmo = Bmo::new();
    bmo.app_with("red", &painted("#ff0000"));

    let shot = bmo.run(&["shot", "red"]);

    assert!(shot.success, "{}", shot.output());
    assert_eq!(center_color(&shot.stdout), [255, 0, 0]);
}
