// The mod's row in Options > Mods.
fn totem_counter_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_totem_counter as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Reset Pop Counter".into(), m::DEFAULT_KEY.into()]],
    });
}
