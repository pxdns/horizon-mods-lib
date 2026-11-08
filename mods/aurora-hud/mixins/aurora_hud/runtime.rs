// Aurora HUD, a native Horizon mod: Aurora client's info lines in one box: the day, the game clock, the light level, the count of the held item and a stopwatch for the time in this world.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_aurora_hud`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "aurora_hud";
pub const NAME: &str = "Aurora HUD";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "The Aurora client's info lines in one box: the day, the game clock, the light level, the count of the held item and how long you have been in this world.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("right", "Right side", false),
    ("day", "Day", true),
    ("clock", "Game clock", true),
    ("light", "Light level", true),
    ("held", "Held item count", true),
    ("stopwatch", "Time in this world", true),
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
pub const COMMAND: &str = "aurora";

/// What the box can show.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Reading {
    /// The world's time of day in ticks; it keeps counting past 24000.
    pub day_time: i64,
    /// The brightest light at the player's feet, 0 to 15.
    pub light: u8,
    /// The count of the held stack, 0 for an empty hand.
    pub held: u32,
    /// Ticks since the player entered this world.
    pub world_ticks: u64,
}

/// The day, from 1: a day is 24000 ticks.
pub fn day(day_time: i64) -> i64 {
    day_time.max(0) / 24_000 + 1
}

/// The game clock as `HH:MM`. Tick 0 is 06:00, as the sun rises.
pub fn clock(day_time: i64) -> String {
    let minutes = ((day_time.rem_euclid(24_000) * 1440 / 24_000) + 6 * 60) % 1440;
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}

/// `m:ss`, or `h:mm:ss` from an hour.
pub fn stopwatch(ticks: u64) -> String {
    let s = ticks / 20;
    let (h, m, s) = (s / 3600, s / 60 % 60, s % 60);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

/// The box's lines: `show` is day, clock, light, held item, stopwatch.
pub fn lines(r: Reading, show: [bool; 5]) -> Vec<String> {
    let mut out = Vec::new();
    if show[0] {
        out.push(format!("Day {}", day(r.day_time)));
    }
    if show[1] {
        out.push(format!("Time {}", clock(r.day_time)));
    }
    if show[2] {
        out.push(format!("Light {}", r.light));
    }
    if show[3] && r.held > 0 {
        out.push(format!("Holding {}", r.held));
    }
    if show[4] {
        out.push(format!("In world {}", stopwatch(r.world_ticks)));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_day_is_24000_ticks_counted_from_one() {
        assert_eq!(day(0), 1);
        assert_eq!(day(23_999), 1);
        assert_eq!(day(24_000), 2);
        assert_eq!(day(-5), 1);
    }

    #[test]
    fn the_clock_starts_at_six_in_the_morning() {
        assert_eq!(clock(0), "06:00");
        assert_eq!(clock(6000), "12:00");
        assert_eq!(clock(12_000), "18:00");
        assert_eq!(clock(18_000), "00:00");
        assert_eq!(clock(24_000), "06:00");
        assert_eq!(clock(1000), "07:00");
    }

    #[test]
    fn the_stopwatch_counts_twenty_ticks_to_a_second() {
        assert_eq!(stopwatch(0), "0:00");
        assert_eq!(stopwatch(20 * 75), "1:15");
        assert_eq!(stopwatch(20 * 3661), "1:01:01");
    }

    #[test]
    fn only_the_picked_lines_show_and_an_empty_hand_has_no_line() {
        let r = Reading { day_time: 30_000, light: 12, held: 0, world_ticks: 1200 };
        assert_eq!(lines(r, [true, false, false, false, false]), ["Day 2"]);
        assert_eq!(
            lines(Reading { held: 64, ..r }, [true, true, true, true, true]),
            ["Day 2", "Time 12:00", "Light 12", "Holding 64", "In world 1:00"]
        );
        assert_eq!(
            lines(r, [true, true, true, true, true]),
            ["Day 2", "Time 12:00", "Light 12", "In world 1:00"]
        );
        assert!(lines(r, [false; 5]).is_empty());
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
