// The zoom in effect, unless another mod's came first.
fn simple_zoom_view(game: &crate::game::Game<'_>, zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    use crate::game::lib_simple_zoom as m;
    if zoom.is_none()
        && game.native_on(m::ID)
        && let Some((factor, smooth, turn)) = m::view()
    {
        *zoom = Some(crate::game::native_hooks::NativeZoom { factor, smooth, turn });
    }
}
