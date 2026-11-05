// A mob's extra layers, and its emissive twin of the body texture drawn at full brightness.
fn entity_texture_features_layers(m: &Mob, age: f32) -> std::ops::ControlFlow<Vec<Layer>> {
    use lib_entity_texture_features as r;
    if !r::emissive_on() || !r::enter_layers() {
        return std::ops::ControlFlow::Continue(());
    }
    let (model, texture) = body(m);
    let Some(glow) = r::glow_of(&texture) else {
        r::leave_layers();
        return std::ops::ControlFlow::Continue(());
    };
    // The original again: the guard lets this call through the hook.
    let mut layers = layers(m, age);
    r::leave_layers();
    layers.push(Layer { model, texture: Cow::Owned(glow), color: 0xFFFF_FFFF, full_bright: true, scroll: 0.0 });
    std::ops::ControlFlow::Break(layers)
}
