// The big-map key and the zoom keys: the player's own choice of key.
fn minimap_plus_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_minimap_plus as m;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    if key == game.native_mod_key(m::ID, m::BIG_KEY, m::BIG_DEFAULT) {
        m::toggle_big();
        return;
    }
    let closer = key == game.native_mod_key(m::ID, m::IN_KEY, m::IN_DEFAULT);
    if closer || key == game.native_mod_key(m::ID, m::OUT_KEY, m::OUT_DEFAULT) {
        let next = {
            let game = &*game;
            m::step_zoom(m::read(|k| game.native_setting(m::ID, k)).zoom, closer)
        };
        game.native_set_setting(m::ID, "zoom", &next.to_string());
    }
}
