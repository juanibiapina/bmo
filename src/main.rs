mod app;
mod bundle;
mod desktop;
mod dev;
mod home;
mod install;
mod scaffold;

use app::App;
use home::Home;

fn main() {
    let home = Home::from_environment();
    home.share_with_runtime();

    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let Some(command) = own_command(arguments.first()) else {
        return run_host();
    };
    let result = named(&home, command, arguments.get(1)).and_then(|app| match command {
        "new" => scaffold::create(&app),
        "dev" => dev::start(&home, &app),
        "shot" => dev::shot(&home, &app),
        "stop" => dev::stop(&app),
        _ => install::install(&home, &app),
    });
    match result {
        Ok(message) => println!("{message}"),
        Err(message) => {
            eprintln!("bmo: {message}");
            std::process::exit(1);
        }
    }
}

fn own_command(argument: Option<&String>) -> Option<&str> {
    let command = argument?.as_str();
    ["new", "dev", "shot", "stop", "install"]
        .contains(&command)
        .then_some(command)
}

fn named(home: &Home, command: &str, name: Option<&String>) -> Result<App, String> {
    let name = name.ok_or(format!("{command} needs a name: bmo {command} <name>"))?;
    App::named(home, name)
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
