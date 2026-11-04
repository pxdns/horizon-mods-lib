// The statistics screen opens: a fresh, empty search.
fn search_stats_open(game: &mut crate::game::Game<'_>) {
    use lib_search_stats as m;
    let show = game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "box");
    m::reset(show);
}
