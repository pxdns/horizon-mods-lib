// The box in the bottom-left corner; the debug screen takes the top corners.
fn better_mount_hud_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_better_mount_hud as m;
    use horizon_client::extras::Corner;
    if !game.native_on(m::ID) || !game.native_corner_shown(Corner::BottomLeft) {
        return;
    }
    if !m::is(|k| game.native_setting(m::ID, k), "shown") {
        return;
    }
    let Some(id) = game.riding else {
        return;
    };
    let Some(mob) = game.mobs.iter().find(|mob| mob.id == id) else {
        return;
    };
    let get = |k: &str| game.native_setting(m::ID, k);
    let mut lines = vec![m::health_line(f64::from(mob.health), f64::from(mob.attrs.max_health))];
    if m::is(get, "speed") {
        lines.push(m::speed_line(mob.position.x - mob.previous.x, mob.position.z - mob.previous.z));
    }
    if m::is(get, "jump") && game.jumpable_mount() {
        lines.push(m::jump_line(f64::from(mob.jump_power()), game.mount.jump_scale));
    }
    if lines.is_empty() {
        return;
    }
    let w = lines.iter().map(|(t, _)| game.native_text_width(t)).fold(0.0, f32::max) + 4.0;
    let h = 10.0 * lines.len() as f32 + 2.0;
    let (x, y) = corners.place(Corner::BottomLeft, w, h);
    game.native_fill(batch, [x, y, w, h], m::BACKGROUND);
    for (i, (line, ink)) in lines.iter().enumerate() {
        game.native_text(batch, line, [x + 2.0, y + 2.0 + 10.0 * i as f32], *ink, true);
    }
}
