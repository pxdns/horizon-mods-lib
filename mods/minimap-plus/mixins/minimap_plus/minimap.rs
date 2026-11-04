// The minimap Horizon draws, unless another mod's came first.
fn minimap_plus_view(game: &crate::game::Game<'_>, minimap: &mut Option<crate::game::native_hooks::NativeMinimap>) {
    use crate::game::lib_minimap_plus as m;
    use horizon_client::extras::Corner;
    if minimap.is_some() || !game.native_on(m::ID) {
        return;
    }
    let config = m::read(|k| game.native_setting(m::ID, k));
    if let Some((corner, size, blocks_per_pixel, rotate, round)) = m::view(&config, m::big()) {
        let corner = [Corner::TopRight, Corner::BottomRight, Corner::BottomLeft, Corner::TopLeft][corner];
        *minimap = Some(crate::game::native_hooks::NativeMinimap { corner, size, blocks_per_pixel, rotate, round });
    }
}
