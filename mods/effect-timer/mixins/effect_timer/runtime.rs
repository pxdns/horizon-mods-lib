// Effect Timer, a native Horizon mod: Your active potion effects and how long each has left, red in the last ten seconds.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_effect_timer`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "effect_timer";
pub const NAME: &str = "Effect Timer";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Your active potion effects and how long each has left, red in the last ten seconds.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("shown", "Shown", true),
    ("ambient", "Show beacon effects", true),
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

/// `minecraft:diamond_helmet` as `Diamond Helmet`.
pub fn title(id: &str) -> String {
    id.rsplit(':')
        .next()
        .unwrap_or(id)
        .split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| {
                f.to_uppercase().collect::<String>() + c.as_str()
            })
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Slash command that hides or shows the box.
pub const COMMAND: &str = "effects";

/// One active effect, as the game keeps it.
#[derive(Clone, Debug, PartialEq)]
pub struct Active {
    pub id: String,
    /// Ticks left; -1 is endless.
    pub duration: i32,
    pub amplifier: i32,
    pub ambient: bool,
}

/// `m:ss`, or `h:mm:ss` from an hour; `endless` for -1.
pub fn clock(ticks: i32) -> String {
    if ticks < 0 {
        return "endless".to_owned();
    }
    let s = (ticks + 19) / 20;
    let (h, m, s) = (s / 3600, s / 60 % 60, s % 60);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

/// Roman numerals for the amplifier (0 reads as nothing, 1 as II).
pub fn level(amplifier: i32) -> String {
    const NUMERALS: [&str; 10] = ["", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];
    match usize::try_from(amplifier) {
        Ok(0) | Err(_) => String::new(),
        Ok(n) if n < NUMERALS.len() => NUMERALS[n].to_owned(),
        Ok(n) => (n + 1).to_string(),
    }
}

/// The box's lines, shortest time first; red when ten seconds or fewer are left.
pub fn lines(list: &[Active], show_ambient: bool) -> Vec<(String, u32)> {
    let mut shown: Vec<&Active> = list.iter().filter(|e| show_ambient || !e.ambient).collect();
    shown.sort_by_key(|e| if e.duration < 0 { i32::MAX } else { e.duration });
    shown
        .into_iter()
        .map(|e| {
            let name = title(&e.id);
            let tier = level(e.amplifier);
            let label = if tier.is_empty() {
                name
            } else {
                format!("{name} {tier}")
            };
            let ink = if (0..=200).contains(&e.duration) {
                0xFF_5555
            } else {
                INK
            };
            (format!("{label} {}", clock(e.duration)), ink)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_become_titles() {
        assert_eq!(title("minecraft:diamond_helmet"), "Diamond Helmet");
        assert_eq!(title("totem_of_undying"), "Totem Of Undying");
        assert_eq!(title(""), "");
    }

    #[test]
    fn the_clock_rounds_up_like_the_game_does() {
        assert_eq!(clock(1200), "1:00");
        assert_eq!(clock(1), "0:01");
        assert_eq!(clock(0), "0:00");
        assert_eq!(clock(72_000), "1:00:00");
        assert_eq!(clock(-1), "endless");
    }

    #[test]
    fn levels_are_roman() {
        assert_eq!(level(0), "");
        assert_eq!(level(1), "II");
        assert_eq!(level(4), "V");
        assert_eq!(level(11), "12");
        assert_eq!(level(-3), "");
    }

    #[test]
    fn lines_sort_colour_and_filter() {
        let e = |id: &str, duration, amplifier, ambient| Active {
            id: id.into(),
            duration,
            amplifier,
            ambient,
        };
        let list = [
            e("speed", 3600, 1, false),
            e("poison", 100, 0, false),
            e("haste", -1, 0, true),
        ];
        let out = lines(&list, true);
        assert_eq!(out[0], ("Poison 0:05".to_owned(), 0xFF_5555));
        assert_eq!(out[1], ("Speed II 3:00".to_owned(), INK));
        assert_eq!(out[2].0, "Haste endless");
        assert_eq!(lines(&list, false).len(), 2, "beacon effects hidden");
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
