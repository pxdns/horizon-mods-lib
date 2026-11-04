// InvMove, a native Horizon mod: Keep walking, jumping and sneaking while your inventory is open.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_invmove`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "invmove";
pub const NAME: &str = "InvMove";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Keep walking, jumping and sneaking while your inventory is open.";

use std::cell::RefCell;

/// The mod's on/off settings: key (the stored file), label and default.
pub const OPTIONS: &[(&str, &str, bool)] = &[("jump", "Jump and sneak too", true)];

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
    /// True while the mod runs the tick again.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// Whether to run the tick with no screen open: the inventory is open, a movement key is held and this is
/// not the nested run. Marks the nested run when it answers yes.
pub fn enter(inventory_open: bool, moving: bool) -> bool {
    if INSIDE.with(Cell::get) || !inventory_open || !moving {
        return false;
    }
    INSIDE.with(|i| i.set(true));
    true
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_only_runs_with_the_inventory_open_and_a_key_held() {
        leave();
        assert!(!enter(false, true), "no inventory open");
        assert!(!enter(true, false), "nothing held");
        assert!(enter(true, true));
        leave();
    }

    #[test]
    fn the_nested_tick_runs_the_original() {
        leave();
        assert!(enter(true, true));
        assert!(!enter(true, true), "nested");
        leave();
        assert!(enter(true, true), "free again after leave");
        leave();
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
