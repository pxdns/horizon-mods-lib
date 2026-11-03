// The camera's hurt and bob pose: Horizon's own, rebuilt with the hurt tilt scaled and the death tilt left out when asked.
fn better_hurt_cam_bob(game: &crate::game::Game<'_>, partial: f32) -> std::ops::ControlFlow<glam::Mat4> {
    use crate::game::lib_better_hurt_cam as m;
    if !game.native_on(m::ID) {
        return std::ops::ControlFlow::Continue(());
    }
    let settings = m::settings(|k| game.native_setting(m::ID, k));
    if settings.is_vanilla() {
        return std::ops::ControlFlow::Continue(());
    }
    let Some(p) = game.player.as_ref() else {
        return std::ops::ControlFlow::Continue(());
    };
    let hurt = &p.health.hurt;
    let state = BobState {
        living: Some(HurtBob {
            hurt_time: p.health.hurt_animation_time as f32 - partial,
            hurt_duration: hurt.hurt_duration,
            hurt_dir: p.health.hurt_dir,
            death_time: if settings.death { game.death_time().map(|t| t as f32 + partial) } else { None },
        }),
        player: Some((
            game.avatar.backwards_interpolated_walk_distance(partial),
            game.avatar.interpolated_bob(partial),
        )),
    };
    let tilt = game.settings.file.double("damageTiltStrength") * settings.factor();
    std::ops::ControlFlow::Break(glam::Mat4::from_cols_array(&bob_matrix(&state, tilt, game.settings.view_bobbing).m))
}
