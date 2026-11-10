// A key went down or up: time it, and the toggle key turns the mod on or off.
fn snappy_tappy_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_snappy_tappy as m;
    m::pressed(key, pressed);
    if pressed && game.native_playing() && key == game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY) {
        let now = !m::is(|k| game.native_setting(m::ID, k), "enabled");
        game.native_set_setting(m::ID, "enabled", if now { "true" } else { "false" });
        let line = format!("Snappy Tappy: {}", if now { "ON" } else { "OFF" });
        game.titles.set_overlay(horizon_client::text::Text::literal(line), false);
    }
}
