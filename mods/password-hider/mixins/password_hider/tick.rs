// Once a tick: tell the chat what the player switched on.
fn password_hider_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::chat::lib_password_hider as m;
    if !game.native_window() {
        return;
    }
    let on = game.native_on(m::ID);
    let get = |k: &str| game.native_setting(m::ID, k);
    m::configure(on && m::is(get, "input"), on && m::is(get, "history"));
}
