// Once a tick: replies that arrived from the lookups go to chat.
fn tiers_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_tiers as m;
    if !game.native_window() {
        return;
    }
    let now = game.ticks as u64 / 20;
    for line in m::take_replies(now) {
        game.native_chat(&line);
    }
}
