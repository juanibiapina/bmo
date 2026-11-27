mod support;

use support::Bmo;
use support::fakes::Fakes;

const TERMINALS: &str = r#"{"terminals":[
  {"address":"t1","agent":"pi","cwd":"/work/api","state":"live","status":"working","where":"api / pi"},
  {"address":"t2","agent":"zsh","cwd":"/work","state":"live","status":"idle","where":"work / 1"}
]}"#;

fn bmo_with(fakes: &Fakes) -> Bmo {
    Bmo::new().with("PATH", &fakes.path())
}

#[test]
fn sessions_lists_herdrs_terminals() {
    let fakes = Fakes::new().program("hc", TERMINALS);

    bmo_with(&fakes).shows_green(
        "lister",
        r#"
        const all = await bmo.sessions();
        return all.length === 2 && all[0].address === "t1" && all[0].agent === "pi"
          && all[0].cwd === "/work/api" && all[0].status === "working" && all[1].where === "work / 1";
        "#,
    );

    assert_eq!(fakes.called_with("hc"), ["ls", "--json"]);
}

#[test]
fn the_focused_session_is_the_one_herdr_shows() {
    let fakes = Fakes::new().program("herdr-app", r#"{"agent":"t7","layout":"main"}"#);

    bmo_with(&fakes).shows_green("focused", r#"return (await bmo.focusedSession()) === "t7";"#);
}

#[test]
fn there_is_no_focused_session_when_herdr_shows_none() {
    let fakes = Fakes::new().program("herdr-app", r#"{"agent":null}"#);

    bmo_with(&fakes).shows_green("unfocused", r#"return (await bmo.focusedSession()) === null;"#);
}

#[test]
fn send_gives_a_session_the_text_as_written() {
    let fakes = Fakes::new().program("hc", "");

    bmo_with(&fakes).shows_green(
        "sender",
        r#"await bmo.send("t1", "look at this: it's --not a flag"); return true;"#,
    );

    assert_eq!(fakes.called_with("hc"), ["send", "t1", "look at this: it's --not a flag"]);
}

#[test]
fn show_brings_a_session_up_in_herdr() {
    let fakes = Fakes::new().program("herdr-app", "").program("open", "");

    bmo_with(&fakes).shows_green("shower", r#"await bmo.show("t1"); return true;"#);

    assert_eq!(fakes.called_with("herdr-app"), ["do", "agent.focus", "--terminal", "t1"]);
    assert_eq!(fakes.called_with("open"), ["-b", "dev.herdr.app"]);
}

#[test]
fn start_opens_a_new_session_with_a_task() {
    let fakes = Fakes::new().program("herdr-app", "");

    bmo_with(&fakes).shows_green(
        "starter",
        r#"await bmo.start({ folder: "/work/api", task: "fix the build", name: "build" }); return true;"#,
    );

    let called = fakes.called_with("herdr-app");
    assert_eq!(&called[..2], ["do", "space.new"]);
    let sent: serde_json::Value = serde_json::from_str(&called[2]).expect("one JSON object");
    assert_eq!(sent["folder"], "/work/api");
    assert_eq!(sent["task"], "fix the build");
    assert_eq!(sent["name"], "build");
    assert_eq!(sent["argv"], serde_json::json!(["pi"]));
    assert_eq!(sent["machine"], "");
}

#[test]
fn a_session_call_says_when_herdr_is_missing() {
    Bmo::new().with("PATH", "/usr/bin:/bin").shows_green(
        "alone",
        r#"
        try {
          await bmo.sessions();
          return false;
        } catch (error) {
          return String(error).includes("Herdr is not installed");
        }
        "#,
    );
}
