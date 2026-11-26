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

#[test]
fn exec_feeds_input_and_runs_in_a_folder() {
    Bmo::new().shows_green(
        "piped",
        r#"
        const result = await bmo.exec("/bin/sh", ["-c", "cat; pwd"], { cwd: "/usr", input: "fed " });
        return result.stdout === "fed /usr\n";
        "#,
    );
}

#[test]
fn exec_rejects_a_program_that_runs_too_long() {
    Bmo::new().shows_green(
        "slow",
        r#"
        try {
          await bmo.exec("/bin/sleep", ["5"], { timeout: 100 });
          return false;
        } catch (error) {
          return String(error).includes("did not finish in 100 ms");
        }
        "#,
    );
}

#[test]
fn exec_rejects_a_program_that_cannot_start() {
    Bmo::new().shows_green(
        "missing",
        r#"
        try {
          await bmo.exec("no-such-program-anywhere");
          return false;
        } catch (error) {
          return String(error).includes("cannot run no-such-program-anywhere");
        }
        "#,
    );
}

#[test]
fn exec_uses_the_login_environment_when_started_as_a_mac_app() {
    let shell = tempfile::tempdir().expect("a folder for the shell");
    let fake = shell.path().join("shell");
    std::fs::write(&fake, "#!/bin/sh\necho noise from a shell profile\nexport PROBE=from-login\neval \"$4\"\n").expect("the shell");
    std::fs::set_permissions(&fake, std::os::unix::fs::PermissionsExt::from_mode(0o755)).expect("the shell");

    Bmo::new()
        .launched_like_a_mac_app()
        .with("SHELL", fake.to_str().unwrap())
        .shows_green(
            "login",
            r#"return (await bmo.exec("/bin/sh", ["-c", "printf %s \"$PROBE\""])).stdout === "from-login";"#,
        );
}
