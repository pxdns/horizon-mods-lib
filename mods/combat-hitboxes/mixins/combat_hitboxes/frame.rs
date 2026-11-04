// Each frame: tell the mod how much the player wants the boxes to grow.
fn combat_hitboxes_frame(game: &mut crate::game::Game<'_>, elapsed: std::time::Duration) {
    use crate::game::lib_combat_hitboxes as m;
    let _ = elapsed;
    m::configure(if game.native_on(m::ID) { m::read(|k| game.native_setting(m::ID, k)) } else { 0 });
}
