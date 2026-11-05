// Once a tick: tell the animation what the player chose.
fn totem_tweaks_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::item_activation::lib_totem_tweaks as m;
    if !game.native_window() {
        return;
    }
    let settings = m::read(|k| game.native_setting(m::ID, k));
    m::configure(game.native_on(m::ID), settings);
}
