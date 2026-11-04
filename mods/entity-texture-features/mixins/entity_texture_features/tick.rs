// Once a tick: the switches, and about every five seconds the index of the textures the resource pack provides.
fn entity_texture_features_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::mobs::render::lib_entity_texture_features as r;
    if !game.native_window() {
        return;
    }
    let on = game.native_on(r::ID);
    let (random, emissive) = {
        let get = |k: &str| game.native_setting(r::ID, k);
        (on && r::is(get, "random"), on && r::is(get, "emissive"))
    };
    r::configure(random, emissive);
    if on && game.ticks % 100 == 1 {
        let (variants, glow) = r::index(game.gui.atlas.sprites.keys().map(String::as_str));
        r::set_index(variants, glow);
    }
}
