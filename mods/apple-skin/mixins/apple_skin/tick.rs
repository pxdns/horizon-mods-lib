// Once a tick: switch AppleSkin's overlays on with these settings (off with the mod).
fn apple_skin_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_apple_skin as m;
    if !game.native_on(m::ID) {
        game.native_appleskin(None);
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    game.native_appleskin(Some(horizon_client::appleskin::Config {
        show_food_values_in_tooltip: s.show_food_values_in_tooltip,
        show_food_values_in_tooltip_always: s.show_food_values_in_tooltip_always,
        show_food_debug_info: s.show_food_debug_info,
        show_saturation_hud_overlay: s.show_saturation_hud_overlay,
        show_food_values_hud_overlay: s.show_food_values_hud_overlay,
        show_food_values_hud_overlay_when_offhand: s.show_food_values_hud_overlay_when_offhand,
        show_food_exhaustion_hud_underlay: s.show_food_exhaustion_hud_underlay,
        show_food_health_hud_overlay: s.show_food_health_hud_overlay,
        show_vanilla_animations_overlay: s.show_vanilla_animations_overlay,
        max_hud_overlay_flash_alpha: s.max_hud_overlay_flash_alpha,
        ..Default::default()
    }));
}
