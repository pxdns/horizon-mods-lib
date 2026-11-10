// The mod's row in Options > Mods.
fn no_death_animation_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_no_death_animation as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
