// Chat Heads, a native Horizon mod: Shows the sender's face in front of each chat line.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_chat_heads`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "chat_heads";
pub const NAME: &str = "Chat Heads";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Shows the sender's face in front of each chat line.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("hat", "Show the hat layer", true),
    ("others", "Also find senders in Name: and Name » forms", true)
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

use std::cell::{Cell, RefCell as StateCell};
use std::collections::BTreeSet;

/// How far into a line the sender's name may be.
pub const REACH: usize = 48;

thread_local! {
    static HAT: Cell<bool> = const { Cell::new(true) };
    static OTHERS: Cell<bool> = const { Cell::new(true) };
    static ENABLED: Cell<bool> = const { Cell::new(true) };
    /// The names of the players on the server, lower case.
    static ONLINE: StateCell<BTreeSet<String>> = const { StateCell::new(BTreeSet::new()) };
    /// True while the mod adds a message again with its head.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// Sets what the mod does (the game tells the chat each tick).
pub fn configure(enabled: bool, hat: bool, others: bool) {
    ENABLED.with(|v| v.set(enabled));
    HAT.with(|v| v.set(hat));
    OTHERS.with(|v| v.set(others));
}

/// The names on the server (told once a second), lower case.
pub fn set_online(online: BTreeSet<String>) {
    ONLINE.with_borrow_mut(|o| *o = online);
}

pub fn enabled() -> bool {
    ENABLED.with(Cell::get)
}

pub fn hat() -> bool {
    HAT.with(Cell::get)
}

/// Marks the nested call; false when this is the nested one.
pub fn enter() -> bool {
    if INSIDE.with(Cell::get) {
        return false;
    }
    INSIDE.with(|i| i.set(true));
    true
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

/// A name a player can have: 1 to 16 letters, digits or underscores.
pub fn valid(name: &str) -> bool {
    (1..=16).contains(&name.len()) && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// The sender of a chat line: `<Name> hi`, anywhere in the first characters (after a prefix such as `[VIP]`), or, when the
/// player is online, `Name: hi` or `Name » hi`. The name as written.
pub fn sender(line: &str) -> Option<String> {
    let head: String = line.chars().take(REACH).collect();
    if let Some(open) = head.find('<')
        && let Some(close) = head[open + 1..].find('>')
    {
        let name = &head[open + 1..open + 1 + close];
        if valid(name) {
            return Some(name.to_owned());
        }
    }
    if !OTHERS.with(Cell::get) {
        return None;
    }
    // `[VIP] Name: hi`, `Name » hi`: the last word before the separator.
    let separator = head.find(": ").or_else(|| head.find(" \u{bb} "))?;
    let name = head[..separator].rsplit([' ', ']']).next()?;
    (valid(name) && ONLINE.with_borrow(|o| o.contains(&name.to_lowercase()))).then(|| name.to_owned())
}

/// The text component Horizon draws as a head.
pub fn head_json(name: &str, hat: bool) -> String {
    format!("{{\"object\":\"player\",\"player\":{{\"name\":\"{name}\"}},\"hat\":{hat}}}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn online(names: &[&str]) {
        configure(true, true, true);
        set_online(names.iter().map(|n| n.to_lowercase()).collect());
    }

    #[test]
    fn the_usual_form_is_found() {
        online(&[]);
        assert_eq!(sender("<Steve> hello"), Some("Steve".to_owned()));
        assert_eq!(sender("[VIP] <Alex_99> hi there"), Some("Alex_99".to_owned()));
        assert_eq!(sender("<bad name> hi"), None);
        assert_eq!(sender("<> hi"), None);
        assert_eq!(sender("Server restarting in 5 minutes"), None);
    }

    #[test]
    fn other_forms_need_the_player_to_be_online() {
        online(&["steve"]);
        assert_eq!(sender("Steve: hello"), Some("Steve".to_owned()));
        assert_eq!(sender("[Mod] Steve \u{bb} hello"), Some("Steve".to_owned()));
        assert_eq!(sender("Notch: hello"), None, "not online");
        configure(true, true, false);
        assert_eq!(sender("Steve: hello"), None, "switched off");
        assert_eq!(sender("<Steve> hello"), Some("Steve".to_owned()), "the usual form always counts");
        online(&[]);
    }

    #[test]
    fn a_name_far_into_a_line_is_not_a_sender() {
        online(&[]);
        let line = format!("{}<Steve> hi", "x".repeat(REACH));
        assert_eq!(sender(&line), None);
    }

    #[test]
    fn names_are_valid_minecraft_names() {
        assert!(valid("Steve") && valid("a_b9") && valid("1234567890123456"));
        assert!(!valid("") && !valid("12345678901234567") && !valid("a b") && !valid("a-b"));
    }

    #[test]
    fn the_head_component_is_json() {
        let json = head_json("Steve", false);
        assert_eq!(json, r#"{"object":"player","player":{"name":"Steve"},"hat":false}"#);
    }

    #[test]
    fn the_nested_call_is_not_decorated_again() {
        leave();
        assert!(enter());
        assert!(!enter());
        leave();
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
