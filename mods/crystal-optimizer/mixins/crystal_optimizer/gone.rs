// The server removed an entity it had told us about.
fn crystal_optimizer_gone(id: u32) {
    crate::game::remote::lib_crystal_optimizer::confirmed(id);
}
