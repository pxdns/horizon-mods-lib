// Combat Hitboxes, a native Horizon mod: Makes entities slightly easier to hit by growing the box your crosshair is tested against.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_combat_hitboxes`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "combat_hitboxes";
pub const NAME: &str = "Combat Hitboxes";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Makes entities slightly easier to hit by growing the box your crosshair is tested against.";

use std::cell::Cell;

/// How much the boxes grow, in hundredths of a block: the first button cycles through these.
pub const LEVELS: [u32; 5] = [0, 5, 10, 20, 30];
/// The default, a tenth of a block.
pub const DEFAULT: u32 = 10;

thread_local! {
    static GROW: Cell<u32> = const { Cell::new(DEFAULT) };
    /// True while the mod runs the test again on the larger box.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
}

/// Sets the growth from the stored value (the game tells the mod each frame).
pub fn configure(hundredths: u32) {
    GROW.with(|g| g.set(hundredths));
}

/// The stored `grow`, one of `LEVELS`, else the default.
pub fn read(get: impl Fn(&str) -> Option<String>) -> u32 {
    get("grow")
        .and_then(|v| v.trim().parse().ok())
        .filter(|v| LEVELS.contains(v))
        .unwrap_or(DEFAULT)
}

/// How much to grow the box by in blocks, and marks the nested test; None when there is nothing to grow
/// or the test is already the nested one.
pub fn enter() -> Option<f64> {
    if INSIDE.with(Cell::get) {
        return None;
    }
    let grow = GROW.with(Cell::get);
    if grow == 0 {
        return None;
    }
    INSIDE.with(|i| i.set(true));
    Some(f64::from(grow) / 100.0)
}

pub fn leave() {
    INSIDE.with(|i| i.set(false));
}

pub fn labels(get: impl Fn(&str) -> Option<String>) -> Vec<String> {
    vec![format!(
        "Hitbox growth: +{:.2} blocks",
        f64::from(read(get)) / 100.0
    )]
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
    Some(("grow", next.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn the_default_is_a_tenth_of_a_block() {
        assert_eq!(read(none), 10);
        assert_eq!(labels(none), ["Hitbox growth: +0.10 blocks"]);
    }

    #[test]
    fn the_button_cycles_and_sets_the_growth() {
        let (key, value) = click(none, 0).unwrap();
        assert_eq!((key, value.as_str()), ("grow", "20"));
        assert_eq!(enter(), Some(0.2));
        leave();
        let at_end = |k: &str| (k == "grow").then(|| "30".to_owned());
        assert_eq!(click(at_end, 0).unwrap().1, "0", "30 wraps to 0");
        assert_eq!(enter(), None, "no growth, nothing to run again");
        assert!(click(none, 4).is_none());
        configure(DEFAULT);
    }

    #[test]
    fn the_nested_test_runs_the_original() {
        configure(10);
        assert_eq!(enter(), Some(0.1));
        assert_eq!(enter(), None, "nested");
        leave();
        assert_eq!(enter(), Some(0.1));
        leave();
    }

    #[test]
    fn stored_junk_falls_back() {
        assert_eq!(read(|_| Some("7".to_owned())), DEFAULT);
        assert_eq!(read(|_| Some("banana".to_owned())), DEFAULT);
        assert_eq!(read(|_| Some(" 5\n".to_owned())), 5);
    }
}
