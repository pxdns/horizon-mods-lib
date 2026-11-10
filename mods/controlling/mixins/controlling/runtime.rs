// Controlling, a native Horizon mod: Find key bindings by action or by key, and list conflicts and unbound actions.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_controlling`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "controlling";
pub const NAME: &str = "Controlling";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Find key bindings by action or by key, and list conflicts and unbound actions.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] =
    &[("hide_none", "Leave out unbound actions from find", false)];

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

use std::collections::BTreeMap;

/// Slash command.
pub const COMMAND: &str = "keybinds";
pub const HELP: [&str; 4] = [
    "/keybinds find <words>: bindings with all the words in the action or the key",
    "/keybinds conflicts: keys used by more than one action",
    "/keybinds unbound: actions with no key",
    "Action and key are shown as in the controls screen.",
];

/// What the player asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Request {
    Help,
    Find(Vec<String>),
    Conflicts,
    Unbound,
}

pub fn request(args: &[&str]) -> Request {
    match args {
        ["find", words @ ..] if !words.is_empty() => {
            Request::Find(words.iter().map(|w| w.to_lowercase()).collect())
        }
        ["conflicts"] => Request::Conflicts,
        ["unbound"] => Request::Unbound,
        _ => Request::Help,
    }
}

/// `key.forward` as `forward`, `key.hotbar.3` as `hotbar 3`.
pub fn action_name(action: &str) -> String {
    action
        .strip_prefix("key.")
        .unwrap_or(action)
        .replace('.', " ")
}

/// `key.keyboard.left.shift` as `left shift`, `key.mouse.left` as `mouse left`, `key.keyboard.unknown` as none.
pub fn key_name(key: &str) -> Option<String> {
    if key == "key.keyboard.unknown" || key.is_empty() {
        return None;
    }
    let bare = key
        .strip_prefix("key.keyboard.")
        .map(str::to_owned)
        .or_else(|| key.strip_prefix("key.").map(str::to_owned))
        .unwrap_or_else(|| key.to_owned());
    Some(bare.replace('.', " "))
}

fn line(action: &str, key: &str) -> String {
    format!(
        "{}: {}",
        action_name(action),
        key_name(key).unwrap_or_else(|| "none".to_owned())
    )
}

/// The chat lines answering a request over (action, bound key) pairs.
pub fn answer(request: &Request, bindings: &[(String, String)]) -> Vec<String> {
    match request {
        Request::Help => HELP.map(String::from).to_vec(),
        Request::Find(words) => {
            let found: Vec<String> = bindings
                .iter()
                .filter(|(action, key)| {
                    let hay = format!(
                        "{} {}",
                        action_name(action),
                        key_name(key).unwrap_or_default()
                    )
                    .to_lowercase();
                    words.iter().all(|w| hay.contains(w.as_str()))
                })
                .map(|(action, key)| line(action, key))
                .collect();
            if found.is_empty() {
                vec![format!("No binding matches `{}`", words.join(" "))]
            } else {
                let mut out = vec![format!("{} bindings:", found.len())];
                out.extend(found.into_iter().take(25));
                out
            }
        }
        Request::Conflicts => {
            let mut by_key: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
            for (action, key) in bindings {
                if key_name(key).is_some() {
                    by_key.entry(key).or_default().push(action);
                }
            }
            let conflicts: Vec<String> = by_key
                .into_iter()
                .filter(|(_, actions)| actions.len() > 1)
                .map(|(key, actions)| {
                    format!(
                        "{}: {}",
                        key_name(key).unwrap_or_default(),
                        actions
                            .iter()
                            .map(|a| action_name(a))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })
                .collect();
            if conflicts.is_empty() {
                vec!["No key is used by two actions".to_owned()]
            } else {
                std::iter::once(format!("{} conflicts:", conflicts.len()))
                    .chain(conflicts)
                    .collect()
            }
        }
        Request::Unbound => {
            let none: Vec<String> = bindings
                .iter()
                .filter(|(_, key)| key_name(key).is_none())
                .map(|(a, _)| action_name(a))
                .collect();
            if none.is_empty() {
                vec!["Every action has a key".to_owned()]
            } else {
                vec![format!("Unbound: {}", none.join(", "))]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binds() -> Vec<(String, String)> {
        [
            ("key.forward", "key.keyboard.w"),
            ("key.back", "key.keyboard.s"),
            ("key.sneak", "key.keyboard.left.shift"),
            ("key.sprint", "key.keyboard.left.shift"),
            ("key.attack", "key.mouse.left"),
            ("key.advancements", "key.keyboard.unknown"),
        ]
        .map(|(a, k)| (a.to_owned(), k.to_owned()))
        .to_vec()
    }

    #[test]
    fn requests_parse() {
        assert_eq!(request(&[]), Request::Help);
        assert_eq!(
            request(&["find", "Shift"]),
            Request::Find(vec!["shift".into()])
        );
        assert_eq!(request(&["find"]), Request::Help);
        assert_eq!(request(&["conflicts"]), Request::Conflicts);
        assert_eq!(request(&["unbound"]), Request::Unbound);
    }

    #[test]
    fn names_are_readable() {
        assert_eq!(action_name("key.hotbar.3"), "hotbar 3");
        assert_eq!(
            key_name("key.keyboard.left.shift"),
            Some("left shift".to_owned())
        );
        assert_eq!(key_name("key.mouse.left"), Some("mouse left".to_owned()));
        assert_eq!(key_name("key.keyboard.unknown"), None);
    }

    #[test]
    fn find_searches_the_action_and_the_key() {
        let by_action = answer(&Request::Find(vec!["sneak".into()]), &binds());
        assert_eq!(by_action, ["1 bindings:", "sneak: left shift"]);
        let by_key = answer(&Request::Find(vec!["shift".into()]), &binds());
        assert_eq!(
            by_key,
            ["2 bindings:", "sneak: left shift", "sprint: left shift"]
        );
        let both = answer(
            &Request::Find(vec!["mouse".into(), "attack".into()]),
            &binds(),
        );
        assert_eq!(both, ["1 bindings:", "attack: mouse left"]);
        assert_eq!(
            answer(&Request::Find(vec!["zebra".into()]), &binds()),
            ["No binding matches `zebra`"]
        );
    }

    #[test]
    fn conflicts_and_unbound() {
        assert_eq!(
            answer(&Request::Conflicts, &binds()),
            ["1 conflicts:", "left shift: sneak, sprint"]
        );
        assert_eq!(
            answer(&Request::Conflicts, &binds()[..2]),
            ["No key is used by two actions"]
        );
        assert_eq!(
            answer(&Request::Unbound, &binds()),
            ["Unbound: advancements"]
        );
        assert_eq!(
            answer(&Request::Unbound, &binds()[..2]),
            ["Every action has a key"]
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
