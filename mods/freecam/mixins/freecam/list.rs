// The mod's row in Options > Mods.
fn freecam_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_freecam as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Freecam".into(), m::DEFAULT_KEY.into()]],
    });
}
