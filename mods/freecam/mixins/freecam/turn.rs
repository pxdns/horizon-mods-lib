// Mouse look: while the mod owns the view, the mouse turns that and not the player.
fn freecam_turn(game: &mut crate::game::Game<'_>, xo: f64, yo: f64) -> std::ops::ControlFlow<()> {
    use crate::game::lib_freecam as m;
    if !game.native_on(m::ID) {
        return std::ops::ControlFlow::Continue(());
    }

    if m::turn(xo, yo) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
