// The mod's row in Options > Mods.
fn totem_tweaks_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client::item_activation::lib_totem_tweaks as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
