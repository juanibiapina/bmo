mod app;
mod assets;
mod blocks;
mod bundle;
mod check;
mod desktop;
mod dev;
mod environment;
mod exec;
mod herdr;
mod home;
mod http;
mod install;
mod list;
mod live;
mod log;
mod look;
mod reload;
mod run;
mod scaffold;
mod settings;
mod shot;
mod skill;
mod socket;
mod stage;
mod theme;
mod trust;

use std::convert::Infallible;

use app::App;
use home::Home;

const GUIDE: &str = include_str!("guide.md");
const HELP: &str = include_str!("help.txt");

fn main() {
    let home = Home::from_environment();
    home.share_with_runtime();
    skill::refresh();

    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let wants_help = arguments.iter().any(|argument| argument == "--help" || argument == "-h");
    let command = match arguments.first().map(String::as_str) {
        Some(_) if wants_help => "help",
        Some(command) => command,
        None => "help",
    };
    let options = arguments.get(2..).unwrap_or_default();
    let app = || named(&home, command, arguments.get(1));
    let result = match command {
        "help" => Ok(help(&home)),
        "guide" => Ok(GUIDE
            .replace("{{apps}}", &apps(&home))
            .replace("{{blocks}}", blocks::DECLARATIONS.trim())),
        "list" => list::list(&home),
        "new" => app().and_then(|app| scaffold::create(&app)),
        "dev" => app().and_then(|app| dev::start(&home, &app)),
        "stop" => app().and_then(|app| dev::stop(&app)),
        "install" => app().and_then(|app| install::install(&home, &app)),
        "reload" => app().and_then(|app| reload::reload(&home, &app)),
        "check" => app().and_then(|app| in_window(check::check(&home, &app, options))),
        "shot" => app().and_then(|app| in_window(shot::capture(&home, &app, options))),
        "run" => app().and_then(|app| in_window(run::run(&home, &app, options))),
        unknown => Err(format!("bmo does not know `{unknown}`\n\n{}", help(&home))),
    };
    match result {
        Ok(message) => println!("{message}"),
        Err(message) => {
            eprintln!("bmo: {message}");
            std::process::exit(1);
        }
    }
}

fn in_window(ended: Result<Infallible, String>) -> Result<String, String> {
    ended.map(|never| match never {})
}

fn help(home: &Home) -> String {
    HELP.replace("{{apps}}", &apps(home)).trim_end().to_string()
}

fn apps(home: &Home) -> String {
    home.root().join("apps").display().to_string()
}

fn named(home: &Home, command: &str, name: Option<&String>) -> Result<App, String> {
    let name = name.ok_or(format!("{command} needs a name: bmo {command} <name>"))?;
    App::named(home, name)
}
