// Crosshair Editor, a native Horizon mod: Replace the crosshair with your own: shape, size, gap, thickness, colour (or rainbow), an outline and a centre dot.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_crosshair_editor`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "crosshair_editor";
pub const NAME: &str = "Crosshair Editor";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Replace the crosshair with your own: shape, size, gap, thickness, colour (or rainbow), an outline and a centre dot.";

use std::cell::RefCell;

/// An option that cycles through choices: its stored key, label, the values it can take and the default one.
pub type Choice = (&'static str, &'static str, &'static [&'static str], usize);

/// The mod's settings, in the order of the settings page's buttons. The first is on or off.
pub const OPTIONS: &[Choice] = &[
    ("shown", "Custom crosshair", &["true", "false"], 0),
    ("shape", "Shape", &["cross", "tee", "dot"], 0),
    ("size", "Size", &["2", "3", "4", "5", "6", "7", "8", "10", "12", "15"], 3),
    ("gap", "Gap", &["0", "1", "2", "3", "4", "5", "6", "8"], 1),
    ("thickness", "Thickness", &["1", "2", "3", "4"], 0),
    (
        "colour",
        "Colour",
        &["white", "red", "green", "blue", "yellow", "cyan", "magenta", "orange", "black", "rainbow"],
        0,
    ),
    ("outline", "Outline", &["true", "false"], 0),
    ("dot", "Centre dot", &["false", "true"], 0),
];

thread_local! {
    /// Read once through the game's settings files, then kept: the index chosen for each option.
    static CHOSEN: RefCell<Option<Vec<usize>>> = const { RefCell::new(None) };
}

/// The index of `stored` among the choices, else the default.
pub fn parse(stored: Option<String>, choices: &[&str], default: usize) -> usize {
    stored
        .and_then(|v| choices.iter().position(|c| *c == v.trim()))
        .unwrap_or(default)
}

fn chosen(get: impl Fn(&str) -> Option<String>) -> Vec<usize> {
    CHOSEN.with_borrow_mut(|c| {
        c.get_or_insert_with(|| {
            OPTIONS
                .iter()
                .map(|(key, _, choices, default)| parse(get(key), choices, *default))
                .collect()
        })
        .clone()
    })
}

/// The value option `key` has now.
pub fn value(get: impl Fn(&str) -> Option<String>, key: &str) -> &'static str {
    let at = chosen(get);
    OPTIONS
        .iter()
        .position(|o| o.0 == key)
        .map_or("", |i| OPTIONS[i].2[at[i]])
}

/// Whether option `key` (an on or off one) is on.
pub fn is(get: impl Fn(&str) -> Option<String>, key: &str) -> bool {
    value(get, key) == "true"
}

fn shown(value: &str) -> String {
    match value {
        "true" => "ON".to_owned(),
        "false" => "OFF".to_owned(),
        other => {
            let mut c = other.chars();
            c.next()
                .map_or(String::new(), |f| f.to_uppercase().collect::<String>() + c.as_str())
        }
    }
}

/// The settings page's buttons.
pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let at = chosen(get);
    OPTIONS
        .iter()
        .zip(at)
        .map(|((_, label, choices, _), i)| format!("{label}: {}", shown(choices[i])))
        .collect()
}

/// A click on button `i`: the next choice, and what to store (key, value).
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut at = chosen(get);
    let (key, _, choices, _) = *OPTIONS.get(i)?;
    at[i] = (at[i] + 1) % choices.len();
    let value = choices[at[i]].to_owned();
    CHOSEN.with_borrow_mut(|c| *c = Some(at));
    Some((key, value))
}

/// Forgets what was read (tests only).
#[cfg(test)]
pub fn forget_options() {
    CHOSEN.with_borrow_mut(|c| *c = None);
}

/// Slash command that switches the custom crosshair on or off.
pub const COMMAND: &str = "crosshair";

/// What the crosshair looks like.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Look {
    pub shape: &'static str,
    pub size: i32,
    pub gap: i32,
    pub thickness: i32,
    pub colour: u32,
    pub outline: bool,
    pub dot: bool,
}

/// An opaque colour from a name; `rainbow` turns with `ticks`.
pub fn colour(name: &str, ticks: u32) -> u32 {
    let rgb = match name {
        "red" => 0xFF_5555,
        "green" => 0x55_FF55,
        "blue" => 0x55_55FF,
        "yellow" => 0xFF_FF55,
        "cyan" => 0x55_FFFF,
        "magenta" => 0xFF_55FF,
        "orange" => 0xFF_AA00,
        "black" => 0x00_0000,
        "rainbow" => hue(ticks.wrapping_mul(3) % 360),
        _ => 0xFF_FFFF,
    };
    0xFF00_0000 | rgb
}

/// A colour of full saturation and value at `degrees` round the wheel.
fn hue(degrees: u32) -> u32 {
    let h = degrees % 360;
    let x = 255 - (255 * (h % 120).abs_diff(60) / 60).min(255);
    let (r, g, b) = match h / 60 {
        0 => (255, x, 0),
        1 => (x, 255, 0),
        2 => (0, 255, x),
        3 => (0, x, 255),
        4 => (x, 0, 255),
        _ => (255, 0, x),
    };
    (r << 16) | (g << 8) | b
}

/// The settings as a `Look`.
pub fn look(get: impl Fn(&str) -> Option<String> + Copy, ticks: u32) -> Look {
    let number = |key: &str| value(get, key).parse().unwrap_or(0);
    Look {
        shape: value(get, "shape"),
        size: number("size"),
        gap: number("gap"),
        thickness: number("thickness").max(1),
        colour: colour(value(get, "colour"), ticks),
        outline: is(get, "outline"),
        dot: is(get, "dot"),
    }
}

/// The rectangles `([x, y, w, h], argb)` of a crosshair centred on the pixel (`cx`, `cy`), in drawing order: the outline
/// (black, one pixel round each part) first when it is on, then the parts.
pub fn rects(look: Look, cx: i32, cy: i32) -> Vec<([f32; 4], u32)> {
    let (s, g, t) = (look.size, look.gap, look.thickness);
    let half = t / 2;
    let mut parts: Vec<[i32; 4]> = Vec::new();
    if look.shape != "dot" {
        parts.push([cx + g, cy - half, s, t]); // right
        parts.push([cx - g - s, cy - half, s, t]); // left
        parts.push([cx - half, cy + g, t, s]); // down
        if look.shape != "tee" {
            parts.push([cx - half, cy - g - s, t, s]); // up
        }
    }
    if look.dot || look.shape == "dot" {
        parts.push([cx - half, cy - half, t, t]);
    }
    let mut out = Vec::new();
    if look.outline {
        out.extend(parts.iter().map(|p| {
            (
                [(p[0] - 1) as f32, (p[1] - 1) as f32, (p[2] + 2) as f32, (p[3] + 2) as f32],
                0xFF00_0000,
            )
        }));
    }
    out.extend(
        parts
            .iter()
            .map(|p| ([p[0] as f32, p[1] as f32, p[2] as f32, p[3] as f32], look.colour)),
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn look(shape: &'static str) -> Look {
        Look { shape, size: 5, gap: 2, thickness: 1, colour: 0xFFFF_FFFF, outline: false, dot: false }
    }

    #[test]
    fn a_cross_has_four_arms_round_a_gap() {
        let r = rects(look("cross"), 100, 50);
        let arms: Vec<[f32; 4]> = r.iter().map(|x| x.0).collect();
        assert_eq!(
            arms,
            [[102.0, 50.0, 5.0, 1.0], [93.0, 50.0, 5.0, 1.0], [100.0, 52.0, 1.0, 5.0], [100.0, 43.0, 1.0, 5.0]]
        );
    }

    #[test]
    fn a_tee_has_no_top_arm_and_a_dot_is_one_square() {
        assert_eq!(rects(look("tee"), 0, 0).len(), 3);
        let dot = rects(look("dot"), 10, 10);
        assert_eq!(dot.len(), 1);
        assert_eq!(dot[0].0, [10.0, 10.0, 1.0, 1.0]);
    }

    #[test]
    fn the_centre_dot_adds_a_square_and_the_outline_goes_first() {
        let with_dot = rects(Look { dot: true, ..look("cross") }, 0, 0);
        assert_eq!(with_dot.len(), 5);
        let outlined = rects(Look { outline: true, ..look("cross") }, 0, 0);
        assert_eq!(outlined.len(), 8, "four black parts, then four coloured");
        assert_eq!(outlined[0].1, 0xFF00_0000);
        assert_eq!(outlined[0].0, [1.0, -1.0, 7.0, 3.0], "one pixel round the right arm");
        assert_eq!(outlined[7].1, 0xFFFF_FFFF);
    }

    #[test]
    fn a_thick_crosshair_stays_centred() {
        let r = rects(Look { thickness: 3, ..look("cross") }, 100, 50);
        assert_eq!(r[0].0, [102.0, 49.0, 5.0, 3.0]);
        assert_eq!(r[2].0, [99.0, 52.0, 3.0, 5.0]);
    }

    #[test]
    fn colours_come_from_names_and_the_rainbow_turns() {
        assert_eq!(colour("red", 0), 0xFFFF_5555);
        assert_eq!(colour("nonsense", 0), 0xFFFF_FFFF);
        assert_eq!(colour("rainbow", 0), 0xFFFF_0000);
        assert_ne!(colour("rainbow", 0), colour("rainbow", 40));
        assert_eq!(colour("rainbow", 40) >> 24, 0xFF);
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(labels(none).len(), OPTIONS.len(), "one button per option");
        assert_eq!(labels(none)[0], "Custom crosshair: ON");
        assert_eq!(labels(none)[2], "Size: 5");
        assert!(is(none, "shown"));
        forget_options();
        let stored = |k: &str| (k == "size").then(|| "10".to_owned());
        assert_eq!(value(stored, "size"), "10");
        forget_options();
        assert_eq!(parse(Some("junk".into()), &["a", "b"], 1), 1);
        assert_eq!(parse(Some(" b\n".into()), &["a", "b"], 0), 1);
    }

    #[test]
    fn a_click_moves_to_the_next_choice_and_wraps() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(click(none, 0), Some(("shown", "false".to_owned())));
        assert_eq!(click(none, 0), Some(("shown", "true".to_owned())));
        assert_eq!(click(none, 1), Some(("shape", "tee".to_owned())));
        assert_eq!(click(none, 1), Some(("shape", "dot".to_owned())));
        assert_eq!(click(none, 1), Some(("shape", "cross".to_owned())));
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
}
