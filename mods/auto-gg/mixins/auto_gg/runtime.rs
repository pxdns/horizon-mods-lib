// Auto GG, a native Horizon mod: A key you press to send a good-game message in chat: pick the message. It only sends when you press the key, with a pause between sends.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_auto_gg`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "auto_gg";
pub const NAME: &str = "Auto GG";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "A key you press to send a good-game message in chat: pick the message. It only sends when you press the key, with a pause between sends.";

use std::cell::RefCell;

/// An option that cycles through choices: its stored key, label, the values it can take and the default one.
pub type Choice = (&'static str, &'static str, &'static [&'static str], usize);

/// The mod's settings, in the order of the settings page's buttons. The first is on or off.
pub const OPTIONS: &[Choice] = &[
    ("shown", "Auto GG", &["true", "false"], 0),
    ("message", "Message", &["gg", "GG", "gg wp", "good game", "gf", "ggwp"], 0),
    ("pause", "Pause between sends", &["2", "5", "10", "30"], 1),
];

/// The key that sends the message, as the player can change it in Options > Controls.
pub const KEY: &str = "send";
pub const KEY_DEFAULT: &str = "key.keyboard.g";

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

/// Slash command that switches the key on or off.
pub const COMMAND: &str = "autogg";

thread_local! {
    /// The tick of the last send, to keep the pause.
    static LAST: RefCell<Option<u64>> = const { RefCell::new(None) };
}

/// The message to send at `now` (game ticks) with a `pause` of seconds since the last send, else none. A send is
/// recorded; a refused one is not.
pub fn send(message: &str, pause: u32, now: u64) -> Option<String> {
    LAST.with_borrow_mut(|last| {
        if let Some(t) = *last
            && now >= t
            && now - t < u64::from(pause) * 20
        {
            return None;
        }
        *last = Some(now);
        Some(message.to_owned())
    })
}

/// Forgets the last send (tests only).
#[cfg(test)]
pub fn forget_last() {
    LAST.with_borrow_mut(|l| *l = None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_press_sends_and_a_quick_second_does_not() {
        forget_last();
        assert_eq!(send("gg", 5, 100).as_deref(), Some("gg"));
        assert_eq!(send("gg", 5, 150), None, "2.5 seconds later is inside the pause");
        assert_eq!(send("gg", 5, 200).as_deref(), Some("gg"), "5 seconds later is outside it");
    }

    #[test]
    fn a_refused_press_does_not_restart_the_pause() {
        forget_last();
        assert!(send("gg", 2, 0).is_some());
        assert!(send("gg", 2, 30).is_none());
        assert!(send("gg", 2, 40).is_some(), "40 ticks after the send, not after the refused press");
    }

    #[test]
    fn a_new_world_with_earlier_ticks_is_not_locked_out() {
        forget_last();
        assert!(send("gg", 30, 5000).is_some());
        assert!(send("gg", 30, 10).is_some(), "ticks that went backwards mean a new session");
    }

    #[test]
    fn options_default_then_follow_the_stored_files() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(labels(none).len(), OPTIONS.len(), "one button per option");
        assert_eq!(labels(none)[0], "Auto GG: ON");
        assert_eq!(value(none, "message"), "gg");
        forget_options();
        let stored = |k: &str| (k == "message").then(|| "good game".to_owned());
        assert_eq!(value(stored, "message"), "good game");
        forget_options();
    }

    #[test]
    fn a_click_moves_to_the_next_choice_and_wraps() {
        forget_options();
        let none = |_: &str| None;
        assert_eq!(click(none, 0), Some(("shown", "false".to_owned())));
        assert_eq!(click(none, 1), Some(("message", "GG".to_owned())));
        assert!(click(none, OPTIONS.len()).is_none());
        forget_options();
    }
}
