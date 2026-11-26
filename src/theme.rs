#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Appearance {
    Dark,
    Light,
}

struct Palette {
    background: u32,
    selection: u32,
    status: [u32; 3],
    ansi: [u32; 16],
}

const GRAPHITE: Palette = Palette {
    background: 0x17191d,
    selection: 0x394454,
    status: [0xe5c07b, 0xf0705f, 0x76a8ff],
    ansi: [
        0x45484f, 0xdc8c95, 0xa6bd91, 0xd8be87, 0x96b2d7, 0xb5a2ce, 0x8cbdc3, 0xd2d4d9, 0x818793,
        0xf0a5ad, 0xbad5a2, 0xead2a1, 0xb0cbef, 0xcbb7e6, 0xa4d5db, 0xf0f0f2,
    ],
};

const PAPER: Palette = Palette {
    background: 0xffffff,
    selection: 0xcfdcf5,
    status: [0x9a6700, 0xc93c2c, 0x2f6fe4],
    ansi: [
        0x25272c, 0xc62a2a, 0x1a7f37, 0x8f5d00, 0x1f62d0, 0x7a45c8, 0x0f6f74, 0x5f6068, 0x7b7e87,
        0xd1242f, 0x1f883d, 0x9a6700, 0x0969da, 0x8250df, 0x1b7c83, 0x111214,
    ],
};

pub fn colors(appearance: Appearance) -> Vec<(String, String)> {
    let light = appearance == Appearance::Light;
    let palette = if light { &PAPER } else { &GRAPHITE };
    let background = palette.background;
    let foreground = if light { 0x1b1b1e } else { 0xeeeeef };
    let [working, blocked, done] = palette.status;
    let toward = |other: u32, amount: f32| hex(mix(background, other, amount));
    let warm = 0xe8e8e2;
    let surfaces = if light {
        [
            ("popover", hex(background)),
            ("raised", toward(0, 0.035)),
            ("hover", "#0000000c".into()),
            ("selected", "#00000012".into()),
            ("border", "#00000017".into()),
            ("input", "#00000026".into()),
            ("canvas", toward(warm, 0.2)),
            ("chrome", toward(warm, 0.6)),
        ]
    } else {
        [
            ("popover", toward(0xffffff, 0.06)),
            ("raised", toward(0xffffff, 0.09)),
            ("hover", "#ffffff0e".into()),
            ("selected", "#ffffff17".into()),
            ("border", "#ffffff18".into()),
            ("input", "#ffffff24".into()),
            ("canvas", toward(0, 0.16)),
            ("chrome", toward(0, 0.32)),
        ]
    };
    let solid_foreground = if light { 0xffffff } else { mix(background, 0, 0.28) };
    let mut colors: Vec<(String, String)> = [
        ("background", hex(background)),
        ("surface", hex(background)),
        ("foreground", hex(foreground)),
        ("muted", toward(foreground, if light { 0.66 } else { 0.6 })),
        ("dim", toward(foreground, if light { 0.46 } else { 0.38 })),
        ("accent", hex(palette.ansi[4])),
        ("accent_foreground", hex(background)),
        ("solid", hex(foreground)),
        ("solid_foreground", hex(solid_foreground)),
        ("ring", toward(foreground, 0.7)),
        ("selection", hex(palette.selection)),
        ("working", hex(working)),
        ("blocked", hex(blocked)),
        ("done", hex(done)),
        ("idle", toward(foreground, 0.38)),
    ]
    .into_iter()
    .chain(surfaces)
    .map(|(name, color)| (name.to_string(), color))
    .collect();
    for (index, color) in palette.ansi.iter().enumerate() {
        colors.push((format!("ansi{index}"), hex(*color)));
    }
    colors
}

fn hex(color: u32) -> String {
    format!("#{color:06x}")
}

fn mix(from: u32, to: u32, amount: f32) -> u32 {
    [16, 8, 0].iter().fold(0, |mixed, shift| {
        let (start, end) = (((from >> shift) & 255) as f32, ((to >> shift) & 255) as f32);
        mixed | (((start * (1.0 - amount) + end * amount).round() as u32) << shift)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn color(appearance: Appearance, name: &str) -> String {
        colors(appearance)
            .into_iter()
            .find(|(candidate, _)| candidate == name)
            .map(|(_, color)| color)
            .unwrap_or_else(|| panic!("no color named {name}"))
    }

    #[test]
    fn dark_is_herdrs_graphite() {
        assert_eq!(color(Appearance::Dark, "background"), "#17191d");
        assert_eq!(color(Appearance::Dark, "foreground"), "#eeeeef");
        assert_eq!(color(Appearance::Dark, "accent"), "#96b2d7");
        assert_eq!(color(Appearance::Dark, "hover"), "#ffffff0e");
        assert_eq!(color(Appearance::Dark, "working"), "#e5c07b");
        assert_eq!(color(Appearance::Dark, "raised"), "#2c2e31");
    }

    #[test]
    fn light_is_herdrs_paper() {
        assert_eq!(color(Appearance::Light, "background"), "#ffffff");
        assert_eq!(color(Appearance::Light, "foreground"), "#1b1b1e");
        assert_eq!(color(Appearance::Light, "accent"), "#1f62d0");
        assert_eq!(color(Appearance::Light, "selected"), "#00000012");
        assert_eq!(color(Appearance::Light, "blocked"), "#c93c2c");
    }

    #[test]
    fn both_appearances_name_the_same_colors() {
        let names = |appearance| {
            let mut names: Vec<String> = colors(appearance).into_iter().map(|(name, _)| name).collect();
            names.sort();
            names
        };
        assert_eq!(names(Appearance::Dark), names(Appearance::Light));
    }
}
