// Once a tick: tell the chat what the player switched on and who is online.
fn chat_heads_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_chat_heads as m;
    if !game.native_window() {
        return;
    }
    let (on, hat, others) = {
        let get = |k: &str| game.native_setting(m::ID, k);
        (game.native_on(m::ID), m::is(get, "hat"), m::is(get, "others"))
    };
    m::configure(on, hat, others);
    if on && game.ticks % 20 == 0 {
        let online = game
            .remote_tab_players()
            .map(|(players, ..)| players.iter().map(|p| p.name.to_lowercase()).collect())
            .unwrap_or_default();
        m::set_online(online);
    }
}
