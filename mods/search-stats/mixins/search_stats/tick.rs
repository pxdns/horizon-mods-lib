// Once a tick: the Statistics screens opened from now on have the search box (none while the mod is off).
fn search_stats_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_search_stats as m;
    game.native_stats_search(game.native_on(m::ID));
}
