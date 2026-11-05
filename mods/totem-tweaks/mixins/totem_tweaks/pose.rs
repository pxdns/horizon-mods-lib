// The totem's pose for a frame of the pop, with the player's settings.
fn totem_tweaks_pose(ticks: i32, partial: f32, off_x: f32, off_y: f32, aspect: f32) -> std::ops::ControlFlow<Mat4> {
    use lib_totem_tweaks as m;
    if !m::active() {
        return std::ops::ControlFlow::Continue(());
    }
    let p = m::pose(ticks, partial, off_x, off_y, aspect, &m::current());
    std::ops::ControlFlow::Break(
        Mat4::from_translation(Vec3::new(p.x, p.y, p.z))
            * Mat4::from_scale(Vec3::splat(p.scale))
            * Mat4::from_rotation_y(p.spin.to_radians())
            * Mat4::from_rotation_x(p.rock_x.to_radians())
            * Mat4::from_rotation_z(p.rock_z.to_radians()),
    )
}
