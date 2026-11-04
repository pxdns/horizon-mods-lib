// The mod's row in Options > Mods.
fn f5_split_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_f5_split as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::BACK_KEY.into(), "Back view".into(), m::BACK_DEFAULT.into()], [m::FRONT_KEY.into(), "Front view".into(), m::FRONT_DEFAULT.into()]],
    });
}
