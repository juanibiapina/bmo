mod support;

use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use support::Bmo;

fn plist_value(bundle: &Path, key: &str) -> String {
    let output = Command::new("plutil")
        .args(["-extract", key, "raw"])
        .arg(bundle.join("Contents/Info.plist"))
        .output()
        .expect("plutil runs");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn an_installed_app_has_its_own_id() {
    let bmo = Bmo::new();
    bmo.run(&["new", "hn"]);

    let installed = bmo.run(&["install", "hn"]);

    assert!(installed.success, "{}", installed.output());
    let bundle = bmo.installed("Hn.app");
    assert_eq!(plist_value(&bundle, "CFBundleIdentifier"), "bmo.hn");
    assert_eq!(plist_value(&bundle, "CFBundleName"), "Hn");
}

#[test]
fn an_app_that_does_not_exist_is_not_installed() {
    let bmo = Bmo::new();

    let installed = bmo.run(&["install", "ghost"]);

    assert!(!installed.success);
    assert!(installed.stderr.contains("no app named ghost"), "{}", installed.stderr);
}

#[test]
fn an_installed_app_takes_the_name_its_manifest_gives() {
    let bmo = Bmo::new();
    bmo.run(&["new", "hn"]);
    let manifest = bmo.app("hn").join("gpui-shell.json");
    let renamed = std::fs::read_to_string(&manifest)
        .expect("manifest")
        .replace("\"Hn\"", "\"Hacker News & More\"");
    std::fs::write(&manifest, renamed).expect("manifest");

    let installed = bmo.run(&["install", "hn"]);

    assert!(installed.success, "{}", installed.output());
    let bundle = bmo.installed("Hacker News & More.app");
    assert_eq!(plist_value(&bundle, "CFBundleName"), "Hacker News & More");
    assert_eq!(plist_value(&bundle, "CFBundleIdentifier"), "bmo.hn");
}

#[test]
fn an_installed_app_starts_bmo_on_its_source_folder() {
    let bmo = Bmo::new();
    bmo.run(&["new", "hn"]);
    bmo.run(&["install", "hn"]);

    let bundle = bmo.installed("Hn.app");
    let launcher = bundle
        .join("Contents/MacOS")
        .join(plist_value(&bundle, "CFBundleExecutable"));

    let script = std::fs::read_to_string(&launcher).expect("launcher");
    let mode = std::fs::metadata(&launcher).expect("launcher").permissions().mode();
    assert!(mode & 0o111 != 0, "the launcher is not executable");
    assert!(script.contains(env!("CARGO_BIN_EXE_bmo")), "{script}");
    assert!(script.contains(bmo.home().to_str().unwrap()), "{script}");
    assert!(script.contains("'hn'"), "{script}");
}
