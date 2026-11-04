// The mod's row in Options > Mods.
fn crystal_optimizer_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::remote::lib_crystal_optimizer as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
