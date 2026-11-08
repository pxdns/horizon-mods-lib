// Takes over the crosshair layer: hides the game's crosshair and draws the one the player designed.
fn crosshair_editor_layer(
    game: &mut crate::game::Game<'_>,
    layer: &str,
    batch: &mut horizon_client::hud::HudBatch,
    canceled: &mut bool,
) {
    use crate::game::lib_crosshair_editor as m;
    if layer != "minecraft:crosshair" || !game.native_on(m::ID) {
        return;
    }
    let look = {
        let game = &*game;
        let get = |k: &str| game.native_setting(m::ID, k);
        if !m::is(get, "shown") {
            return;
        }
        m::look(get, game.ticks.into())
    };
    *canceled = true;
    let (cx, cy) = ((batch.width as f32 / 2.0).floor() as i32, (batch.height as f32 / 2.0).floor() as i32);
    for (rect, ink) in m::rects(look, cx, cy) {
        game.native_fill(batch, rect, ink);
    }
}
