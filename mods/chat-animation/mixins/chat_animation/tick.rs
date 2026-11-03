// Once a tick: give the chat module the player's setting.
fn chat_animation_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_chat_animation as m;
    if !game.native_window() {
        return;
    }
    let on = game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "slide");
    m::configure(on);
}
