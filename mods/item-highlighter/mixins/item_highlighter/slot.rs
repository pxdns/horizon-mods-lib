// One slot's item on a container screen; a kind picked up recently gets a twinkling star over it.
fn item_highlighter_slot(
    game: &crate::game::Game<'_>,
    batch: &mut HudBatch,
    drag: Option<&DragView>,
    slot: usize,
    stack: &Option<ItemStack>,
    x: f32,
    y: f32,
) -> std::ops::ControlFlow<()> {
    use lib_item_highlighter as m;
    let Some(item) = stack else {
        return std::ops::ControlFlow::Continue(());
    };
    if !m::is_new(&item.item) || !m::enter() {
        return std::ops::ControlFlow::Continue(());
    }
    game.draw_slot_item(batch, drag, slot, stack, x, y);
    m::leave();
    let millis = game.started.elapsed().as_millis();
    for ([dx, dy, w, h], argb) in m::star(millis) {
        batch.fill(&game.gui.atlas, x + dx, y + dy, w, h, argb);
    }
    std::ops::ControlFlow::Break(())
}
