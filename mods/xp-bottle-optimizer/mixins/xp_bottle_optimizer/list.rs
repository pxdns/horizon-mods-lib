// The mod's row in Options > Mods.
fn xp_bottle_optimizer_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_xp_bottle_optimizer as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
