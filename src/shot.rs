use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::time::Duration;

use gpui_shell::gpui::{
    AnyWindowHandle, App, AsyncApp, Bounds, Keystroke, Window, WindowBounds, WindowOptions, px,
    size,
};

use crate::app::App as BmoApp;
use crate::home::Home;
use crate::look;
use crate::theme::Appearance;
use crate::stage::{self, or_exit, problem};

const SETTLE: Duration = Duration::from_millis(500);
const BETWEEN_KEYS: Duration = Duration::from_millis(50);

pub fn capture(home: &Home, app: &BmoApp, options: &[String]) -> Result<Infallible, String> {
    let Options { keys, label, settle } = options_given(options)?;
    let file = match label {
        Some(label) => format!("{}-{label}.png", app.name()),
        None => format!("{}.png", app.name()),
    };
    let image = home.cache().join("shots").join(file);
    stage::open(home, app, false, move |stage, cx| {
        let runtime = stage.runtime.clone();
        let directory = stage.directory.clone();
        let window = or_exit(stage.window(
            hidden(cx),
            move |window: &mut Window, cx: &mut App| {
                or_exit(runtime.try_load(&directory, window, cx).map_err(problem))
            },
            cx,
        ));
        cx.spawn(async move |cx| {
            or_exit(save(window, &keys, settle, &image, cx).await);
            println!("{}", image.display());
            std::process::exit(0);
        })
        .detach();
    })
}

struct Options {
    keys: Vec<Keystroke>,
    label: Option<String>,
    settle: Duration,
}

fn options_given(options: &[String]) -> Result<Options, String> {
    let mut keys = Vec::new();
    let mut label = None;
    let mut settle = SETTLE;
    let mut options = options.iter();
    while let Some(option) = options.next() {
        match option.as_str() {
            "--as" => {
                let given = options.next().ok_or("--as needs a label, as in --as help")?;
                if !given.chars().all(|character| character.is_ascii_alphanumeric() || character == '-') {
                    return Err(format!("`{given}` is not a label: use letters, digits and dashes"));
                }
                label = Some(given.clone());
                continue;
            }
            "--wait" => {
                settle = options
                    .next()
                    .and_then(|given| given.parse().ok())
                    .filter(|seconds: &f64| seconds.is_finite() && *seconds >= 0.)
                    .map(Duration::from_secs_f64)
                    .ok_or("--wait needs seconds, as in --wait 5")?;
                continue;
            }
            "--dark" => {
                look::choose(Appearance::Dark);
                continue;
            }
            "--light" => {
                look::choose(Appearance::Light);
                continue;
            }
            "--keys" => {}
            _ => {
                return Err(format!(
                    "shot does not take `{option}`: bmo shot <name> [--keys \"j j enter\"] [--wait <seconds>] [--as <label>] [--dark | --light]"
                ));
            }
        }
        let listed = options
            .next()
            .ok_or("--keys needs the keys to press, as in --keys \"j j enter\"")?;
        for key in listed.split_whitespace() {
            let key: &'static str = key.to_string().leak();
            keys.push(Keystroke::parse(key).map_err(|error| format!("`{key}` is not a key: {error}"))?);
        }
    }
    Ok(Options { keys, label, settle })
}

async fn save(
    window: AnyWindowHandle,
    keys: &[Keystroke],
    settle: Duration,
    image: &Path,
    cx: &mut AsyncApp,
) -> Result<(), String> {
    let pause = async |time: Duration, cx: &mut AsyncApp| cx.background_executor().timer(time).await;
    pause(settle, cx).await;
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
        pause(settle, cx).await;
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

pub fn hidden(cx: &App) -> WindowOptions {
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
