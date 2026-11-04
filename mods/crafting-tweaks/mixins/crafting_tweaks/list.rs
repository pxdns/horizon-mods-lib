// The mod's row in Options > Mods.
fn crafting_tweaks_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_crafting_tweaks as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::ROTATE_KEY.into(), "Rotate".into(), m::ROTATE_DEFAULT.into()], [m::BALANCE_KEY.into(), "Balance".into(), m::BALANCE_DEFAULT.into()], [m::CLEAR_KEY.into(), "Clear".into(), m::CLEAR_DEFAULT.into()]],
    });
}
