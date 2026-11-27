mod support;

use support::{Bmo, center_color};

fn draws(path: &str) -> String {
    format!(
        r#"
import {{ View, div, image }} from "gpui-kit";

export default class App extends View {{
  render() {{
    return div().size_full().child(image("{path}").size_full());
  }}
}}
"#
    )
}

fn blue_picture(file: &std::path::Path) {
    std::fs::create_dir_all(file.parent().unwrap()).expect("the folder");
    image::RgbImage::from_pixel(8, 8, image::Rgb([0, 0, 255]))
        .save(file)
        .expect("the picture");
}

#[test]
fn an_app_draws_a_picture_from_its_own_folder() {
    let bmo = Bmo::new();
    bmo.app_with("inside", &draws("pictures/blue.png"));
    blue_picture(&bmo.app("inside").join("pictures/blue.png"));

    let shot = bmo.run(&["shot", "inside"]);

    assert_eq!(center_color(&shot.stdout), [0, 0, 255], "{}", shot.output());
}

#[test]
fn an_app_draws_a_picture_from_anywhere_on_the_disk() {
    let bmo = Bmo::new();
    let picture = bmo.home().join("data/outside/blue.png");
    blue_picture(&picture);
    bmo.app_with("outside", &draws(picture.to_str().unwrap()));

    let shot = bmo.run(&["shot", "outside"]);

    assert_eq!(center_color(&shot.stdout), [0, 0, 255], "{}", shot.output());
}

#[test]
fn a_screenshot_shows_a_picture_the_app_finds_after_it_starts() {
    let bmo = Bmo::new();
    let picture = bmo.home().join("data/late/blue.png");
    blue_picture(&picture);
    bmo.app_with(
        "late",
        &format!(
            r#"
import {{ View, div, image }} from "gpui-kit";
import {{ exec }} from "bmo";

export default class App extends View {{
  init(_props, cx) {{
    this.found = false;
    cx.spawn(async (cx) => {{
      await exec("sleep", ["0.2"]);
      this.found = true;
      cx.notify();
    }});
  }}

  render() {{
    const box = div().size_full();
    return this.found ? box.child(image("{}").size_full()) : box;
  }}
}}
"#,
            picture.display()
        ),
    );

    let shot = bmo.run(&["shot", "late"]);

    assert_eq!(center_color(&shot.stdout), [0, 0, 255], "{}", shot.output());
}
