// Key Overlay, a native Horizon mod: Shows W, A, S, D, jump and the mouse buttons on screen and lights the ones you hold.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_key_overlay`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "key_overlay";
pub const NAME: &str = "Key Overlay";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Shows W, A, S, D, jump and the mouse buttons on screen and lights the ones you hold.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("mouse", "Mouse buttons", true),
    ("space", "Jump bar", true)
];

thread_local! {
    /// Read once through the game's settings files, then kept.
    static CHOSEN: RefCell<Option<Vec<bool>>> = const { RefCell::new(None) };
}

/// A stored value: `true` or `false`, else the default.
pub fn parse(stored: Option<String>, default: bool) -> bool {
    stored.map_or(default, |v| match v.trim() {
        "true" => true,
        "false" => false,
        _ => default,
    })
}

fn chosen(get: impl Fn(&str) -> Option<String>) -> Vec<bool> {
    CHOSEN.with_borrow_mut(|c| {
        c.get_or_insert_with(|| {
            OPTIONS
                .iter()
                .map(|(key, _, default)| parse(get(key), *default))
                .collect()
        })
        .clone()
    })
}

/// Whether option `key` is on.
pub fn is(get: impl Fn(&str) -> Option<String>, key: &str) -> bool {
    OPTIONS
        .iter()
        .position(|o| o.0 == key)
        .is_some_and(|i| chosen(get)[i])
}

/// The settings page's buttons.
pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    let on = chosen(get);
    OPTIONS
        .iter()
        .zip(on)
        .map(|((_, label, _), on)| format!("{label}: {}", if on { "ON" } else { "OFF" }))
        .collect()
}

/// A click on button `i`: flips it and returns what to store (key, value).
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    let mut on = chosen(get);
    let (key, _, _) = *OPTIONS.get(i)?;
    on[i] = !on[i];
    let value = on[i].to_string();
    CHOSEN.with_borrow_mut(|c| *c = Some(on));
    Some((key, value))
}

/// Forgets what was read (tests only).
#[cfg(test)]
pub fn forget_options() {
    CHOSEN.with_borrow_mut(|c| *c = None);
}

/// Slash command that hides or shows the overlay.
pub const COMMAND: &str = "keys";
/// A held key's box and text; an idle key's.
pub const HELD_BACKGROUND: u32 = 0xB0FF_FFFF;
pub const HELD_INK: u32 = 0x00_0000;
pub const IDLE_BACKGROUND: u32 = 0x7800_0000;
pub const IDLE_INK: u32 = 0xFF_FFFF;

/// One key's box, in GUI pixels from the overlay's top-left corner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    /// The vanilla mapping it follows (`key.forward`…).
    pub mapping: &'static str,
    /// The text when the mapping's own label is not wanted.
    pub fixed: Option<&'static str>,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

const KEY: f32 = 16.0;
const GAP: f32 = 2.0;

/// The boxes to draw: W over A S D, the jump bar, then the two mouse buttons.
pub fn cells(mouse: bool, space: bool) -> Vec<Cell> {
    let step = KEY + GAP;
    let mut out = vec![
        Cell { mapping: "key.forward", fixed: None, x: step, y: 0.0, w: KEY, h: KEY },
        Cell { mapping: "key.left", fixed: None, x: 0.0, y: step, w: KEY, h: KEY },
        Cell { mapping: "key.back", fixed: None, x: step, y: step, w: KEY, h: KEY },
        Cell { mapping: "key.right", fixed: None, x: 2.0 * step, y: step, w: KEY, h: KEY },
    ];
    let mut y = 2.0 * step;
    if space {
        out.push(Cell { mapping: "key.jump", fixed: Some("SPACE"), x: 0.0, y, w: 3.0 * KEY + 2.0 * GAP, h: 12.0 });
        y += 12.0 + GAP;
    }
    if mouse {
        let w = (3.0 * KEY + 2.0 * GAP - GAP) / 2.0;
        out.push(Cell { mapping: "key.attack", fixed: Some("LMB"), x: 0.0, y, w, h: 14.0 });
        out.push(Cell { mapping: "key.use", fixed: Some("RMB"), x: w + GAP, y, w, h: 14.0 });
    }
    out
}

/// The overlay's width and height.
pub fn bounds(cells: &[Cell]) -> (f32, f32) {
    cells.iter().fold((0.0, 0.0), |(w, h), c| (f32::max(w, c.x + c.w), f32::max(h, c.y + c.h)))
}

/// A key's label as drawn: at most four characters, upper case.
pub fn short(label: &str) -> String {
    label.chars().take(4).collect::<String>().to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boxes_do_not_overlap_and_fit_the_bounds() {
        for (mouse, space) in [(true, true), (true, false), (false, true), (false, false)] {
            let cells = cells(mouse, space);
            let (w, h) = bounds(&cells);
            for (i, a) in cells.iter().enumerate() {
                assert!(a.x + a.w <= w && a.y + a.h <= h);
                for b in &cells[i + 1..] {
                    let apart = a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y;
                    assert!(apart, "{} overlaps {}", a.mapping, b.mapping);
                }
            }
        }
    }

    #[test]
    fn options_choose_the_rows() {
        assert_eq!(cells(false, false).len(), 4);
        assert_eq!(cells(true, true).len(), 7);
        assert_eq!(bounds(&cells(false, false)), (52.0, 34.0));
    }

    #[test]
    fn labels_are_short_and_upper_case() {
        assert_eq!(short("w"), "W");
        assert_eq!(short("Space"), "SPAC");
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(
            labels(none).len(),
            OPTIONS.len(),
            "one button per option"
        );
        assert_eq!(is(none, OPTIONS[0].0), OPTIONS[0].2);
        forget_options();
        let stored = |k: &str| (k == OPTIONS[0].0).then(|| "false".to_owned());
        assert!(!is(stored, OPTIONS[0].0));
        forget_options();
        assert!(parse(Some("junk".into()), true));
        assert!(!parse(Some(" false\n".into()), true));
    }

    #[test]
    fn a_click_flips_and_reports_what_to_store() {
        forget_options();
        let none = |_: &str| None;
        let (key, value) = click(none, 0).expect("button 0 exists");
        assert_eq!(key, OPTIONS[0].0);
        assert_eq!(value, (!OPTIONS[0].2).to_string());
        assert_eq!(is(none, key), !OPTIONS[0].2);
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
}
