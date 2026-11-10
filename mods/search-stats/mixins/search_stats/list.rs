// The mod's row in Options > Mods.
fn search_stats_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::lib_search_stats as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
