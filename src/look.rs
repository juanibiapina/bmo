use std::sync::OnceLock;

use gpui_shell::gpui::{App, Hsla, Rgba, WindowAppearance};

use crate::theme::{self, Appearance};

static CHOSEN: OnceLock<Appearance> = OnceLock::new();

pub fn choose(appearance: Appearance) {
    let _ = CHOSEN.set(appearance);
}

pub fn appearance(cx: &App) -> Appearance {
    CHOSEN.get().copied().unwrap_or(match cx.window_appearance() {
        WindowAppearance::Light | WindowAppearance::VibrantLight => Appearance::Light,
        _ => Appearance::Dark,
    })
}

pub fn install(cx: &mut App) {
    let colors = theme::colors(appearance(cx));
    let color = |name: &str| {
        colors
            .iter()
            .find(|(candidate, _)| candidate == name)
            .map(|(_, hex)| parsed(hex))
            .unwrap_or_default()
    };
    let tokens = &mut gpui_base::Theme::global_mut(cx).tokens.colors;
    tokens.background = color("background");
    tokens.foreground = color("foreground");
    tokens.surface = color("surface");
    tokens.surface_foreground = color("foreground");
    tokens.primary = color("solid");
    tokens.primary_foreground = color("solid_foreground");
    tokens.secondary = color("raised");
    tokens.secondary_foreground = color("foreground");
    tokens.muted = color("raised");
    tokens.muted_foreground = color("muted");
    tokens.accent = color("accent");
    tokens.accent_foreground = color("accent_foreground");
    tokens.destructive = color("blocked");
    tokens.destructive_foreground = color("background");
    tokens.border = color("border");
    tokens.input = color("input");
    tokens.ring = color("ring");
    tokens.selection = color("selection");
}

fn parsed(hex: &str) -> Hsla {
    let digits = hex.trim_start_matches('#');
    let channel = |index: usize| {
        digits
            .get(index * 2..index * 2 + 2)
            .and_then(|pair| u8::from_str_radix(pair, 16).ok())
            .map(|value| value as f32 / 255.)
    };
    Rgba {
        r: channel(0).unwrap_or(0.),
        g: channel(1).unwrap_or(0.),
        b: channel(2).unwrap_or(0.),
        a: channel(3).unwrap_or(1.),
    }
    .into()
}
