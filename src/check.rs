use std::convert::Infallible;

use gpui_shell::gpui::{App, AppContext, Empty, Window};

use crate::app::App as BmoApp;
use crate::home::Home;
use crate::shot::hidden;
use crate::stage::{self, fail, or_exit, problem};

pub fn check(home: &Home, app: &BmoApp, options: &[String]) -> Result<Infallible, String> {
    let print_spec = match options {
        [] => false,
        [option] if option == "--print-spec" => true,
        _ => return Err("check takes only --print-spec: bmo check <name> [--print-spec]".into()),
    };
    stage::open(home, app, move |stage, cx| {
        let window = or_exit(stage.window(
            hidden(cx),
            |_: &mut Window, cx: &mut App| cx.new(|_| Empty),
            cx,
        ));
        let checked = window
            .update(cx, |_, window, cx| stage.runtime.check(&stage.directory, window, cx))
            .and_then(|checked| checked)
            .map_err(problem);
        match checked {
            Ok(spec) => {
                if print_spec {
                    println!("{spec}");
                }
                println!("check passed: {}", stage.directory.display());
                std::process::exit(0);
            }
            Err(message) => {
                eprintln!("{message}");
                fail(&format!("check failed: {}", stage.directory.display()));
            }
        }
    })
}
