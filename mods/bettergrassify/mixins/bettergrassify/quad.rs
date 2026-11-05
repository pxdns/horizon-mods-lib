// Mesher::quad with Better Grass: the base quad of a grass side (untinted) and its overlay quad (tinted) become one quad with the grass top texture, in the overlay's colours.
fn better_grassify_quad(
    mesher: &Mesher<'_>,
    out: &mut SectionMesh,
    solid_leaves: bool,
    tile: TileLayer,
    v: &[TerrainVertex; 4],
    tint: Option<u32>,
) -> std::ops::ControlFlow<()> {
    use lib_better_grassify as m;
    // One half of a grass side (the base quad has no tint, the overlay has one) waits here for the other, which comes later, after the
    // other directions' quads: they are told apart by the cell and the direction.
    #[derive(Clone, Copy)]
    struct Half {
        key: (i32, i32, i32, u8),
        solid_leaves: bool,
        tile: TileLayer,
        v: [TerrainVertex; 4],
        tint: Option<u32>,
        out: usize,
    }
    thread_local! {
        static WAIT: std::cell::RefCell<Vec<Half>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    if m::inside() {
        return std::ops::ControlFlow::Continue(());
    }
    let Some((x, y, z, facing, id)) = m::take_pending() else {
        return std::ops::ControlFlow::Continue(());
    };
    // Only a quad of that cell: a face with no quad of its own must not lend its note to the next one.
    let (fx, fy, fz) = (x as f32, y as f32, z as f32);
    let inside = v.iter().all(|p| {
        (fx - 0.01..=fx + 1.01).contains(&p.pos[0]) && (fy - 0.01..=fy + 1.01).contains(&p.pos[1]) && (fz - 0.01..=fz + 1.01).contains(&p.pos[2])
    });
    if !inside {
        return std::ops::ControlFlow::Continue(());
    }
    let key = (x, y, z, facing);
    let here = out as *const SectionMesh as usize;
    let me = Half { key, solid_leaves, tile, v: *v, tint, out: here };
    let (orphans, other) = WAIT.with_borrow_mut(|wait| {
        // Halves of other cells never found their partner (a pack with no overlay): they are drawn as they were.
        let mut orphans = Vec::new();
        let mut i = 0;
        while i < wait.len() {
            if (wait[i].key.0, wait[i].key.1, wait[i].key.2) != (x, y, z) || wait[i].out != here {
                orphans.push(wait.swap_remove(i));
            } else {
                i += 1;
            }
        }
        let other = wait.iter().position(|h| h.key == key && h.tint.is_some() != tint.is_some()).map(|i| wait.swap_remove(i));
        if other.is_none() {
            wait.push(me);
        }
        (orphans, other)
    });
    m::enter();
    for h in orphans.into_iter().filter(|h| h.out == here) {
        mesher.quad(out, h.solid_leaves, h.tile, h.v, h.tint);
    }
    let Some(other) = other else {
        m::leave();
        return std::ops::ControlFlow::Break(());
    };
    let (base, overlay) = if tint.is_none() { (me, other) } else { (other, me) };
    let top = match m::cached(id) {
        Some(layer) => layer,
        None => {
            let mut models = Vec::new();
            let found = mesher
                .render
                .model_catalog
                .models_at(BlockStateId(id), BlockPos::new(0, 0, 0), &mut models)
                .ok()
                .and_then(|()| models.iter().flat_map(|model| model.faces.iter()).find(|f| f.quad.facing == ModelDirection::Up).map(|f| f.layer));
            m::remember(id, found);
            found
        }
    };
    match (top, base.tile) {
        (Some(layer), TileLayer::Layer { translucent, opaque, .. }) => {
            // The base quad's shape and light, the grass top's sprite, the overlay's tinted colours.
            let mut merged = base.v;
            for (vertex, tinted) in merged.iter_mut().zip(overlay.v.iter()) {
                vertex.color = tinted.color;
            }
            let tile = TileLayer::Layer { layer: layer as u16, translucent, opaque };
            mesher.quad(out, base.solid_leaves, tile, merged, overlay.tint);
        }
        _ => {
            // No grass top to show: both quads as the game drew them.
            mesher.quad(out, base.solid_leaves, base.tile, base.v, base.tint);
            mesher.quad(out, overlay.solid_leaves, overlay.tile, overlay.v, overlay.tint);
        }
    }
    m::leave();
    std::ops::ControlFlow::Break(())
}
