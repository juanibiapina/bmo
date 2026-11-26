mod support;

use support::{Bmo, GREEN, center_color, green_when};

fn passes(work: &str) {
    let bmo = Bmo::new();
    bmo.app_with("free", &green_when(work));

    let shot = bmo.run(&["shot", "free"]);

    assert!(shot.success, "{}", shot.output());
    assert_eq!(center_color(&shot.stdout), GREEN, "{}", shot.output());
}

#[test]
fn an_app_keeps_what_it_stores_inside_bmos_home() {
    let bmo = Bmo::new();
    bmo.app_with(
        "keeper",
        &green_when(r#"localStorage.setItem("seen", "yes"); return process.dataDir.endsWith("/data/keeper");"#),
    );

    let shot = bmo.run(&["shot", "keeper"]);

    assert_eq!(center_color(&shot.stdout), GREEN, "{}", shot.output());
    let stored = std::fs::read_to_string(bmo.home().join("data/keeper/store.json")).expect("the store");
    assert!(stored.contains("seen"), "{stored}");
}

#[test]
fn an_app_reads_any_file_without_asking() {
    passes(r#"return (await fs.readFile("/etc/hosts", "utf8")).includes("localhost");"#);
}

#[test]
fn an_app_runs_any_program_without_asking() {
    passes(r#"return (await process.run("/bin/echo", ["hi"])).stdout === "hi\n";"#);
}
