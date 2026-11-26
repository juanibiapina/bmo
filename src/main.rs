mod app;
mod home;
mod install;
mod scaffold;

use app::App;
use home::Home;

fn main() {
    let home = Home::from_environment();
    home.share_with_runtime();

    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.first().map(String::as_str) {
        Some("new") => finish(named(&home, &arguments).and_then(|app| scaffold::create(&app))),
        Some("install") => {
            finish(named(&home, &arguments).and_then(|app| install::install(&home, &app)))
        }
        _ => run_host(),
    }
}

fn named(home: &Home, arguments: &[String]) -> Result<App, String> {
    let command = &arguments[0];
    let name = arguments
        .get(1)
        .ok_or(format!("{command} needs a name: bmo {command} <name>"))?;
    App::named(home, name)
}

fn finish(result: Result<String, String>) {
    match result {
        Ok(message) => println!("{message}"),
        Err(message) => {
            eprintln!("bmo: {message}");
            std::process::exit(1);
        }
    }
}

fn run_host() {
    let components = match gpui_shell::components::components() {
        Ok(components) => components,
        Err(error) => {
            eprintln!("bmo: {error:#}");
            std::process::exit(1);
        }
    };
    gpui_shell::host::main_with_brand_and_components(
        gpui_shell::host::HostBrand::new("bmo", env!("CARGO_PKG_VERSION")),
        components,
    );
}
