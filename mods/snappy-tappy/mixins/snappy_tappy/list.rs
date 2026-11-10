// The mod's row in Options > Mods.
fn snappy_tappy_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_snappy_tappy as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Toggle Snappy Tappy".into(), m::DEFAULT_KEY.into()]],
    });
}
