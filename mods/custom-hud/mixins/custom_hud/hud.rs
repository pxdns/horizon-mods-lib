// The box in the corner the player picked; the debug screen takes the top corners.
fn custom_hud_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_custom_hud as m;
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
            [m::is(get, "fps"), m::is(get, "coords"), m::is(get, "facing"), m::is(get, "cps")],
        )
    };
    let corner = if right { Corner::TopRight } else { Corner::TopLeft };
    if !shown || !game.native_corner_shown(corner) {
        return;
    }
    let clicks = if show[3] { game.native_clicks().0 } else { 0 };
    let game = &*game;
    let (fps, _, _) = game.native_frame_rate();
    let mut reading = m::Reading { fps, clicks, ..m::Reading::default() };
    if let Some(p) = game.native_player() {
        reading.x = p.x;
        reading.y = p.y;
        reading.z = p.z;
        reading.yaw = p.yaw;
    } else {
        // Coordinates and facing mean nothing outside a world.
    }
    let lines = m::lines(reading, show);
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
