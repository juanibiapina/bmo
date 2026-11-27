mod support;

use support::Bmo;

const COMMANDS: [&str; 9] = ["guide", "new", "check", "shot", "dev", "stop", "run", "install", "list"];

#[test]
fn bmo_alone_shows_its_commands() {
    let shown = Bmo::new().run(&[]);

    assert!(shown.success, "{}", shown.output());
    for command in COMMANDS {
        assert!(shown.stdout.contains(&format!("bmo {command}")), "{command} is missing: {}", shown.stdout);
    }
}

#[test]
fn any_command_answers_a_call_for_help() {
    let bmo = Bmo::new();

    for arguments in [&["shot", "--help"][..], &["help", "shot"], &["new", "-h"]] {
        let shown = bmo.run(arguments);
        assert!(shown.success, "{}", shown.output());
        assert!(shown.stdout.contains("bmo shot <name>"), "{}", shown.stdout);
        assert!(shown.stdout.contains("--as"), "{}", shown.stdout);
    }
}

#[test]
fn an_unknown_command_is_refused_with_the_commands() {
    let refused = Bmo::new().run(&["bundle"]);

    assert!(!refused.success);
    assert!(refused.stderr.contains("bmo does not know `bundle`"), "{}", refused.stderr);
    assert!(refused.stderr.contains("bmo guide"), "{}", refused.stderr);
}

#[test]
fn the_list_says_which_apps_are_installed() {
    let bmo = Bmo::new();
    bmo.run(&["new", "hn"]);
    bmo.run(&["new", "mail"]);
    bmo.run(&["install", "mail"]);

    let listed = bmo.run(&["list"]);

    assert!(listed.success, "{}", listed.output());
    let line = |name: &str| listed.stdout.lines().find(|line| line.starts_with(name)).unwrap_or_default().to_string();
    assert!(line("hn").contains("bmo.hn") && !line("hn").contains("installed"), "{}", listed.stdout);
    assert!(line("mail").contains("bmo.mail") && line("mail").contains("installed"), "{}", listed.stdout);
}

#[test]
fn the_list_is_empty_before_any_app_exists() {
    let listed = Bmo::new().run(&["list"]);

    assert!(listed.success, "{}", listed.output());
    assert!(listed.stdout.contains("no apps yet"), "{}", listed.stdout);
}

#[test]
fn the_guide_teaches_the_loop_the_blocks_and_the_conventions() {
    let bmo = Bmo::new();
    let guide = bmo.run(&["guide"]);

    assert!(guide.success, "{}", guide.output());
    for part in [
        "bmo check",
        "bmo shot",
        "export function exec(",
        "export function fetch(",
        "export function connect(",
        "process.dataDir",
        "export function sessions(",
        "export function theme(",
        "j / k",
        bmo.home().join("apps").to_str().unwrap(),
    ] {
        assert!(guide.stdout.contains(part), "{part} is missing");
    }
}
