// Apple Skin, a native Horizon mod: Your hunger and saturation as numbers, and what the food in your hand would restore.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_apple_skin`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "apple_skin";
pub const NAME: &str = "Apple Skin";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Your hunger and saturation as numbers, and what the food in your hand would restore.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("preview", "Held food preview", true),
    ("saturation", "Show saturation", true),
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
