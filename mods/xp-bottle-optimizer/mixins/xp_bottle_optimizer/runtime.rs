// XP Bottle Optimizer, a native Horizon mod: Your level, the experience points left to the next one and how many bottles of enchanting that takes.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_xp_bottle_optimizer`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "xp_bottle_optimizer";
pub const NAME: &str = "XP Bottle Optimizer";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Your level, the experience points left to the next one and how many bottles of enchanting that takes.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("carried", "Show bottles carried", true),
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
pub const COMMAND: &str = "xpbottles";

/// What one bottle o' enchanting gives: 3 to 11 points (vanilla `ExperienceBottle`: 3 + rand(5) + rand(5)).
pub const BOTTLE_MIN: i32 = 3;
pub const BOTTLE_MAX: i32 = 11;

/// The points a level takes to fill (vanilla `Player.getXpNeededForNextLevel`).
pub fn needed(level: i32) -> i32 {
    match level {
        i32::MIN..=14 => 2 * level.max(0) + 7,
        15..=29 => 5 * level - 38,
        _ => 9 * level - 158,
    }
}

/// The points still missing from `level` with `progress` (0 to 1) of it filled.
pub fn missing(level: i32, progress: f32) -> i32 {
    let n = needed(level);
    (n - (progress.clamp(0.0, 1.0) * n as f32) as i32).clamp(0, n)
}

/// How many bottles fill `points`: the fewest (all give 11) and the most (all give 3).
pub fn bottles(points: i32) -> (i32, i32) {
    if points <= 0 {
        return (0, 0);
    }
    (
        (points + BOTTLE_MAX - 1) / BOTTLE_MAX,
        (points + BOTTLE_MIN - 1) / BOTTLE_MIN,
    )
}

/// The box's lines.
pub fn lines(level: i32, progress: f32, carried: Option<u32>) -> Vec<String> {
    let points = missing(level, progress);
    let (few, many) = bottles(points);
    let mut out = vec![format!("Level {level}: {points} to {}", level + 1)];
    out.push(if few == many {
        format!("{few} bottles")
    } else {
        format!("{few} to {many} bottles")
    });
    if let Some(n) = carried {
        out.push(format!("Carrying {n}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_level_takes_what_vanilla_says() {
        // Vanilla: 7 at level 0, 17 at 5, 37 at 15, 112 at 30, 121 at 31.
        assert_eq!(
            [needed(0), needed(5), needed(15), needed(30), needed(31)],
            [7, 17, 37, 112, 121]
        );
    }

    #[test]
    fn missing_points_follow_the_bar() {
        assert_eq!(missing(0, 0.0), 7);
        assert_eq!(missing(5, 0.5), 9);
        assert_eq!(missing(30, 1.0), 0);
        assert_eq!(missing(30, 2.0), 0);
    }

    #[test]
    fn bottles_are_a_range() {
        assert_eq!(bottles(0), (0, 0));
        assert_eq!(bottles(11), (1, 4));
        assert_eq!(bottles(112), (11, 38));
    }

    #[test]
    fn lines_name_level_and_bottles() {
        assert_eq!(lines(0, 0.0, None), ["Level 0: 7 to 1", "1 to 3 bottles"]);
        assert_eq!(lines(30, 1.0, Some(4))[2], "Carrying 4");
        assert_eq!(lines(30, 1.0, None)[1], "0 bottles");
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
