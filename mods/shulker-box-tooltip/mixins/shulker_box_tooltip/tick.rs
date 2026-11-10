// Once a tick: switch the container previews on with these settings (off with the mod).
fn shulker_box_tooltip_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_shulker_box_tooltip as m;
    use horizon_client::shulker_preview as sp;
    if !game.native_on(m::ID) {
        game.native_shulker_preview(None);
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_shulker_preview(Some(sp::Config {
        enable: s.enable,
        always_on: s.always_on,
        swap_modes: s.swap_modes,
        colored: s.colored,
        merge: match s.merge.as_str() {
            "ignore" => sp::Merge::Ignore,
            "first_item" => sp::Merge::FirstItem,
            _ => sp::Merge::Separate,
        },
        order: if s.order == "preview_order" { sp::Order::PreviewOrder } else { sp::Order::StackSize },
        max_row_size: s.max_row_size as usize,
        position: match s.position.as_str() {
            "outside" => sp::Position::Outside,
            "outside_top" => sp::Position::OutsideTop,
            "outside_bottom" => sp::Position::OutsideBottom,
            _ => sp::Position::Inside,
        },
        short_item_counts: s.short_item_counts,
        generic_container: s.generic_container,
        show_key_hints: s.show_key_hints,
        tooltip_type: match s.tooltip_type.as_str() {
            "vanilla" => sp::TooltipType::Vanilla,
            "none" => sp::TooltipType::None,
            _ => sp::TooltipType::Mod,
        },
        preview_key: s.preview_key,
        full_preview_key: s.full_preview_key,
    }));
}
