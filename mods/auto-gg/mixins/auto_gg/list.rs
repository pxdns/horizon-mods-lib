// The mod's row in Options > Mods.
fn auto_gg_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_auto_gg as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY.into(), "Send the message".into(), m::KEY_DEFAULT.into()]],
    });
}
