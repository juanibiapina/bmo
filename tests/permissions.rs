mod support;

use support::Bmo;

#[test]
fn an_app_keeps_what_it_stores_inside_bmos_home() {
    let bmo = Bmo::new();

    bmo.shows_green(
        "keeper",
        r#"localStorage.setItem("seen", "yes"); return process.dataDir.endsWith("/data/keeper");"#,
    );

    let stored = std::fs::read_to_string(bmo.home().join("data/keeper/store.json")).expect("the store");
    assert!(stored.contains("seen"), "{stored}");
}

#[test]
fn an_app_reads_any_file_without_asking() {
    Bmo::new().shows_green(
        "reader",
        r#"return (await fs.readFile("/etc/hosts", "utf8")).includes("localhost");"#,
    );
}

#[test]
fn an_app_runs_any_program_without_asking() {
    Bmo::new().shows_green(
        "runner",
        r#"return (await process.run("/bin/echo", ["hi"])).stdout === "hi\n";"#,
    );
}
