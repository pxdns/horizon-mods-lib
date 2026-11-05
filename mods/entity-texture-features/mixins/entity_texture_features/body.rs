// A mob's body model and texture: the texture may be one of the variants a resource pack adds.
fn entity_texture_features_body(m: &Mob) -> std::ops::ControlFlow<(Cow<'static, str>, Cow<'_, str>)> {
    use lib_entity_texture_features as r;
    if !r::random_on() || !r::enter_body() {
        return std::ops::ControlFlow::Continue(());
    }
    let (model, texture) = body(m);
    r::leave_body();
    let texture = r::pick(&texture, m.id);
    std::ops::ControlFlow::Break((model, Cow::Owned(texture)))
}
