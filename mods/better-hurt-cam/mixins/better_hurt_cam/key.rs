// One of the three keys went down: the toggle, or the multiplier up or down by a tenth (shown over the hotbar).
fn better_hurt_cam_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::lib_better_hurt_cam as m;
    if !pressed || !game.native_on(m::ID) || !game.native_playing() {
        return;
    }
    for name in [m::KEY_TOGGLE, m::KEY_PLUS, m::KEY_MINUS] {
        if key != game.native_mod_key(m::ID, name, m::DEFAULT_KEY) {
            continue;
        }
        let mut settings = m::Settings::read(|k| game.native_setting(m::ID, k));
        if let Some((file, value, show)) = settings.key(name) {
            game.native_set_setting(m::ID, file, &value);
            if show {
                game.native_overlay_message("Hurtcam multiplier modified to ", &settings.multiplier_text());
            }
        }
        return;
    }
}
