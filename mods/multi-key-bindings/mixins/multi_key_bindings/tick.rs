// Once a tick: the first tick loads the stored extra keys.
fn multi_key_bindings_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::keybinds::lib_multi_key_bindings as m;
    if !game.native_window() {
        return;
    }
    if game.native_on(m::ID) {
        m::load(game.native_setting(m::ID, "extras"));
    } else {
        m::forget_all();
    }
}
