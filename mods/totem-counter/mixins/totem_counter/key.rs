// The reset key went down.
fn totem_counter_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_totem_counter as m;
    if !pressed || !game.native_on(m::ID) || key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    game.native_totem_counter_reset(None);
    game.native_chat(&m::reset_message(None, true));
}
