"""BetterGrassify: a grass block's side shows the grass top texture where the block below the neighbouring cell is grass too. Pinned to one Horizon commit."""
from mods_chat import toml_text, native, G, common_native

GRASS = []

LIGHT_HELPER = '''// lighting::ambient_occlusion runs once for each face lit with smooth lighting: note, for the side of a grass block, whether the cell beside it steps down onto more grass.
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
'''

QUAD_HELPER = '''// Mesher::quad with Better Grass: the base quad of a grass side (untinted) and its overlay quad (tinted) become one quad with the grass top texture, in the overlay's colours.
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
'''

GRASS.append(dict(
    slug="bettergrassify", id="bettergrassify", name="BetterGrassify", title="BetterGrassify", raw=True,
    usage="Nothing to press: where a grass block's side is exposed next to a step down onto more grass, the side shows the grass top instead of the dirt strip with a grass fringe, so a grassy slope looks continuous (OptiFine's \"Better Grass: Fancy\").",
    description="Connected grass: a grass block's side shows the grass top where the neighbouring step down is grass too.",
    upstream="BetterGrassify (bettergrass)", upstream_licence="Apache-2.0 for the idea; no upstream code is used (its sprite code is LGPL from another project)",
    command=None, hooks=["light", "quad", "tick", "list", "settings", "click"],
    options=[("fancy", "Better grass on grass block sides", True)],
    modpath="horizon_client_render::mesher::lib_better_grassify",
    raw_note=("It lives in the chunk mesher (`crates/horizon-client-render/src/mesher.rs` and its `lighting.rs`) as two hooks: a HEAD hook on `lighting::ambient_occlusion`, which runs once per face with smooth lighting and sees the block, the cell beside it and the face, notes whether this is the side of a grass block with grass one step down; "
              "and a cancellable HEAD hook on `Mesher::quad`, which every face passes through. A grass side is two quads, the base (untinted) and the tinted overlay; the hook holds the first until the second arrives, drops the overlay and draws the base with the grass top's texture in the overlay's tinted colours. "
              "(It used to hook `Mesher::face`; that function now takes a tuple pattern as a parameter, which horizon-mixins cannot bind.) "
              "None of this is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. **I could not look at the result**: it is checked to compile and its state is tested, but how it renders was not seen. "
              "It works with smooth lighting on (the lighting hook only runs then). Only the \"fancy\" mode for plain grass blocks is done; snowy grass, the \"fast\" mode and better snow are not."),
    toml_text=toml_text("better_grassify", "1.0.0", "crates/horizon-client-render/src/mesher.rs", "lib_better_grassify", [
        dict(fields=[("id", '"light"'), ("kind", '"HEAD"'), ("target", '"horizon_client_render::mesher::lighting::ambient_occlusion"'), ("helper", '"light.rs"')],
             bindings=[("pad", "&Padded", "pad"), ("info", "&[Info]", "info"), ("own", "&Info", "own"), ("pos", "[i32; 3]", "pos"), ("face", "D", "face")]),
        dict(fields=[("id", '"quad"'), ("kind", '"HEAD"'), ("cancellable", "true"), ("target", '"horizon_client_render::mesher::Mesher<\'_>::quad"'), ("helper", '"quad.rs"')],
             bindings=[("mesher", "&Mesher<'_>", "&*self"), ("out", "&mut SectionMesh", "out"), ("solid_leaves", "bool", "solid_leaves"), ("tile", "TileLayer", "tile"),
                       ("v", "&[TerrainVertex; 4]", "&v"), ("tint", "Option<u32>", "tint")]),
        native("tick", "native_tick", [G]),
    ] + common_native(False), visibility="pub"),
    runtime_extra=r'''
use std::cell::{Cell, RefCell as StateCell};
use std::collections::BTreeMap;

/// What the lighting hook notes for the face being drawn: the cell (x, y, z), the face's direction and the block state id.
pub type Pending = (i32, i32, i32, u8, u32);

thread_local! {
    static ON: Cell<bool> = const { Cell::new(true) };
    /// Set by the lighting hook for a grass side with grass one step down, taken by the quad hook that follows.
    static PENDING: Cell<Option<Pending>> = const { Cell::new(None) };
    /// Whether the quad hook is calling the original `quad` itself, which it lets through.
    static INSIDE: Cell<bool> = const { Cell::new(false) };
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

/// Notes (or clears) what the face about to be drawn is.
pub fn set_pending(note: Option<Pending>) {
    PENDING.with(|c| c.set(note));
}

/// The note for the quad being drawn, once.
pub fn take_pending() -> Option<Pending> {
    PENDING.with(Cell::take)
}

/// The quad hook is about to call the original itself.
pub fn enter() {
    INSIDE.with(|c| c.set(true));
}

pub fn leave() {
    INSIDE.with(|c| c.set(false));
}

pub fn inside() -> bool {
    INSIDE.with(Cell::get)
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
    fn a_note_is_taken_once() {
        set_pending(Some((1, 2, 3, 4, 5)));
        assert_eq!(take_pending(), Some((1, 2, 3, 4, 5)));
        assert_eq!(take_pending(), None, "a quad must not reuse the note of the one before");
        set_pending(Some((0, 0, 0, 0, 1)));
        set_pending(None);
        assert_eq!(take_pending(), None, "the next face clears a stale note");
    }

    #[test]
    fn the_guard_lets_the_originals_own_call_through() {
        assert!(!inside());
        enter();
        assert!(inside());
        leave();
        assert!(!inside());
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
        "light": LIGHT_HELPER,
        "quad": QUAD_HELPER,
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
