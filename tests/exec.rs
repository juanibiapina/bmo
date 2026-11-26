mod support;

use support::Bmo;

#[test]
fn exec_gives_the_exit_code_and_both_outputs() {
    Bmo::new().shows_green(
        "outputs",
        r#"
        const result = await bmo.exec("/bin/sh", ["-c", "echo out; echo err >&2; exit 3"]);
        return result.code === 3 && result.stdout === "out\n" && result.stderr === "err\n";
        "#,
    );
}
