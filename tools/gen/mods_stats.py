"""Search Stats: type to filter the statistics screen's Items and Mobs lists. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

STATS = []

STATS.append(dict(
    slug="search-stats", id="search_stats", name="SearchStats", title="SearchStats", raw=False, needs_api=408,
    usage="Nothing to configure (the original has no settings). Open Statistics and type: a box beside Done keeps the General, Items and Mobs rows whose name contains what you typed, in any case, as one piece.",
    description="Adds a search bar to the statistics screen to quickly find specific entries.",
    upstream="SearchStats (searchstats) by TerminalMC", upstream_licence="Apache-2.0",
    command=None, hooks=["list", "tick"],
    options=None,
    runtime_extra=r'''
/// The search box is the whole mod: its text, hint and layout are Horizon's (`native_stats_search`, hook API 408).
/// The box's width and height as the original gives them, and the Done button beside it.
pub const BOX_WIDTH: u32 = 120;
pub const BOX_HEIGHT: u32 = 18;
pub const DONE_WIDTH: u32 = 120;
/// `SearchStats.setSearchString` lower-cases the text, and each list keeps the rows whose lower-cased name contains it.
pub fn matches(name: &str, query: &str) -> bool {
    query.is_empty() || name.to_lowercase().contains(&query.to_lowercase())
}
''',
    tests=r'''
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
''',
    helpers={
        "tick": '''// Once a tick: the Statistics screens opened from now on have the search box (none while the mod is off).
fn search_stats_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_search_stats as m;
    game.native_stats_search(game.native_on(m::ID));
}
''',
    },
))
