// Once a tick: hand the game the hurt camera these settings give (vanilla when the mod is off).
fn better_hurt_cam_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_better_hurt_cam as m;
    if !game.native_on(m::ID) {
        game.native_hurt_cam_clear();
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_hurt_cam(s.enabled, s.multiplier, s.old_type, s.heart_blink);
}
