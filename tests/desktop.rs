mod support;

use support::Bmo;

#[test]
#[ignore = "opens a window on the desktop"]
fn a_running_app_works_without_grants_and_can_quit() {
    let bmo = Bmo::new();
    let proof = bmo.home().join("proof.txt");
    bmo.app_with(
        "runner",
        &format!(
            r#"
import {{ View, div }} from "gpui-kit";
import * as fs from "fs/promises";
import process from "process";

export default class App extends View {{
  init(_props, cx) {{
    cx.spawn(async () => {{
      await fs.writeFile("{}", "ran");
      process.exit(0);
    }});
  }}

  render() {{
    return div();
  }}
}}
"#,
            proof.display()
        ),
    );

    let ran = bmo.run(&["run", "runner"]);

    assert!(ran.success, "{}", ran.output());
    assert_eq!(std::fs::read_to_string(proof).expect("the proof"), "ran");
}

#[test]
#[ignore = "opens a window on the desktop"]
fn a_running_app_can_be_stopped() {
    let bmo = Bmo::new();
    bmo.run(&["new", "devtest"]);

    let started = bmo.run(&["dev", "devtest"]);
    assert!(started.success, "{}", started.output());

    let stopped = bmo.run(&["stop", "devtest"]);
    assert!(stopped.success, "{}", stopped.output());

    let again = bmo.run(&["stop", "devtest"]);
    assert!(!again.success);
    assert!(again.stderr.contains("devtest is not running"), "{}", again.stderr);
}
