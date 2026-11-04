"""BetterGrassify: a grass block's side shows the grass top texture where the block below the neighbouring cell is grass too. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

GRASS = []

FACE_HELPER = '''// Mesher::face with Better Grass: the side of a grass block next to a step down to more grass draws the grass top texture.
fn better_grassify_face(
    mesher: &Mesher<'_>,
    pad: &Padded,
    tints: &mut Tints<'_>,
    out: &mut SectionMesh,
    pos: [i32; 3],
    origin: [f32; 3],
    face: &BakedFace,
    ao: bool,
    state: &Info,
) -> std::ops::ControlFlow<()> {
    use lib_better_grassify as m;
    if !m::on() || !matches!(state.tints[0], Tint::GrassBlock) {
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
    let id = pad.blocks[pi(x, y, z)];
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
'''

GRASS.append(dict(
    slug="bettergrassify", id="bettergrassify", name="BetterGrassify", title="BetterGrassify", raw=True,
    usage="Nothing to press: where a grass block's side is exposed next to a step down onto more grass, the side shows the grass top instead of the dirt strip with a grass fringe, so a grassy slope looks continuous (OptiFine's \"Better Grass: Fancy\").",
    description="Connected grass: a grass block's side shows the grass top where the neighbouring step down is grass too.",
    upstream="BetterGrassify (bettergrass)", upstream_licence="Apache-2.0 for the idea; no upstream code is used (its sprite code is LGPL from another project)",
    command=None, hooks=["face", "tick", "list", "settings", "click"],
    options=[("fancy", "Better grass on grass block sides", True)],
    modpath="horizon_client_render::mesher::lib_better_grassify",
    raw_note=("It lives in the chunk mesher (`crates/horizon-client-render/src/mesher.rs`): a cancellable HEAD hook on `Mesher::face`, rebuilt from the function's own source with one change, the texture layer, so it draws the grass top texture on the base quad of a grass block's side and drops the tinted overlay quad. "
              "`face` is not part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. **I could not look at the result**: it is checked to compile and its logic is tested, but how it renders was not seen. "
              "Only the \"fancy\" mode for plain grass blocks is done; snowy grass, the \"fast\" mode and better snow are not."),
    toml_text=toml_text("better_grassify", "1.0.0", "crates/horizon-client-render/src/mesher.rs", "lib_better_grassify", [
        dict(fields=[("id", '"face"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_render::mesher::Mesher<\'_>::face"'), ("helper", '"face.rs"')],
             bindings=[("mesher", "&Mesher<'_>", "&*self"), ("pad", "&Padded", "pad"), ("tints", "&mut Tints<'_>", "tints"), ("out", "&mut SectionMesh", "out"),
                       ("pos", "[i32; 3]", "pos"), ("origin", "[f32; 3]", "origin"), ("face", "&BakedFace", "face"), ("ao", "bool", "ao"), ("state", "&Info", "state")]),
        native("tick", "native_tick", [G]),
    ] + common_native(False), visibility="pub"),
    runtime_extra=r'''
use std::cell::{Cell, RefCell as StateCell};
use std::collections::BTreeMap;

thread_local! {
    static ON: Cell<bool> = const { Cell::new(true) };
    /// Per block state id: the grass top texture layer, or none when the model has no up face.
    static TOP: StateCell<BTreeMap<u32, Option<u32>>> = const { StateCell::new(BTreeMap::new()) };
}

/// Sets whether the mesher uses Better Grass (the game tells it each tick).
pub fn configure(on: bool) {
    ON.with(|v| v.set(on));
}

pub fn on() -> bool {
    ON.with(Cell::get)
}

/// The remembered top layer of a state: nothing remembered yet, or Some(None) when the model has no up face.
pub fn cached(id: u32) -> Option<Option<u32>> {
    TOP.with_borrow(|t| t.get(&id).copied())
}

pub fn remember(id: u32, layer: Option<u32>) {
    TOP.with_borrow_mut(|t| {
        t.insert(id, layer);
    });
}

/// Forgets the layers (a resource reload changes them).
pub fn forget() {
    TOP.with_borrow_mut(BTreeMap::clear);
}
''',
    tests=r'''
    #[test]
    fn the_switch_works() {
        configure(false);
        assert!(!on());
        configure(true);
        assert!(on());
    }

    #[test]
    fn a_states_top_layer_is_remembered() {
        forget();
        assert_eq!(cached(9), None);
        remember(9, Some(42));
        assert_eq!(cached(9), Some(Some(42)));
        remember(10, None);
        assert_eq!(cached(10), Some(None), "a state without an up face is remembered too");
        forget();
        assert_eq!(cached(9), None);
    }
''',
    helpers={
        "face": FACE_HELPER,
        "tick": '''// Once a tick: tell the mesher what the player switched on.
fn bettergrassify_tick(game: &mut crate::game::Game<'_>) {
    use horizon_client::mesher::lib_better_grassify as m;
    if !game.native_window() {
        return;
    }
    m::configure(game.native_on(m::ID) && m::is(|k| game.native_setting(m::ID, k), "fancy"));
}
''',
    },
))
