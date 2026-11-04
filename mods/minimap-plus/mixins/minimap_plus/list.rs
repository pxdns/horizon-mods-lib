// The mod's row in Options > Mods.
fn minimap_plus_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_minimap_plus as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::BIG_KEY.into(), "Big map".into(), m::BIG_DEFAULT.into()], [m::IN_KEY.into(), "Zoom in".into(), m::IN_DEFAULT.into()], [m::OUT_KEY.into(), "Zoom out".into(), m::OUT_DEFAULT.into()]],
    });
}
