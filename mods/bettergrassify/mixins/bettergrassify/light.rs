// lighting::ambient_occlusion runs once for each face lit with smooth lighting: note, for the side of a grass block, whether the cell beside it steps down onto more grass.
fn better_grassify_light(pad: &Padded, info: &[Info], own: &Info, pos: [i32; 3], face: D) {
    use super::lib_better_grassify as m;
    if !m::on() || !matches!(own.tints[0], super::Tint::GrassBlock) {
        m::set_pending(None);
        return;
    }
    let (dx, dz) = match face {
        D::North => (0, -1),
        D::South => (0, 1),
        D::West => (-1, 0),
        D::East => (1, 0),
        _ => {
            m::set_pending(None);
            return;
        }
    };
    let [x, y, z] = pos;
    let below = pad.blocks[pi(x + dx, y - 1, z + dz)];
    if !matches!(info[below as usize].tints[0], super::Tint::GrassBlock) {
        m::set_pending(None);
        return;
    }
    m::set_pending(Some((x, y, z, face as u8, pad.blocks[pi(x, y, z)])));
}
