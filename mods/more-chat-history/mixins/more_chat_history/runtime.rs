// More Chat History, a native Horizon mod: Chat remembers up to 2000 lines instead of 100, so you can scroll back through a long conversation.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_more_chat_history`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "more_chat_history";
pub const NAME: &str = "More Chat History";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Chat remembers up to 2000 lines instead of 100, so you can scroll back through a long conversation.";

use std::cell::Cell;

/// The line limits the first button cycles through.
pub const LEVELS: [usize; 5] = [100, 250, 500, 1000, 2000];
/// Horizon's own limit, and the default here.
pub const DEFAULT: usize = 1000;

thread_local! {
    static LIMIT: Cell<usize> = const { Cell::new(DEFAULT) };
}

/// How many messages and wrapped lines chat keeps.
pub fn limit() -> usize {
    LIMIT.with(Cell::get)
}

/// Sets the limit from the stored value (the game tells the chat each tick).
pub fn configure(limit: usize) {
    LIMIT.with(|l| l.set(limit));
}

/// The stored `limit`, one of `LEVELS`, else the default.
pub fn read(get: impl Fn(&str) -> Option<String>) -> usize {
    get("limit")
        .and_then(|v| v.trim().parse().ok())
        .filter(|v| LEVELS.contains(v))
        .unwrap_or(DEFAULT)
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    vec![format!("History: {} lines", read(get))]
}

/// A click on the settings page: what to store.
pub fn click(get: impl Fn(&str) -> Option<String>, i: usize) -> Option<(&'static str, String)> {
    if i != 0 {
        return None;
    }
    let current = read(get);
    let at = LEVELS.iter().position(|&l| l == current).unwrap_or(0);
    let next = LEVELS[(at + 1) % LEVELS.len()];
    configure(next);
    Some(("limit", next.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn the_default_is_a_thousand_lines() {
        assert_eq!(read(none), 1000);
        assert_eq!(labels(none), ["History: 1000 lines"]);
    }

    #[test]
    fn the_button_cycles_through_the_levels_and_sets_the_limit() {
        let stored = |k: &str| (k == "limit").then(|| "2000".to_owned());
        let (key, value) = click(stored, 0).unwrap();
        assert_eq!((key, value.as_str()), ("limit", "100"), "2000 wraps to 100");
        assert_eq!(limit(), 100);
        let (_, next) = click(none, 0).unwrap();
        assert_eq!(next, "2000", "from the default 1000 the next is 2000");
        assert_eq!(limit(), 2000);
        assert!(click(none, 3).is_none());
        configure(DEFAULT);
    }

    #[test]
    fn stored_junk_falls_back() {
        assert_eq!(read(|_| Some("7".to_owned())), DEFAULT);
        assert_eq!(read(|_| Some("banana".to_owned())), DEFAULT);
        assert_eq!(read(|_| Some(" 500\n".to_owned())), 500);
    }
}
