// The mod's row in Options > Mods.
fn sprint_indicator_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_sprint_indicator as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
