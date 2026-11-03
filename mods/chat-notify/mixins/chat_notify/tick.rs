// Once a tick: give the chat module the player's name and options, and play the sound when a ping is due.
fn chat_notify_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_chat_notify as m;
    if !game.native_on(m::ID) || !game.native_window() {
        return;
    }
    m::set_name(&game.options.player_name());
    let get = |k: &str| game.native_setting(m::ID, k);
    m::configure(m::is(get, "players"), m::is(get, "server"), m::is(get, "own"));
    if m::take() {
        let at = game.native_player().map(|p| [p.x, p.y, p.z]);
        game.audio.play("minecraft:entity.experience_orb.pickup", at, 1.0, 1.0);
    }
}
