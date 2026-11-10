// Once a tick: hand the game the hitbox style these settings give (vanilla hitboxes when the mod is off).
fn combat_hitboxes_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_combat_hitboxes as m;
    if !game.native_on(m::ID) {
        game.native_combat_hitboxes_clear();
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_combat_hitboxes(horizon_client::gizmos::HitboxStyle {
        enabled: s.enabled,
        hide_arrow: s.hide_arrow,
        hide_fireworks: s.hide_fireworks,
        hide_items: s.hide_items,
        change_target_color: s.change_target_color,
        eye_color: m::parse_color(&s.eye_color),
        look_color: m::parse_color(&s.look_color),
        hit_box_color: m::parse_color(&s.hit_box_color),
        target_box_color: m::parse_color(&s.target_box_color),
        hit_box_hurt: s.hit_box_hurt,
        render_eye_height: s.render_eye_height,
        render_look_dir: s.render_look_dir,
        line_look_dir: s.line_look_dir,
        hurt_box_color: m::parse_color(&s.hurt_box_color),
        line1: s.line1,
        dist_for_2: s.dist_for_2,
        line2: s.line2,
        outline_enabled: s.outline_enabled,
        outline_color: m::parse_color(&s.outline_color),
        outline_multiplier: s.outline_multiplier,
    });
}
