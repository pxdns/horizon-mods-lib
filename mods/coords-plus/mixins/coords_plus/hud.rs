// The box in the corner the player picked; the debug screen takes the top corners.
fn coords_plus_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_coords_plus as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) {
        return;
    }
    let (shown, right, show) = {
        let game = &*game;
        let get = |k: &str| game.native_setting(m::ID, k);
        (
            m::is(get, "shown"),
            m::is(get, "right"),
            [m::is(get, "coords"), m::is(get, "nether"), m::is(get, "biome")],
        )
    };
    let corner = if right { Corner::BottomRight } else { Corner::BottomLeft };
    if !shown || !game.native_corner_shown(corner) {
        return;
    }
    let game = &*game;
    // Nothing to read outside a world.
    let Some(p) = game.native_player() else {
        return;
    };
    let (bx, by, bz) = (p.x.floor() as i32, p.y.floor() as i32, p.z.floor() as i32);
    // The biome of the loaded chunk the player stands in, as the advancement checks read it.
    let biome = game
        .world
        .chunks
        .get(&(bx >> 4, bz >> 4))
        .map(|c| game.res.biomes().get(c.biome(bx, by, bz)).name.clone())
        .unwrap_or_default();
    let reading = m::Reading { x: p.x, y: p.y, z: p.z, dimension: p.dimension.to_owned(), biome };
    let lines = m::lines(&reading, show);
    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|l| game.native_text_width(l)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(corner, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, line) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], m::INK, true);
    }
}
