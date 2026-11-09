// Adds the player's offsets, tilt and size to each first-person hand's pose.
fn held_item_pose(
    game: &crate::game::Game<'_>,
    hand: horizon_client::first_person::Hand,
    x: &mut f32,
    y: &mut f32,
    z: &mut f32,
    roll: &mut f32,
    scale: &mut f32,
) {
    use crate::game::lib_held_item as m;
    if !game.native_on(m::ID) {
        return;
    }
    let main = matches!(hand, horizon_client::first_person::Hand::Main);
    let Some(p) = m::pose(|k: &str| game.native_setting(m::ID, k), main) else {
        return;
    };
    *x += p.x;
    *y += p.y;
    *z += p.z;
    *roll += p.tilt;
    *scale *= p.scale;
}
