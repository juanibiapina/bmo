mod home;
mod scaffold;

use home::Home;

fn main() {
    let home = Home::from_environment();
    home.share_with_runtime();

    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.first().map(String::as_str) {
        Some("new") => finish(scaffold::create(&home, arguments.get(1).map(String::as_str))),
        _ => run_host(),
    }
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
