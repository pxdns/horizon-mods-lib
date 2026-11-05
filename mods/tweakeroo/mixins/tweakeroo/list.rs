// The mod's row in Options > Mods.
fn tweakeroo_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_tweakeroo as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::CYCLE_KEY.into(), "Cycle hotbar".into(), m::CYCLE_DEFAULT.into()]],
    });
}
