// The mod's row in Options > Mods.
fn multi_key_bindings_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client::keybinds::lib_multi_key_bindings as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
