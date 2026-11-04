// A key went down: on the statistics screen it types into the search.
fn search_stats_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::stats_screen::lib_search_stats as m;
    if pressed && game.screen == Screen::Stats {
        m::press(key);
    }
}
