mod support;

use support::{Bmo, center_color};

const GRAPHITE: [u8; 3] = [0x17, 0x19, 0x1d];
const PAPER: [u8; 3] = [0xff, 0xff, 0xff];

fn painted_with(color: &str) -> String {
    format!(
        r#"
import {{ View, div }} from "gpui-kit";
import {{ theme }} from "bmo";

export default class App extends View {{
  render(cx) {{
    return div().size_full().bg({color});
  }}
}}
"#
    )
}

fn background(color: &str, appearance: &str) -> [u8; 3] {
    let bmo = Bmo::new();
    bmo.app_with("look", &painted_with(color));
    let shot = bmo.run(&["shot", "look", appearance]);
    assert!(shot.success, "{}", shot.output());
    center_color(&shot.stdout)
}

#[test]
fn the_dark_look_is_herdrs_graphite() {
    assert_eq!(background("theme().background", "--dark"), GRAPHITE);
}

#[test]
fn the_light_look_is_herdrs_paper() {
    assert_eq!(background("theme().background", "--light"), PAPER);
}

#[test]
fn the_runtimes_own_colors_follow_the_look() {
    assert_eq!(background("cx.theme().colors.background", "--dark"), GRAPHITE);
    assert_eq!(background("cx.theme().colors.background", "--light"), PAPER);
}
