// SearchStats, a native Horizon mod: Adds a search bar to the statistics screen to quickly find specific entries.
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_search_stats`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "search_stats";
pub const NAME: &str = "SearchStats";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str = "Adds a search bar to the statistics screen to quickly find specific entries.";

/// The search box is the whole mod: its text, hint and layout are Horizon's (`native_stats_search`, hook API 408).
/// The box's width and height as the original gives them, and the Done button beside it.
pub const BOX_WIDTH: u32 = 120;
pub const BOX_HEIGHT: u32 = 18;
pub const DONE_WIDTH: u32 = 120;
/// `SearchStats.setSearchString` lower-cases the text, and each list keeps the rows whose lower-cased name contains it.
pub fn matches(name: &str, query: &str) -> bool {
    query.is_empty() || name.to_lowercase().contains(&query.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_stays_when_its_name_contains_the_text_in_any_case() {
        assert!(matches("Diamond Pickaxe", "PICK"));
        assert!(matches("Zombie", "zom"));
        assert!(!matches("Zombie", "skeleton"));
    }

    #[test]
    fn an_empty_search_keeps_everything() {
        assert!(matches("anything", ""));
    }

    #[test]
    fn the_text_is_one_piece_not_words() {
        assert!(!matches("Diamond Pickaxe", "pickaxe diamond"));
        assert!(matches("Time Played", "me pl"));
    }

    #[test]
    fn the_box_and_done_are_the_originals_sizes() {
        assert_eq!((BOX_WIDTH, BOX_HEIGHT, DONE_WIDTH), (120, 18, 120));
    }
}
