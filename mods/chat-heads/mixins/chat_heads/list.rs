// The mod's row in Options > Mods.
fn chat_heads_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client::chat::lib_chat_heads as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
