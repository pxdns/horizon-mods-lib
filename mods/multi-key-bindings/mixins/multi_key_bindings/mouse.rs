// KeyBinds.translate_mouse_all, plus the extra buttons.
fn multi_key_bindings_translate_mouse(binds: &KeyBinds, button: MouseButton) -> std::ops::ControlFlow<Vec<Result<KeyCode, MouseButton>>> {
    use lib_multi_key_bindings as m;
    let name = mouse_name(button);
    let extra = m::actions_for(&name);
    if extra.is_empty() {
        return std::ops::ControlFlow::Continue(());
    }
    let mut bound: Vec<_> = binds
        .translated_mappings(&name)
        .chain(MAPPINGS.iter().filter(|mapping| extra.iter().any(|a| a == mapping.0)).map(|mapping| mapping.0))
        .filter_map(|mapping| {
            let default = KeyBinds::default_of(mapping);
            match mouse_button(default) {
                Some(b) => Some(Err(b)),
                None => key_code(default).map(Ok),
            }
        })
        .collect();
    bound = distinct(bound);
    std::ops::ControlFlow::Break(bound)
}
