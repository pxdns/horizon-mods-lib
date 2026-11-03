// Mouse look: while the mod owns the view, the mouse turns that and not the player.
fn freelook_turn(game: &mut crate::game::Game<'_>, xo: f64, yo: f64) -> std::ops::ControlFlow<()> {
    use crate::game::lib_freelook as m;
    if !game.native_on(m::ID) {
        return std::ops::ControlFlow::Continue(());
    }
    let invert = m::is(|k| game.native_setting(m::ID, k), "invert");
    if m::turn(xo, yo, invert) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
