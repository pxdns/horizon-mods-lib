// While the zoom is on, the wheel changes it instead of moving the hotbar.
fn zoomify_wheel(game: &crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    use crate::game::lib_zoomify as m;
    if game.native_on(m::ID) && m::wheel(steps) {
        *claimed = true;
    }
}
