// A bundle's item grid in a tooltip: answered for a shulker box with its 27 slots, nine across.
fn shulker_box_tooltip_bundle(
    items: &Items,
    stack: &ItemStack,
    x: f32,
    y: f32,
    box_width: f32,
) -> std::ops::ControlFlow<Option<TooltipLayout>> {
    use lib_shulker_box_tooltip as m;
    let _ = items;
    if !m::grid_on() {
        return std::ops::ControlFlow::Continue(());
    }
    let Some(data) = stack.data() else {
        return std::ops::ControlFlow::Continue(());
    };
    if data.container.is_empty() {
        return std::ops::ControlFlow::Continue(());
    }
    let slots = data
        .container
        .iter()
        .filter_map(|(slot, s)| m::slot_at(*slot, x, y, box_width).map(|at| (at, s.clone())))
        .collect();
    std::ops::ControlFlow::Break(Some(TooltipLayout {
        width: m::GRID_WIDTH,
        height: m::GRID_HEIGHT,
        slots,
        selected: None,
        overflow: None,
        // The progress bar a bundle has is drawn far off screen.
        bar: [-1000.0, -1000.0],
        fill: 0.0,
        full: false,
        label: None,
        empty_description: None,
    }))
}
