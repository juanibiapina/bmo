use std::cell::RefCell;
use std::convert::Infallible;
use std::rc::Rc;

use gpui_shell::ShellRoot;
use gpui_shell::gpui::{
    self, App, AppContext, Bounds, Context, IntoElement, KeyBinding, Render, TitlebarOptions,
    Window, WindowBounds, WindowOptions, point, px, size,
};
use gpui_shell::plugin::PluginManifest;

use crate::app::App as BmoApp;
use crate::home::Home;
use crate::stage::{self, or_exit, problem};

gpui::actions!(bmo, [Quit]);

pub fn run(home: &Home, app: &BmoApp, options: &[String]) -> Result<Infallible, String> {
    let watching = match options {
        [] => false,
        [option] if option == "--watch" => true,
        _ => return Err("run takes only --watch: bmo run <name> [--watch]".into()),
    };
    let title = app.title();
    let id = app.id();
    stage::open(home, app, watching, move |stage, cx| {
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
        gpui_shell::on_exit_request(|request, _, _| std::process::exit(request.code()));

        let loaded = Rc::new(RefCell::new(None));
        let sink = loaded.clone();
        let runtime = stage.runtime.clone();
        let directory = stage.directory.clone();
        let build = move |window: &mut Window, cx: &mut App| {
            match runtime.try_load(&directory, window, cx) {
                Ok(root) => {
                    *sink.borrow_mut() = Some(root.clone());
                    root
                }
                Err(error) => {
                    let message = problem(error);
                    eprintln!("{message}");
                    let failure = cx.new(|_| LoadFailure(message));
                    cx.new(|cx| ShellRoot::new(failure.into(), window, cx))
                }
            }
        };
        let options = shown(&stage.directory, title, id, stage.settings.title_bar, cx);
        let window = or_exit(stage.window(options, build, cx));

        cx.spawn(async move |cx| {
            cx.update(|cx| {
                cx.activate(true);
                let _ = window.update(cx, |_, window, _| window.activate_window());
            });
        })
        .detach();

        let root = loaded.borrow_mut().take();
        if let (true, Some(root)) = (watching, root) {
            let watch = window
                .update(cx, |_, window, cx| stage.runtime.watch(&root, window, cx))
                .and_then(|watch| watch);
            match watch {
                Ok(watch) => watch.forget(),
                Err(error) => eprintln!("bmo: cannot watch for changes: {error:#}"),
            }
        }
    })
}

fn shown(
    directory: &std::path::Path,
    title: String,
    id: String,
    title_bar: bool,
    cx: &App,
) -> WindowOptions {
    let size = PluginManifest::read(directory)
        .ok()
        .and_then(|manifest| manifest.window())
        .map(|window| size(px(window.width()), px(window.height())))
        .unwrap_or(size(px(880.), px(720.)));
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(None, size, cx))),
        // Without a title bar the window keeps its title and stays an
        // ordinary one, so a window manager still tiles it: the bar turns
        // transparent and its three buttons move out of sight.
        titlebar: Some(TitlebarOptions {
            title: Some(title.into()),
            appears_transparent: !title_bar,
            traffic_light_position: (!title_bar).then(|| point(px(-100.), px(-100.))),
        }),
        app_id: Some(id),
        ..Default::default()
    }
}

struct LoadFailure(String);

impl Render for LoadFailure {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        gpui_shell::failure_surface(
            "This app could not be loaded",
            &self.0,
            "Run bmo check on it to see what is wrong.",
            window,
            cx,
        )
    }
}
