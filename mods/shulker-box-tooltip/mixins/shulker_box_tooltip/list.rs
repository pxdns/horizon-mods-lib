// The mod's row in Options > Mods.
fn shulker_box_tooltip_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_items::bundle::lib_shulker_box_tooltip as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
