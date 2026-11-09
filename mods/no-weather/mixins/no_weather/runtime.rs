// No Weather, a native Horizon mod: Keeps the sky clear in your own worlds: the game's rain and thunder never start. Servers keep their own weather.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_no_weather`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "no_weather";
pub const NAME: &str = "No Weather";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Keeps the sky clear in your own worlds: the game's rain and thunder never start. Servers keep their own weather.";

use std::cell::RefCell;

/// An option that cycles through choices: its stored key, label, the values it can take and the default one.
pub type Choice = (&'static str, &'static str, &'static [&'static str], usize);

/// The mod's settings, in the order of the settings page's buttons. The first is on or off.
pub const OPTIONS: &[Choice] = &[
    ("shown", "No weather", &["true", "false"], 0),
    ("what", "Stops", &["all", "thunder"], 0),
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
            c.next().map_or(String::new(), |f| {
                f.to_uppercase().collect::<String>() + c.as_str()
            })
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

/// Slash command that switches the weather stopper on or off.
pub const COMMAND: &str = "noweather";

/// Whether a weather change to `raining` and `thundering` is refused: `all` keeps every rain and
/// thunderstorm away, `thunder` only the thunder.
pub fn refuses(what: &str, raining: bool, thundering: bool) -> bool {
    match what {
        "thunder" => thundering,
        _ => raining || thundering,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_refuses_rain_and_thunder_but_not_clear_skies() {
        assert!(refuses("all", true, false));
        assert!(refuses("all", true, true));
        assert!(refuses("all", false, true));
        assert!(!refuses("all", false, false));
    }

    #[test]
    fn thunder_only_lets_the_rain_fall() {
        assert!(refuses("thunder", true, true));
        assert!(!refuses("thunder", true, false));
        assert!(!refuses("thunder", false, false));
    }

    #[test]
    fn the_defaults_stop_everything() {
        forget_options();
        assert!(is(|_: &str| None, "shown"));
        assert_eq!(value(|_: &str| None, "what"), "all");
        forget_options();
    }

    #[test]
    fn parse_picks_the_stored_choice_else_the_default() {
        assert_eq!(parse(Some("b".into()), &["a", "b"], 0), 1);
        assert_eq!(parse(Some("zz".into()), &["a", "b"], 0), 0);
    }

    #[test]
    fn a_click_moves_to_the_next_choice_and_wraps() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(click(none, 0), Some(("shown", "false".to_owned())));
        assert_eq!(click(none, 1), Some(("what", "thunder".to_owned())));
        assert_eq!(click(none, 1), Some(("what", "all".to_owned())));
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
}
