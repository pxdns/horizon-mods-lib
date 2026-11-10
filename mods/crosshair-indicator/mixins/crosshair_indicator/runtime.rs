// Crosshair Indicator, a native Horizon mod: Shows corner brackets around your crosshair when you're aiming at a player.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_crosshair_indicator`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "crosshair_indicator";
pub const NAME: &str = "Crosshair Indicator";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Shows corner brackets around your crosshair when you're aiming at a player.";

/// The crosshair sprite is 15 by 15 pixels, centred on the screen.
pub const SPRITE: i32 = 15;
/// The 12 white pixels of the original's `crosshair.png`: four corner brackets round a 7 by 7 square at (4, 4).
pub const PIXELS: [(i32, i32); 12] = [
    (4, 4), (5, 4), (9, 4), (10, 4),
    (4, 5), (10, 5),
    (4, 9), (10, 9),
    (4, 10), (5, 10), (9, 10), (10, 10),
];

/// Whether the brackets show: the crosshair is on a player who is not in spectator mode. A mob, an armor stand or
/// anything else does not count, and neither does a block or nothing (the game then gives no target).
pub fn active(kind: &str, spectator: bool) -> bool {
    kind == "minecraft:player" && !spectator
}

/// The sprite's top-left corner for a screen of `width` by `height` GUI pixels: `Gui.extractCrosshair`'s `(w - 15) / 2`.
pub fn corner(width: i32, height: i32) -> (i32, i32) {
    ((width - SPRITE) / 2, (height - SPRITE) / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_player_shows_the_brackets() {
        assert!(active("minecraft:player", false));
    }

    #[test]
    fn a_player_in_spectator_mode_does_not() {
        assert!(!active("minecraft:player", true));
    }

    #[test]
    fn a_mob_or_another_entity_does_not() {
        for kind in ["minecraft:cow", "minecraft:zombie", "minecraft:armor_stand", "minecraft:item_frame", "minecraft:mannequin"] {
            assert!(!active(kind, false), "{kind}");
        }
    }

    #[test]
    fn no_target_and_a_block_give_nothing_to_ask() {
        // The game answers `None` for a block and for nothing, so the hook never reaches `active`; an empty kind is no player.
        assert!(!active("", false));
    }

    #[test]
    fn the_sprite_is_the_originals_twelve_pixels_inside_a_15_square() {
        assert_eq!(PIXELS.len(), 12);
        assert!(PIXELS.iter().all(|&(x, y)| (0..SPRITE).contains(&x) && (0..SPRITE).contains(&y)));
        // Four corner brackets: each is an L of three pixels round a 7 by 7 square from (4, 4) to (10, 10), none inside it.
        assert!(PIXELS.iter().all(|&(x, y)| (4..=10).contains(&x) && (4..=10).contains(&y)));
        assert!(PIXELS.iter().all(|&(x, y)| x == 4 || x == 10 || y == 4 || y == 10), "only the square's edge");
        let mut sorted = PIXELS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 12, "no pixel twice");
        // The middle of each side is empty: the brackets leave a gap.
        assert!(!PIXELS.contains(&(7, 4)) && !PIXELS.contains(&(4, 7)) && !PIXELS.contains(&(10, 7)) && !PIXELS.contains(&(7, 10)));
    }

    #[test]
    fn the_corner_matches_the_games_integer_division() {
        assert_eq!(corner(1920, 1080), (952, 532));
        assert_eq!(corner(427, 240), (206, 112));
        assert_eq!(corner(854, 480), (419, 232));
    }
}
