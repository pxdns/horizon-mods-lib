// More Chat History, a native Horizon mod: Increases the maximum length of the chat history, from the vanilla 100 messages to 16384.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_more_chat_history`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "more_chat_history";
pub const NAME: &str = "More Chat History";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Increases the maximum length of the chat history, from the vanilla 100 messages to 16384.";

use std::cell::Cell;

/// Vanilla's limit.
pub const VANILLA: usize = 100;
/// The original's `EXPANDED_MAX_HISTORY`, fixed: it has no setting.
pub const EXPANDED: usize = 16384;

thread_local! {
    static LIMIT: Cell<usize> = const { Cell::new(EXPANDED) };
}

/// How many messages and wrapped lines chat keeps.
pub fn limit() -> usize {
    LIMIT.with(Cell::get)
}

/// The game tells the chat each tick: the expanded limit while the mod is on, vanilla's while it is off.
pub fn configure(on: bool) {
    LIMIT.with(|l| l.set(if on { EXPANDED } else { VANILLA }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_limit_is_the_originals_16384() {
        configure(true);
        assert_eq!(limit(), 16384);
    }

    #[test]
    fn off_gives_vanillas_hundred() {
        configure(false);
        assert_eq!(limit(), 100);
        configure(true);
    }

    #[test]
    fn it_follows_the_switch_both_ways() {
        configure(false);
        configure(true);
        assert_eq!(limit(), EXPANDED);
        configure(false);
        assert_eq!(limit(), VANILLA);
        configure(true);
    }

    #[test]
    fn the_limit_is_far_above_vanilla() {
        assert_eq!(EXPANDED / VANILLA, 163);
    }
}
