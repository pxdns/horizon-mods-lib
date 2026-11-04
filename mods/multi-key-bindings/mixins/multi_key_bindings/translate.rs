// KeyBinds.translate_all, plus the extra keys: a key bound again to an action drives that action's default key too.
fn multi_key_bindings_translate(binds: &KeyBinds, key: KeyCode) -> std::ops::ControlFlow<Vec<Result<KeyCode, MouseButton>>> {
    use lib_multi_key_bindings as m;
    let Some(name) = key_name(key) else {
        return std::ops::ControlFlow::Continue(());
    };
    let extra = m::actions_for(name);
    if extra.is_empty() {
        return std::ops::ControlFlow::Continue(());
    }
    let mut bound: Vec<_> = binds
        .translated_mappings(name)
        .chain(MAPPINGS.iter().filter(|mapping| extra.iter().any(|a| a == mapping.0)).map(|mapping| mapping.0))
        .filter_map(|mapping| {
            let default = KeyBinds::default_of(mapping);
            match key_code(default) {
                Some(code) => Some(Ok(code)),
                None => mouse_button(default).map(Err),
            }
        })
        .collect();
    bound = distinct(bound);
    std::ops::ControlFlow::Break(bound)
}
