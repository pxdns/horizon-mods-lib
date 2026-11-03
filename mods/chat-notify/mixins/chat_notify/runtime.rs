// Chat Notify, a native Horizon mod: Plays a sound when someone says your name in chat.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_chat_notify`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "chat_notify";
pub const NAME: &str = "Chat Notify";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Plays a sound when someone says your name in chat.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("players", "Player messages", true),
    ("server", "Server messages", false),
    ("own", "My own messages", false),
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

use std::cell::Cell;

thread_local! {
    static PLAYER: RefCell<String> = const { RefCell::new(String::new()) };
    static PENDING: Cell<bool> = const { Cell::new(false) };
    /// Which sources count: players, the server, the player's own messages.
    static WANT: Cell<[bool; 3]> = const { Cell::new([true, false, false]) };
}

/// The player's name, set each tick.
pub fn set_name(name: &str) {
    PLAYER.with_borrow_mut(|n| {
        if n != name {
            name.clone_into(n);
        }
    });
}

/// Which messages count, set each tick.
pub fn configure(players: bool, server: bool, own: bool) {
    WANT.with(|w| w.set([players, server, own]));
}

/// Whether `name` appears in `text` as a word (case does not matter).
pub fn mentions(text: &str, name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let (t, n) = (text.to_lowercase(), name.to_lowercase());
    let word = |c: Option<char>| c.is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
    t.match_indices(&n)
        .any(|(at, _)| word(t[..at].chars().next_back()) && word(t[at + n.len()..].chars().next()))
}

/// Whether the message is the player's own, in the usual `<name> text` form.
pub fn is_own(text: &str, name: &str) -> bool {
    text.to_lowercase()
        .starts_with(&format!("<{}>", name.to_lowercase()))
}

/// A chat line arrived: remember to ping when it counts. `player` and `server` say where it came from.
pub fn message(text: &str, player: bool, server: bool) {
    let [players, servers, own] = WANT.with(Cell::get);
    let allowed = (player && players) || (server && servers);
    if !allowed {
        return;
    }
    let hit = PLAYER.with_borrow(|name| mentions(text, name) && (own || !is_own(text, name)));
    if hit {
        PENDING.with(|p| p.set(true));
    }
}

/// Whether a ping is due, and clears it.
pub fn take() -> bool {
    PENDING.with(|p| p.replace(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_match_as_words_in_any_case() {
        assert!(mentions("hey Steve, look", "steve"));
        assert!(mentions("STEVE!", "Steve"));
        assert!(mentions("<Alex> steve", "Steve"));
        assert!(
            !mentions("steven is here", "steve"),
            "a longer word is not a mention"
        );
        assert!(!mentions("nobody", "steve"));
        assert!(!mentions("anything", ""));
        assert!(
            !mentions("ping steve_", "steve"),
            "an underscore continues the word"
        );
    }

    #[test]
    fn own_messages_are_recognised() {
        assert!(is_own("<Steve> hello", "steve"));
        assert!(!is_own("<Alex> hello steve", "steve"));
    }

    #[test]
    fn pings_follow_the_sources_and_the_own_option() {
        set_name("Steve");
        configure(true, false, false);
        take();
        message("<Alex> hi steve", true, false);
        assert!(take());
        message("<Steve> talking to myself, steve", true, false);
        assert!(!take(), "own messages are skipped");
        configure(true, false, true);
        message("<Steve> talking to myself, steve", true, false);
        assert!(take());
        configure(true, false, false);
        message("steve joined", false, true);
        assert!(!take(), "server messages are off");
        configure(true, true, false);
        message("steve joined", false, true);
        assert!(take());
        assert!(!take(), "a ping fires once");
        configure(true, false, false);
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
