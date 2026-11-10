// Once a tick: hand the game the ping view these settings give (the vanilla bars when the mod is off).
fn ping_view_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_ping_view as m;
    if !game.native_on(m::ID) {
        game.native_ping_view_clear();
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_ping_view(s.rgb(), s.show_ms, s.show_nametag, s.text_shadow);
}
