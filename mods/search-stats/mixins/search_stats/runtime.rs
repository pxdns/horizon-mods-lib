// Search Stats, a native Horizon mod: Adds a search box to the statistics screen that filters the Items and Mobs lists as you type.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_search_stats`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "search_stats";
pub const NAME: &str = "Search Stats";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Adds a search box to the statistics screen that filters the Items and Mobs lists as you type.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[("box", "Show the search box", true)];

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
    static QUERY: RefCell<String> = const { RefCell::new(String::new()) };
    static SHOW: Cell<bool> = const { Cell::new(true) };
}

/// Longest search the box takes.
pub const MAX: usize = 40;

/// Starts a fresh search (the screen opened); `show` is the "Show the search box" setting.
pub fn reset(show: bool) {
    QUERY.with_borrow_mut(String::clear);
    SHOW.with(|s| s.set(show));
}

pub fn shown() -> bool {
    SHOW.with(Cell::get)
}

pub fn query() -> String {
    QUERY.with_borrow(Clone::clone)
}

/// The character a key name types, if any: `key.keyboard.a` -> `a`, `key.keyboard.space` -> ` `.
pub fn typed(key: &str) -> Option<char> {
    let name = key.strip_prefix("key.keyboard.")?;
    match name {
        "space" => Some(' '),
        "minus" => Some('-'),
        n if n.len() == 1
            && n.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) =>
        {
            n.chars().next()
        }
        _ => None,
    }
}

/// A key went down while the screen is open: edits the search. Returns whether it changed.
pub fn press(key: &str) -> bool {
    if !shown() {
        return false;
    }
    QUERY.with_borrow_mut(|q| match key {
        "key.keyboard.backspace" => q.pop().is_some(),
        "key.keyboard.delete" => {
            let had = !q.is_empty();
            q.clear();
            had
        }
        _ => match typed(key) {
            Some(c) if q.chars().count() < MAX => {
                q.push(c);
                true
            }
            _ => false,
        },
    })
}

/// `minecraft:diamond_pickaxe` as `diamond pickaxe`.
pub fn plain(id: &str) -> String {
    id.rsplit(':')
        .next()
        .unwrap_or(id)
        .replace('_', " ")
        .to_lowercase()
}

/// Whether `id` matches: every word of `query` is somewhere in its plain name. An empty query matches everything.
pub fn matches(query: &str, id: &str) -> bool {
    let name = plain(id);
    query
        .split_whitespace()
        .all(|word| name.contains(&word.to_lowercase()))
}

/// The ids that match the current search, in their order.
pub fn filter(ids: Vec<String>) -> Vec<String> {
    if !shown() {
        return ids;
    }
    let query = query();
    if query.trim().is_empty() {
        return ids;
    }
    ids.into_iter().filter(|id| matches(&query, id)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids() -> Vec<String> {
        [
            "minecraft:diamond_pickaxe",
            "minecraft:diamond",
            "minecraft:stone_pickaxe",
            "minecraft:cobblestone",
        ]
        .map(String::from)
        .to_vec()
    }

    fn type_text(text: &str) {
        for c in text.chars() {
            let key = if c == ' ' {
                "key.keyboard.space".to_owned()
            } else {
                format!("key.keyboard.{c}")
            };
            press(&key);
        }
    }

    #[test]
    fn keys_type_letters_digits_space_and_minus() {
        assert_eq!(typed("key.keyboard.a"), Some('a'));
        assert_eq!(typed("key.keyboard.7"), Some('7'));
        assert_eq!(typed("key.keyboard.space"), Some(' '));
        assert_eq!(typed("key.keyboard.minus"), Some('-'));
        assert_eq!(typed("key.keyboard.enter"), None);
        assert_eq!(typed("key.keyboard.left.shift"), None);
        assert_eq!(typed("key.mouse.left"), None);
    }

    #[test]
    fn names_are_matched_word_by_word() {
        assert!(matches("pick", "minecraft:diamond_pickaxe"));
        assert!(matches("diamond pick", "minecraft:diamond_pickaxe"));
        assert!(
            matches("PICKAXE diam", "minecraft:diamond_pickaxe"),
            "any order, any case"
        );
        assert!(!matches("diamond pick", "minecraft:stone_pickaxe"));
        assert!(matches("", "minecraft:stone"));
        assert!(matches("   ", "minecraft:stone"));
        assert!(
            !matches("minecraft", "minecraft:stone"),
            "the namespace is not searched"
        );
    }

    #[test]
    fn typing_filters_and_backspace_widens() {
        reset(true);
        assert_eq!(filter(ids()).len(), 4);
        type_text("pickaxe");
        assert_eq!(
            filter(ids()),
            ["minecraft:diamond_pickaxe", "minecraft:stone_pickaxe"]
        );
        type_text(" st");
        assert_eq!(filter(ids()), ["minecraft:stone_pickaxe"]);
        press("key.keyboard.backspace");
        press("key.keyboard.backspace");
        press("key.keyboard.backspace");
        assert_eq!(query(), "pickaxe");
        assert!(press("key.keyboard.delete"));
        assert_eq!(query(), "");
        assert!(!press("key.keyboard.delete"), "nothing to clear");
        assert_eq!(filter(ids()).len(), 4);
    }

    #[test]
    fn the_search_is_capped_and_can_be_switched_off() {
        reset(true);
        for _ in 0..(MAX + 10) {
            press("key.keyboard.a");
        }
        assert_eq!(query().len(), MAX);
        reset(false);
        assert!(!press("key.keyboard.a"));
        assert_eq!(filter(ids()).len(), 4, "off means no filtering");
        reset(true);
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
