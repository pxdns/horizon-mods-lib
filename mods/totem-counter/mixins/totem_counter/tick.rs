// Once a tick: switch TotemCounter on with these settings (off with the mod); it also reads the totems carried.
fn totem_counter_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::lib_totem_counter as m;
    if !game.native_on(m::ID) {
        game.native_totem_counter(None);
        return;
    }
    let s = m::Settings::read(|k| game.native_setting(m::ID, k));
    let (x, y) = if s.position == 1 { (4, 4) } else { (-1, -1) };
    game.native_totem_counter(Some(horizon_client::totem_counter::Config {
        display_enabled: s.display_enabled,
        x,
        y,
        use_default_totem: s.use_default_totem,
        display_colors: s.display_colors,
        colored_xp_bar: s.colored_xp_bar,
        always_show_bar: s.always_show_bar,
        show_pop_counter: s.show_pop_counter,
        counter_enabled: s.counter_enabled,
        separator: s.separator,
        counter_colors: s.counter_colors,
        show_in_tab: s.show_in_tab,
    }));
}
