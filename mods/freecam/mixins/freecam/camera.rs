// The camera the frame renders from: the original run again with the mod's own angles, then adjusted.
fn freecam_camera(
    game: &mut crate::game::Game<'_>,
    partial: f32,
) -> std::ops::ControlFlow<Option<(glam::DVec3, f32, f32)>> {
    use crate::game::lib_freecam as m;
    if !game.native_on(m::ID) {
        return std::ops::ControlFlow::Continue(());
    }
    // `enter` is None when the mod is off or this is the nested call: the original body runs.
    let Some((yaw, pitch)) = m::enter() else {
        return std::ops::ControlFlow::Continue(());
    };
    let (real_yaw, real_pitch) = (game.yaw, game.pitch);
    game.yaw = yaw;
    game.pitch = pitch;
    let view = game.camera_view(partial);
    game.yaw = real_yaw;
    game.pitch = real_pitch;
    m::leave();
    let real = view.map(|(p, yaw, pitch)| ([p.x, p.y, p.z], yaw, pitch));
    std::ops::ControlFlow::Break(m::view(real).map(|(p, yaw, pitch)| (glam::DVec3::new(p[0], p[1], p[2]), yaw, pitch)))
}
