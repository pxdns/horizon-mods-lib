// The mod's row in Options > Mods.
fn visual_keys_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_visual_keys as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::TOGGLE_KEY.into(), "Show keyboard".into(), m::TOGGLE_DEFAULT.into()]],
    });
}
