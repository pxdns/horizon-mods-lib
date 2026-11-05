// Mesher::face with Better Grass: the side of a grass block next to a step down to more grass draws the grass top texture.
fn better_grassify_face(
    mesher: &Mesher<'_>,
    pad: &Padded,
    tints: &mut Tints<'_>,
    out: &mut SectionMesh,
    pos: [i32; 3],
    origin: [f32; 3],
    face: &BakedFace,
    ao: bool,
) -> std::ops::ControlFlow<()> {
    use lib_better_grassify as m;
    if !m::on() {
        return std::ops::ControlFlow::Continue(());
    }
    // The block's id and its facts: `face` takes them as a tuple pattern, which a hook cannot bind.
    let id = pad.blocks[pi(pos[0], pos[1], pos[2])];
    let state = &mesher.tables.info[id as usize];
    if !matches!(state.tints[0], Tint::GrassBlock) {
        return std::ops::ControlFlow::Continue(());
    }
    let q = &face.quad;
    use horizon_assets::model::Direction as Dir;
    let (dx, dz) = match q.facing {
        Dir::North => (0, -1),
        Dir::South => (0, 1),
        Dir::West => (-1, 0),
        Dir::East => (1, 0),
        _ => return std::ops::ControlFlow::Continue(()),
    };
    let [x, y, z] = pos;
    let below = pad.blocks[pi(x + dx, y - 1, z + dz)];
    if !matches!(mesher.tables.info[below as usize].tints[0], Tint::GrassBlock) {
        return std::ops::ControlFlow::Continue(());
    }
    // The overlay quad (the tinted one) is dropped; the base quad takes the top texture and the tint.
    if q.tint_index >= 0 {
        return std::ops::ControlFlow::Break(());
    }
    let top = match m::cached(id) {
        Some(layer) => layer,
        None => {
            let mut models = Vec::new();
            let found = mesher
                .render
                .model_catalog
                .models_at(BlockStateId(id), BlockPos::new(0, 0, 0), &mut models)
                .ok()
                .and_then(|()| models.iter().flat_map(|model| model.faces.iter()).find(|f| f.quad.facing == Dir::Up).map(|f| f.layer));
            m::remember(id, found);
            found
        }
    };
    let Some(top) = top else {
        return std::ops::ControlFlow::Continue(());
    };
    let tier = state.tier.max(hdr_tier("", q.light_emission));
    let shade_face = q.shade_direction_override.unwrap_or(q.facing);
    let directional = lighting::cardinal(mesher.nether_shading, shade_face);
    let (gray, coords) = if ao {
        let info = &mesher.tables.info;
        let (brightness, coords) = lighting::ambient_occlusion(pad, info, state, pos, &q.positions, q.facing);
        let gray = brightness.map(|b| lighting::scale(lighting::gray(b), directional));
        (gray, coords)
    } else {
        let coords = match q.cull {
            Some(d) => lighting::coords(pad, state, lighting::step(pos, d)),
            None => lighting::flat(pad, state, pos, &q.positions, q.facing),
        };
        ([lighting::gray(directional); 4], [coords; 4])
    };
    let tint = Mesher::tint(state, 0, tints, pos);
    let target = if face.translucent { &mut out.translucent } else { &mut out.solid };
    for i in 0..4 {
        let g = gray[i] as u32;
        let rgb = match tint {
            Some(t) => [16, 8, 0].map(|shift| g * (t >> shift & 255) / 255),
            None => [g; 3],
        };
        let p = q.positions[i];
        target.push(TerrainVertex::new(
            [p[0] + origin[0], p[1] + origin[1], p[2] + origin[2]],
            q.uv[i],
            top as u16 | OPAQUE_LAYER,
            emitting(pack_rgb(rgb), tier),
            pack_coords(lighting::with_emission(coords[i], q.light_emission)),
        ));
    }
    std::ops::ControlFlow::Break(())
}
