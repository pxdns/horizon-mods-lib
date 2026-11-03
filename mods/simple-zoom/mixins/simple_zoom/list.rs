// The mod's row in Options > Mods.
fn simple_zoom_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_simple_zoom as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Zoom".into(), m::DEFAULT_KEY.into()]],
    });
}
