// Once a tick: switch Slyde on with this setting (off with the mod).
fn slyde_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_slyde as m;
    if !game.native_on(m::ID) {
        game.native_slyde(None);
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_slyde(Some(horizon_client::slyde::Config { hold_key_to_activate: s.hold_key_to_activate }));
}
