// Held Item, a native Horizon mod: Move, tilt and resize the item and hand you hold in first person: offsets, tilt and size, for either or both hands.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_held_item`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "held_item";
pub const NAME: &str = "Held Item";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Move, tilt and resize the item and hand you hold in first person: offsets, tilt and size, for either or both hands.";

use std::cell::RefCell;

/// An option that cycles through choices: its stored key, label, the values it can take and the default one.
pub type Choice = (&'static str, &'static str, &'static [&'static str], usize);

/// The mod's settings, in the order of the settings page's buttons. The first is on or off.
pub const OPTIONS: &[Choice] = &[
    ("shown", "Held item", &["true", "false"], 0),
    ("hands", "Hands", &["both", "main", "off"], 0),
    (
        "x",
        "Right (blocks)",
        &["-0.2", "-0.1", "0", "0.1", "0.2", "0.3"],
        2,
    ),
    (
        "y",
        "Up (blocks)",
        &["-0.3", "-0.2", "-0.1", "0", "0.1", "0.2"],
        3,
    ),
    (
        "z",
        "Forward (blocks)",
        &["-0.3", "-0.2", "-0.1", "0", "0.1", "0.2"],
        3,
    ),
    (
        "tilt",
        "Tilt (degrees)",
        &["-30", "-15", "0", "15", "30"],
        2,
    ),
    (
        "scale",
        "Size",
        &["0.5", "0.75", "0.9", "1", "1.1", "1.25", "1.5"],
        3,
    ),
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

/// Slash command that switches the held-item changes on or off.
pub const COMMAND: &str = "helditem";

/// What to add to a hand's pose.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tilt: f32,
    pub scale: f32,
}

fn number(get: impl Fn(&str) -> Option<String>, key: &str, default: f32) -> f32 {
    value(get, key).parse().unwrap_or(default)
}

/// The pose to add for a hand (`main` or the other one), or none when the mod is off or leaves that hand alone.
pub fn pose(get: impl Fn(&str) -> Option<String> + Copy, main: bool) -> Option<Pose> {
    if !is(get, "shown") {
        return None;
    }
    match (value(get, "hands"), main) {
        ("main", false) | ("off", true) => return None,
        _ => {}
    }
    let p = Pose {
        x: number(get, "x", 0.0),
        y: number(get, "y", 0.0),
        z: number(get, "z", 0.0),
        tilt: number(get, "tilt", 0.0),
        scale: number(get, "scale", 1.0),
    };
    // The off hand is the main hand's mirror: its sideways offset and tilt turn round.
    Some(if main {
        p
    } else {
        Pose {
            x: -p.x,
            tilt: -p.tilt,
            ..p
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defaults_change_nothing() {
        forget_options();
        let p = pose(|_: &str| None, true).unwrap();
        assert_eq!(
            p,
            Pose {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                tilt: 0.0,
                scale: 1.0
            }
        );
        forget_options();
    }

    #[test]
    fn the_off_hand_mirrors_sideways_and_tilt() {
        forget_options();
        let get = |k: &str| match k {
            "x" => Some("0.2".to_owned()),
            "tilt" => Some("15".to_owned()),
            "scale" => Some("1.25".to_owned()),
            _ => None,
        };
        let main = pose(get, true).unwrap();
        let off = pose(get, false).unwrap();
        assert_eq!((main.x, main.tilt, main.scale), (0.2, 15.0, 1.25));
        assert_eq!((off.x, off.tilt, off.scale), (-0.2, -15.0, 1.25));
        forget_options();
    }

    #[test]
    fn hands_picks_which_hand_changes() {
        forget_options();
        let only = |h: &'static str| move |k: &str| (k == "hands").then(|| h.to_owned());
        assert!(pose(only("main"), true).is_some() && pose(only("main"), false).is_none());
        forget_options();
        assert!(pose(only("off"), true).is_none() && pose(only("off"), false).is_some());
        forget_options();
    }

    #[test]
    fn off_changes_nothing() {
        forget_options();
        assert!(pose(|k: &str| (k == "shown").then(|| "false".to_owned()), true).is_none());
        forget_options();
    }

    #[test]
    fn parse_picks_the_stored_choice_else_the_default() {
        assert_eq!(parse(Some("b".into()), &["a", "b"], 0), 1);
        assert_eq!(parse(Some("zz".into()), &["a", "b"], 0), 0);
        assert_eq!(parse(None, &["a", "b"], 1), 1);
    }

    #[test]
    fn a_click_moves_to_the_next_choice_and_wraps() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(click(none, 0), Some(("shown", "false".to_owned())));
        assert_eq!(click(none, 0), Some(("shown", "true".to_owned())));
        assert_eq!(click(none, 1), Some(("hands", "main".to_owned())));
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
}
