// The crosshair's ray against an entity's box, run again on a box that has grown.
fn combat_hitboxes_ray(eye: [f64; 3], dir: [f64; 3], reach: f64, b: &Aabb) -> std::ops::ControlFlow<Option<f64>> {
    use crate::game::lib_combat_hitboxes as m;
    let Some(grow) = m::enter() else {
        return std::ops::ControlFlow::Continue(());
    };
    let bigger = b.clone().inflate(Vec3::new(grow, grow, grow));
    let hit = ray_box(eye, dir, reach, bigger);
    m::leave();
    std::ops::ControlFlow::Break(hit)
}
