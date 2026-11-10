// The toggle key went down: the widget turns off or on.
fn armor_hud_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_armor_hud as m;
    if !pressed || !game.native_on(m::ID) || key != game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        return;
    }
    let stored = {
        let game = &*game;
        m::click(|k| game.native_setting(m::ID, k), 0)
    };
    if let Some((k, value)) = stored {
        game.native_set_setting(m::ID, k, &value);
        game.native_chat(if value == "true" { "Armor Hud toggled on" } else { "Armor Hud toggled off" });
    }
}
