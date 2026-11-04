// The mod's row in Options > Mods.
fn ping_view_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client::debug_chart::lib_ping_view as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
