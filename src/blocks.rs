use gpui_shell::gpui::WindowAppearance;
use gpui_shell::{HostArguments, HostError, HostModule, HostObject, HostValue};

use crate::theme::{self, Appearance};

const DECLARATIONS: &str = include_str!("blocks.d.ts");

pub fn export() -> Result<(), HostError> {
    gpui_shell::export_module(
        HostModule::new("bmo")
            .function("theme", |_: &HostArguments| Ok(colors()))
            .declarations(DECLARATIONS),
    )
}

fn colors() -> HostValue {
    theme::colors(appearance())
        .into_iter()
        .fold(HostObject::new(), |object, (name, color)| object.field(name, color))
        .into()
}

fn appearance() -> Appearance {
    match gpui_shell::with_current_app(|app| app.window_appearance()) {
        Some(WindowAppearance::Light | WindowAppearance::VibrantLight) => Appearance::Light,
        _ => Appearance::Dark,
    }
}
