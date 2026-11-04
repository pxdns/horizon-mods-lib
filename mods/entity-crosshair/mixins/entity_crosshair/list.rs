// The mod's row in Options > Mods.
fn entity_crosshair_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client::hud_layout::lib_entity_crosshair as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
