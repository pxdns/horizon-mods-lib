// The box in the corner the player picked; the debug screen takes the top corners.
fn aurora_hud_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_aurora_hud as m;
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
            [m::is(get, "day"), m::is(get, "clock"), m::is(get, "light"), m::is(get, "held"), m::is(get, "stopwatch")],
        )
    };
    let corner = if right { Corner::TopRight } else { Corner::TopLeft };
    if !shown || !game.native_corner_shown(corner) {
        return;
    }
    let game = &*game;
    // Nothing to read outside a world.
    let Some(p) = game.native_player() else {
        return;
    };
    // The stopwatch starts at the first frame in a world; the tick it started on is kept for the run.
    thread_local! {
        static JOINED: std::cell::Cell<Option<u32>> = const { std::cell::Cell::new(None) };
    }
    let now = game.ticks;
    let joined = JOINED.with(|j| match j.get() {
        Some(t) if t <= now => t,
        _ => {
            j.set(Some(now));
            now
        }
    });
    let feet = [p.x.floor() as i32, p.y.floor() as i32, p.z.floor() as i32];
    let (block, sky) = game.native_light(feet);
    let held = game.inventory.get(game.selected).and_then(|s| s.as_ref()).map_or(0, |s| u32::try_from(s.count).unwrap_or(0));
    let reading = m::Reading {
        day_time: game.native_day_time(),
        light: block.max(sky),
        held,
        world_ticks: u64::from(now - joined),
    };
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
