// Clumps, a native Horizon mod: Clumps XP orbs together to reduce lag.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_clumps`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "clumps";
pub const NAME: &str = "Clumps";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Clumps XP orbs together to reduce lag.";

/// The original has no settings: the mod is its switch. The rules live in Horizon (`horizon_client::orb_clumps`, hook API 414).
/// The orb merge box and the scan interval are the game's own (1 block on spawn, 0.5 every 20 ticks).
pub const SPAWN_BOX: f64 = 1.0;
/// `ExperienceOrb.tryMergeToExisting` looks for an orb in a box this wide around where the new one appears.
pub fn merge_box() -> f64 {
    SPAWN_BOX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_spawn_box_is_one_block_as_in_the_original() {
        assert_eq!(merge_box(), 1.0);
    }

    #[test]
    fn the_mod_is_named_and_credited() {
        assert_eq!(NAME, "Clumps");
        assert_eq!(ID, "clumps");
        assert!(DESCRIPTION.starts_with("Clumps XP orbs"));
    }
}
