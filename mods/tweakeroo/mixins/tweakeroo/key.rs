// The hotbar cycle key.
fn tweakeroo_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_tweakeroo as m;
    use horizon_client::menu::Input;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() || game.remote.is_some() {
        return;
    }
    if key != game.native_mod_key(m::ID, m::CYCLE_KEY, m::CYCLE_DEFAULT) || !m::is(|k| game.native_setting(m::ID, k), "cycle") {
        return;
    }
    for (slot, hotbar) in m::cycle_plan() {
        game.tweak_click(slot, hotbar as i32, Input::Swap);
    }
}
