// The mod's row in Options > Mods.
fn better_hurt_cam_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_better_hurt_cam as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: vec![[m::KEY_TOGGLE.into(), "Toggle HurtCam".into(), m::DEFAULT_KEY.into()], [m::KEY_PLUS.into(), "Increase HurtCam Strength".into(), m::DEFAULT_KEY.into()], [m::KEY_MINUS.into(), "Decrease HurtCam Strength".into(), m::DEFAULT_KEY.into()]],
    });
}
