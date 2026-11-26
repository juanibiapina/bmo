use std::convert::Infallible;
use std::path::PathBuf;
use std::rc::Rc;

use gpui_shell::gpui::{AnyWindowHandle, App, Entity, Render, Window, WindowOptions};
use gpui_shell::{AppAssets, FrozenComponentRegistry, ShellRuntime};

use crate::app::App as BmoApp;
use crate::home::Home;
use crate::{log, trust};

pub struct Stage {
    pub runtime: Rc<ShellRuntime>,
    pub directory: PathBuf,
    components: FrozenComponentRegistry,
}

pub fn open(
    home: &Home,
    app: &BmoApp,
    then: impl FnOnce(Stage, &mut App) + 'static,
) -> Result<Infallible, String> {
    app.require()?;
    log::to_stderr(tracing::Level::INFO);
    trust::grant_everything(home, app)?;
    let components = gpui_shell::components::components().map_err(|error| format!("{error:#}"))?;
    let directory = app.directory().clone();
    gpui_platform::application()
        .with_assets(AppAssets::new(directory.clone()))
        .run(move |cx| {
            gpui_shell::init_with_components(cx, &components);
            let runtime = or_exit(
                ShellRuntime::new_with_components(cx, components.clone()).map_err(problem),
            );
            then(
                Stage {
                    runtime,
                    directory,
                    components,
                },
                cx,
            );
        });
    unreachable!("bmo exits from inside the application loop")
}

impl Stage {
    pub fn window<V: Render + 'static>(
        &self,
        options: WindowOptions,
        mut build: impl FnMut(&mut Window, &mut App) -> Entity<V> + 'static,
        cx: &mut App,
    ) -> Result<AnyWindowHandle, String> {
        match self.components.window_opener() {
            Some(open) => {
                let mut build = |window: &mut Window, cx: &mut App| build(window, cx).into();
                open(cx, options, &mut build)
            }
            None => cx.open_window(options, build).map(Into::into),
        }
        .map_err(problem)
    }
}

pub fn problem(error: gpui_shell::anyhow::Error) -> String {
    format!("{error:#}")
}

pub fn or_exit<T>(result: Result<T, String>) -> T {
    result.unwrap_or_else(|message| fail(&message))
}

pub fn fail(message: &str) -> ! {
    eprintln!("bmo: {message}");
    std::process::exit(1);
}
