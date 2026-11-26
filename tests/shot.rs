mod support;

use support::{Bmo, center_color, painted};

const TURNS_GREEN_ON_G: &str = r##"
import { View, div } from "gpui-kit";

export default class App extends View {
  init(_props, cx) {
    this.handle = cx.focus_handle();
    this.handle.focus();
    this.color = "#ff0000";
  }

  render() {
    return div()
      .size_full()
      .bg(this.color)
      .track_focus(this.handle)
      .on_key_down((event, cx) => {
        if (event.key === "g") this.color = "#00ff00";
        if (event.key === "b" && this.color === "#00ff00") this.color = "#0000ff";
        cx.notify();
      });
  }
}
"##;

#[test]
fn a_screenshot_comes_with_what_the_app_logged() {
    let bmo = Bmo::new();
    bmo.app_with(
        "talker",
        &support::green_when(r#"console.log("hello from the app"); throw new Error("it broke");"#),
    );

    let shot = bmo.run(&["shot", "talker"]);

    assert!(shot.success, "{}", shot.output());
    assert!(shot.stderr.contains("hello from the app"), "{}", shot.stderr);
    assert!(shot.stderr.contains("it broke"), "{}", shot.stderr);
}

#[test]
fn a_screenshot_can_follow_key_presses() {
    let bmo = Bmo::new();
    bmo.app_with("keys", TURNS_GREEN_ON_G);

    let color_after = |keys: &[&str]| {
        let shot = bmo.run(&[&["shot", "keys"], keys].concat());
        assert!(shot.success, "{}", shot.output());
        center_color(&shot.stdout)
    };

    assert_eq!(color_after(&[]), [255, 0, 0]);
    assert_eq!(color_after(&["--keys", "g"]), [0, 255, 0]);
    assert_eq!(color_after(&["--keys", "g b"]), [0, 0, 255]);
}

#[test]
fn a_screenshot_shows_what_the_app_draws() {
    let bmo = Bmo::new();
    bmo.app_with("red", &painted("#ff0000"));

    let shot = bmo.run(&["shot", "red"]);

    assert!(shot.success, "{}", shot.output());
    assert_eq!(center_color(&shot.stdout), [255, 0, 0]);
}
