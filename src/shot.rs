use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::time::Duration;

use gpui_shell::gpui::{
    AnyWindowHandle, App, AsyncApp, Bounds, Keystroke, Window, WindowBounds, WindowOptions, px,
    size,
};
use gpui_shell::{AppAssets, ShellRuntime};

use crate::app::App as BmoApp;
use crate::home::Home;
use crate::log;
use crate::trust;

const SETTLE: Duration = Duration::from_millis(500);
const BETWEEN_KEYS: Duration = Duration::from_millis(50);

pub fn capture(home: &Home, app: &BmoApp, options: &[String]) -> Result<Infallible, String> {
    app.require()?;
    let keys = keys(options)?;
    let components = gpui_shell::components::components().map_err(|error| format!("{error:#}"))?;
    log::to_stderr(tracing::Level::INFO);
    trust::grant_everything(home, app)?;
    let directory = app.directory().clone();
    let image = home.cache().join("shots").join(format!("{}.png", app.name()));
    gpui_platform::application()
        .with_assets(AppAssets::new(directory.clone()))
        .run(move |cx| {
            gpui_shell::init_with_components(cx, &components);
            let window_opener = components.window_opener();
            let runtime = or_exit(
                ShellRuntime::new_with_components(cx, components).map_err(|error| format!("{error:#}")),
            );
            let loader = runtime.clone();
            let mut build = move |window: &mut Window, cx: &mut App| {
                or_exit(
                    loader
                        .try_load(&directory, window, cx)
                        .map_err(|error| format!("{error:#}")),
                )
            };
            let options = hidden(cx);
            let window: AnyWindowHandle = or_exit(
                match window_opener {
                    Some(open) => {
                        let mut build = |window: &mut Window, cx: &mut App| build(window, cx).into();
                        open(cx, options, &mut build)
                    }
                    None => cx.open_window(options, build).map(Into::into),
                }
                .map_err(|error| format!("{error:#}")),
            );
            cx.spawn(async move |cx| {
                let saved = save(window, &keys, &image, cx).await;
                or_exit(saved);
                println!("{}", image.display());
                std::process::exit(0);
            })
            .detach();
        });
    unreachable!("the screenshot exits from inside the application loop")
}

fn keys(options: &[String]) -> Result<Vec<Keystroke>, String> {
    let mut keys = Vec::new();
    let mut options = options.iter();
    while let Some(option) = options.next() {
        if option != "--keys" {
            return Err(format!("shot does not take `{option}`: bmo shot <name> [--keys \"j j enter\"]"));
        }
        let listed = options.next().ok_or("--keys needs the keys to press, as in --keys \"j j enter\"")?;
        for key in listed.split_whitespace() {
            let key: &'static str = key.to_string().leak();
            keys.push(Keystroke::parse(key).map_err(|error| format!("`{key}` is not a key: {error}"))?);
        }
    }
    Ok(keys)
}

async fn save(
    window: AnyWindowHandle,
    keys: &[Keystroke],
    image: &Path,
    cx: &mut AsyncApp,
) -> Result<(), String> {
    let pause = async |time: Duration, cx: &mut AsyncApp| cx.background_executor().timer(time).await;
    let problem = |error: gpui_shell::anyhow::Error| format!("{error:#}");
    pause(SETTLE, cx).await;
    for key in keys {
        cx.update(|cx| {
            window.update(cx, |_, window, cx| {
                window.draw(cx).clear(cx);
                window.dispatch_keystroke(key.clone(), cx);
            })
        })
        .map_err(problem)?;
        pause(BETWEEN_KEYS, cx).await;
    }
    if !keys.is_empty() {
        pause(SETTLE, cx).await;
    }
    let rendered = cx
        .update(|cx| {
            window.update(cx, |_, window, cx| {
                window.draw(cx).clear(cx);
                window.render_to_image()
            })
        })
        .map_err(problem)?
        .map_err(problem)?;
    write(&rendered, image).map_err(|error| format!("cannot write {}: {error}", image.display()))
}

fn write(rendered: &image::RgbaImage, image: &Path) -> Result<(), String> {
    let folder: PathBuf = image.parent().map(Path::to_path_buf).unwrap_or_default();
    std::fs::create_dir_all(folder).map_err(|error| error.to_string())?;
    rendered.save(image).map_err(|error| error.to_string())
}

fn hidden(cx: &App) -> WindowOptions {
    WindowOptions {
        show: false,
        focus: false,
        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
            None,
            size(px(880.), px(720.)),
            cx,
        ))),
        ..Default::default()
    }
}

fn or_exit<T>(result: Result<T, String>) -> T {
    result.unwrap_or_else(|message| {
        eprintln!("bmo: {message}");
        std::process::exit(1);
    })
}
