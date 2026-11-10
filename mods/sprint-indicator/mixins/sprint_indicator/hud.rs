// The speed or slowness icon beside the hotbar, after the hotbar is drawn.
fn sprint_indicator_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::lib_sprint_indicator as m;
    use horizon_client::hud_layout::{self, Blend, GuiDraw};
    let _ = &corners;
    if !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    let Some(motion) = game.native_local_motion() else {
        return;
    };
    let settings = m::Settings::read(|k| game.native_setting(m::ID, k));
    let Some(icon) = settings.icon(motion.sprinting, motion.crouching, motion.sprint_key) else {
        return;
    };
    let (x, y) = settings.position(batch.width as i32, batch.height as i32, motion.main_hand_left);
    let draw = GuiDraw::Sprite {
        sprite: std::borrow::Cow::Borrowed(if icon.crouch { "mob_effect/slowness" } else { "mob_effect/speed" }),
        x,
        y,
        w: m::SIZE,
        h: m::SIZE,
        region: None,
        color: hud_layout::white(icon.alpha),
        blend: Blend::Textured,
    };
    hud_layout::paint(batch, &game.gui.atlas, &[draw]);
}
