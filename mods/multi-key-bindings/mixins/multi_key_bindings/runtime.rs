// Multi Key Bindings, a native Horizon mod: More than one key for the same action.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_multi_key_bindings`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "multi_key_bindings";
pub const NAME: &str = "Multi Key Bindings";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "More than one key for the same action.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[
    ("say", "Say in chat when an extra key changes", true)
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

/// Slash command.
pub const COMMAND: &str = "multikey";

thread_local! {
    /// (action, key name) pairs.
    static EXTRAS: StateCell<Vec<(String, String)>> = const { StateCell::new(Vec::new()) };
    static LOADED: Cell<bool> = const { Cell::new(false) };
}

pub fn loaded() -> bool {
    LOADED.with(Cell::get)
}

/// The stored extras, `key.forward=key.keyboard.up` per line.
pub fn decode(text: &str) -> Vec<(String, String)> {
    text.lines().filter_map(|l| l.split_once('=')).map(|(a, k)| (a.trim().to_owned(), k.trim().to_owned())).filter(|(a, k)| !a.is_empty() && !k.is_empty()).collect()
}

pub fn encode(extras: &[(String, String)]) -> String {
    extras.iter().map(|(a, k)| format!("{a}={k}")).collect::<Vec<_>>().join("\n")
}

/// Loads the stored extras (once) from the setting's text.
pub fn load(text: Option<String>) {
    if !LOADED.with(|l| l.replace(true)) {
        EXTRAS.with_borrow_mut(|e| *e = decode(&text.unwrap_or_default()));
    }
}

pub fn extras() -> Vec<(String, String)> {
    EXTRAS.with_borrow(Clone::clone)
}

/// The actions an extra key drives, by their vanilla mapping names.
pub fn actions_for(key_name: &str) -> Vec<String> {
    EXTRAS.with_borrow(|e| e.iter().filter(|(_, k)| k == key_name).map(|(a, _)| a.clone()).collect())
}

/// What the player typed as a key (`up`, `space`, `mouse.4`, `key.keyboard.up`) as its vanilla name.
pub fn key_full(typed: &str) -> Option<String> {
    let t = typed.to_lowercase();
    let ok = !t.is_empty() && t.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_');
    if !ok {
        return None;
    }
    Some(if t.starts_with("key.") {
        t
    } else if let Some(button) = t.strip_prefix("mouse.") {
        format!("key.mouse.{button}")
    } else {
        format!("key.keyboard.{t}")
    })
}

/// The mapping name for what the player typed (`forward`, `key.forward`), among `actions`.
pub fn action_full(typed: &str, actions: &[&str]) -> Option<String> {
    let t = typed.to_lowercase();
    let full = if t.starts_with("key.") { t } else { format!("key.{t}") };
    actions.iter().find(|a| a.to_lowercase() == full).map(|a| (*a).to_owned())
}

/// Runs the arguments after `/multikey`: the chat lines, and the text to store when the extras changed.
pub fn run(args: &[&str], actions: &[&str]) -> (Vec<String>, Option<String>) {
    match args {
        ["add", action, key] => {
            let Some(a) = action_full(action, actions) else {
                return (vec![format!("Unknown action `{action}`")], None);
            };
            let Some(k) = key_full(key) else {
                return (vec![format!("Not a key name: `{key}`")], None);
            };
            let text = EXTRAS.with_borrow_mut(|e| {
                if !e.contains(&(a.clone(), k.clone())) {
                    e.push((a.clone(), k.clone()));
                }
                encode(e)
            });
            (vec![format!("{} now also on {}", short(&a), short_key(&k))], Some(text))
        }
        ["remove", action, rest @ ..] => {
            let Some(a) = action_full(action, actions) else {
                return (vec![format!("Unknown action `{action}`")], None);
            };
            let key = rest.first().and_then(|k| key_full(k));
            let (removed, text) = EXTRAS.with_borrow_mut(|e| {
                let before = e.len();
                e.retain(|(x, k)| !(*x == a && key.as_ref().is_none_or(|key| key == k)));
                (before - e.len(), encode(e))
            });
            if removed == 0 { (vec![format!("{} has no extra key", short(&a))], None) } else { (vec![format!("Removed {removed} extra key(s) of {}", short(&a))], Some(text)) }
        }
        ["list"] => {
            let e = extras();
            if e.is_empty() {
                (vec!["No extra keys. /multikey add <action> <key>".to_owned()], None)
            } else {
                (e.iter().map(|(a, k)| format!("{}: {}", short(a), short_key(k))).collect(), None)
            }
        }
        _ => (vec!["Usage: /multikey add <action> <key> | remove <action> [key] | list".to_owned()], None),
    }
}

fn short(action: &str) -> String {
    action.strip_prefix("key.").unwrap_or(action).replace('.', " ")
}

fn short_key(key: &str) -> String {
    key.strip_prefix("key.keyboard.").or_else(|| key.strip_prefix("key.")).unwrap_or(key).replace('.', " ")
}

/// The mod is off: the extra keys do nothing, and are read again when it is turned on.
pub fn forget_all() {
    EXTRAS.with_borrow_mut(Vec::clear);
    LOADED.with(|l| l.set(false));
}

/// Forgets everything (tests).
#[cfg(test)]
pub fn forget() {
    EXTRAS.with_borrow_mut(Vec::clear);
    LOADED.with(|l| l.set(true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const ACTIONS: [&str; 4] = ["key.forward", "key.jump", "key.hotbar.1", "key.inventory"];

    #[test]
    fn typed_names_become_vanilla_names() {
        assert_eq!(key_full("Up"), Some("key.keyboard.up".to_owned()));
        assert_eq!(key_full("left.shift"), Some("key.keyboard.left.shift".to_owned()));
        assert_eq!(key_full("mouse.4"), Some("key.mouse.4".to_owned()));
        assert_eq!(key_full("key.keyboard.f9"), Some("key.keyboard.f9".to_owned()));
        assert_eq!(key_full(""), None);
        assert_eq!(key_full("a b"), None);
        assert_eq!(action_full("Forward", &ACTIONS), Some("key.forward".to_owned()));
        assert_eq!(action_full("hotbar.1", &ACTIONS), Some("key.hotbar.1".to_owned()));
        assert_eq!(action_full("nothing", &ACTIONS), None);
    }

    #[test]
    fn adding_and_removing_extra_keys() {
        forget();
        let (lines, save) = run(&["add", "forward", "up"], &ACTIONS);
        assert_eq!(lines, ["forward now also on up"]);
        assert_eq!(save.as_deref(), Some("key.forward=key.keyboard.up"));
        assert_eq!(actions_for("key.keyboard.up"), ["key.forward"]);
        run(&["add", "forward", "up"], &ACTIONS);
        run(&["add", "jump", "up"], &ACTIONS);
        assert_eq!(actions_for("key.keyboard.up"), ["key.forward", "key.jump"], "no duplicates, and a key can drive two actions");
        let (lines, save) = run(&["remove", "forward", "up"], &ACTIONS);
        assert_eq!(lines, ["Removed 1 extra key(s) of forward"]);
        assert_eq!(save.as_deref(), Some("key.jump=key.keyboard.up"));
        let (lines, save) = run(&["remove", "inventory"], &ACTIONS);
        assert_eq!((lines, save), (vec!["inventory has no extra key".to_owned()], None));
        let (_, save) = run(&["remove", "jump"], &ACTIONS);
        assert_eq!(save.as_deref(), Some(""), "all removed");
        assert!(actions_for("key.keyboard.up").is_empty());
    }

    #[test]
    fn bad_input_changes_nothing() {
        forget();
        assert_eq!(run(&["add", "nonsense", "up"], &ACTIONS).0, ["Unknown action `nonsense`"]);
        assert_eq!(run(&["add", "forward", "a b"], &ACTIONS).0, ["Not a key name: `a b`"]);
        assert!(run(&["add", "forward"], &ACTIONS).0[0].starts_with("Usage"));
        assert!(extras().is_empty());
    }

    #[test]
    fn the_list_and_the_stored_text_round_trip() {
        forget();
        run(&["add", "forward", "up"], &ACTIONS);
        run(&["add", "inventory", "mouse.4"], &ACTIONS);
        assert_eq!(run(&["list"], &ACTIONS).0, ["forward: up", "inventory: mouse 4"]);
        let text = encode(&extras());
        assert_eq!(decode(&text), extras());
        assert_eq!(decode("garbage\n=x\nkey.a=\nkey.b=key.keyboard.k"), vec![("key.b".to_owned(), "key.keyboard.k".to_owned())]);
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
