// Once a tick: tell the chat the limit the player picked.
fn more_chat_history_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_more_chat_history as m;
    if !game.native_window() {
        return;
    }
    m::configure(game.native_on(m::ID));
}
