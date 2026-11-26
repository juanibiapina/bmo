mod support;

use support::Bmo;

#[test]
fn an_app_reads_the_colors_by_name() {
    let bmo = Bmo::new();
    bmo.app_with(
        "colors",
        r#"
import { View, div } from "gpui-kit";
import { theme } from "bmo";

export default class App extends View {
  render() {
    const t = theme();
    return div().bg(t.raised).text_color(t.foreground).child(Object.keys(t).sort().join(" "));
  }
}
"#,
    );

    let checked = bmo.run(&["check", "colors", "--print-spec"]);

    assert!(checked.success, "{}", checked.output());
    for name in ["background", "raised", "hover", "selected", "muted", "dim", "accent", "working", "ansi15"] {
        assert!(checked.stdout.contains(name), "{name} is missing: {}", checked.stdout);
    }
}
