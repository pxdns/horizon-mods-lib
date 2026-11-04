// Apple Skin, a native Horizon mod: Your hunger and saturation as numbers and as overlays on the food bar, and what the food in your hand would restore.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_apple_skin`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "apple_skin";
pub const NAME: &str = "Apple Skin";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Your hunger and saturation as numbers and as overlays on the food bar, and what the food in your hand would restore.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("preview", "Held food preview", true),
    ("saturation", "Show saturation", true),
    ("overlay", "Draw on the food bar", true),
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
/// Translucent black box, white text with a shadow, as Horizon's own HUD extras.
pub const BACKGROUND: u32 = 0x7800_0000;
#[allow(dead_code)]
pub const INK: u32 = 0xFF_FFFF;

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "food";
/// A full food bar.
pub const MAX_FOOD: i32 = 20;

/// What eating a food would do, by `FoodData.eat`: food and saturation after, and the nutrition that is wasted.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Preview {
    pub food: i32,
    pub saturation: f32,
    pub restored: i32,
    pub wasted: i32,
}

/// `food.food = (food + nutrition).clamp(0, 20); saturation = (saturation + gain).clamp(0, food)`.
pub fn preview(food: i32, saturation: f32, nutrition: u32, gain: f32) -> Preview {
    let after = (food + nutrition as i32).clamp(0, MAX_FOOD);
    let sat = (saturation + gain).clamp(0.0, after as f32);
    Preview {
        food: after,
        saturation: sat,
        restored: after - food,
        wasted: nutrition as i32 - (after - food),
    }
}

/// Rectangles `([x, y, w, h], argb)` drawn over the vanilla food bar (ten icons right to left from the right of the
/// hotbar, at `h - 39`): a tint where the held food would fill it, and a thin gold line for the saturation.
pub fn food_overlay(
    w: i32,
    h: i32,
    food: i32,
    saturation: f32,
    after: Option<i32>,
    show_saturation: bool,
) -> Vec<([f32; 4], u32)> {
    let mut out = Vec::new();
    let y = (h - 39) as f32;
    for n in 0..10 {
        let x = (w / 2 + 91 - n * 8 - 9) as f32;
        let low = 2 * n;
        if let Some(a) = after {
            let gain = a.clamp(low, low + 2) - food.clamp(low, low + 2);
            let width = match gain {
                2 => 9.0,
                1 => 5.0,
                _ => 0.0,
            };
            if width > 0.0 {
                out.push(([x, y, width, 9.0], 0x66_FFAA33));
            }
        }
        if show_saturation {
            let amount = (saturation - low as f32).clamp(0.0, 2.0) / 2.0;
            let width = (9.0 * amount).round();
            if width > 0.0 {
                out.push(([x, y + 8.0, width, 1.0], 0xFF_FFD23F));
            }
        }
    }
    out
}

/// The box's lines.
pub fn lines(
    food: i32,
    saturation: f32,
    held: Option<Preview>,
    show_saturation: bool,
) -> Vec<String> {
    let mut out = vec![format!("Food {food}/{MAX_FOOD}")];
    if show_saturation {
        out.push(format!("Saturation {saturation:.1}"));
    }
    if let Some(p) = held {
        let mut text = format!("Eat: +{} food", p.restored);
        if show_saturation {
            text.push_str(&format!(", saturation to {:.1}", p.saturation));
        }
        if p.wasted > 0 {
            text.push_str(&format!(" ({} wasted)", p.wasted));
        }
        out.push(text);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eating_clamps_like_the_game() {
        let p = preview(10, 2.0, 6, 7.2);
        assert_eq!((p.food, p.restored, p.wasted), (16, 6, 0));
        assert!((p.saturation - 9.2).abs() < 1e-4);
        let full = preview(18, 5.0, 8, 12.8);
        assert_eq!((full.food, full.restored, full.wasted), (20, 2, 6));
        assert_eq!(full.saturation, 17.8_f32.min(20.0));
        let capped = preview(20, 19.0, 4, 10.0);
        assert_eq!(
            capped.saturation, 20.0,
            "saturation never passes the food level"
        );
    }

    #[test]
    fn the_overlay_sits_on_the_food_icons() {
        let out = food_overlay(320, 240, 17, 3.0, Some(20), true);
        // Icons 8 and 9 (points 16 to 20) get a tint: a half and a full one; saturation 3 fills icon 0 and half of icon 1.
        let tints: Vec<_> = out.iter().filter(|(_, c)| *c == 0x66_FFAA33).collect();
        assert_eq!(tints.len(), 2);
        assert_eq!(
            tints[0].0,
            [(320 / 2 + 91 - 8 * 8 - 9) as f32, 201.0, 5.0, 9.0]
        );
        let lines: Vec<_> = out.iter().filter(|(_, c)| *c == 0xFF_FFD23F).collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].0, [242.0, 209.0, 9.0, 1.0]);
        assert_eq!(lines[1].0[2], 5.0, "half an icon, rounded up from 4.5");
        assert!(food_overlay(320, 240, 20, 0.0, None, false).is_empty());
    }

    #[test]
    fn lines_follow_the_options() {
        assert_eq!(
            lines(18, 4.25, None, true),
            ["Food 18/20", "Saturation 4.2"]
        );
        assert_eq!(lines(18, 4.25, None, false), ["Food 18/20"]);
        let held = Some(preview(18, 4.0, 8, 12.8));
        let out = lines(18, 4.0, held, true);
        assert_eq!(out[2], "Eat: +2 food, saturation to 16.8 (6 wasted)");
        assert_eq!(
            lines(10, 0.0, Some(preview(10, 0.0, 4, 2.4)), false)[1],
            "Eat: +4 food"
        );
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(labels(none).len(), OPTIONS.len(), "one button per option");
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
