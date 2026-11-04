# Proposal: screen, slot and map hook points for Horizon native mods (`NATIVE_HOOKS_API` 4)

Status: draft, for the Horizon maintainers. Written against Horizon `ba6344a` (`NATIVE_HOOKS_API` 3) from the experience of
porting 186 Fabric mods in [horizon-mods-lib](https://github.com/pxdns/horizon-mods-lib). Nothing here is implemented; it is a
design to discuss.

## Problem

Horizon's stable hook points reach the HUD, keys, the wheel, commands, the zoom, the minimap's shape, settings pages and the
recipe-viewer panel. They do not reach **screens**. A native mod cannot open a screen of its own, cannot add a widget to a screen
Horizon draws, and cannot draw on a slot. Everything else a mod might do there has to be done by replacing a whole Horizon
function with a cancellable HEAD hook, which pins the mod to one Horizon commit and, for functions that use `?` or closures,
is refused by horizon-mixins.

What this blocks today (each is on the catalogue with this as its reason):

| Wanted | Mods | Needs |
|---|---|---|
| A scrollable full-screen world map | Xaero's World Map | A mod-owned screen, terrain colours |
| A better or searchable Statistics screen | Better Stats, Search Stats | A mod-owned screen (or a text box on Horizon's) |
| A Controls screen with search, a second binding per action, a keyboard picture | Controlling, Multi Key Bindings, VisualKeys | A mod-owned screen reading and writing the key map |
| Buttons on the crafting grid | Crafting Tweaks | Widgets on a container screen |
| Highlighting new or chosen slots, durability bars on slots | Item Highlighter | Drawing on a slot |
| Waypoint markers and radar dots on the minimap | Xaero's Minimap | A draw hook over the minimap |
| A player's head beside their chat line or name | Chat Heads | A skin-head draw helper |

(Not covered here, and different in kind: Clumps, crystal and pearl optimizers, ETF, BetterGrassify, Tiers and a Via-style
protocol translator need entity, mesher, network and packet hooks. A `native_packet_in` / `native_packet_out` pair would be a
separate proposal.)

## Precedent

API 2 added the item panel beside container screens (`native_panel_items`, `native_panel_event`, `game/native_panel.rs`). Its
shape is the right one: the **mod says what to show, the game draws and pages it and reports what the player did**. The mod holds
no game references across frames. This proposal follows it.

## Design

All hook points are added with empty default bodies in `game/native_hooks.rs`, each with one cheap check when no mod uses
them, no `cfg` and no macros in the bodies (`tests::hook_points_stay_hookable`), and raise `NATIVE_HOOKS_API` to 4.

### 1. A screen a mod owns

A new `Screen::Native` variant, drawn and fed by the mod.

```rust
/// What a native mod's screen is called and how it behaves.
pub(crate) struct NativeScreenInfo {
    pub(crate) id: String,        // unique within the mod
    pub(crate) title: String,
    /// The world keeps ticking behind it (a map) or pauses in single player (a statistics page).
    pub(crate) pauses: bool,
}

/// Declares the mod's screens; asked when the mod list is built.
fn native_screens(&mut self, mod_id: &str, screens: &mut Vec<NativeScreenInfo>);
/// Draws screen `screen` of mod `mod_id` for a window of `size` GUI pixels; `cursor` is the pointer.
fn native_screen_draw(&mut self, mod_id: &str, screen: &str, batch: &mut HudBatch, size: [f32; 2], cursor: [f32; 2]);
/// Something the player did on it.
fn native_screen_event(&mut self, mod_id: &str, screen: &str, event: &NativeScreenEvent);

pub(crate) enum NativeScreenEvent {
    Click { x: f32, y: f32, button: u8, pressed: bool },
    Drag { x: f32, y: f32, dx: f32, dy: f32, button: u8 },
    Scroll { x: f32, y: f32, dy: f32 },
    Key { key: String, pressed: bool, repeat: bool },   // vanilla names, as native_key
    Text(String),                                       // typed characters (a search box)
    Closed,                                             // Escape, or the mod closed it
}
```

Helpers (stable, with the existing `native_*` helpers): `native_open_screen(mod_id, screen)`, `native_close_screen()`.

Game side (touch points at `ba6344a`): the `Screen` enum (`game.rs:567`), the draw `match` (`game.rs:6651`), pointer move
(`game.rs:13514`), wheel (`game.rs:13621`), mouse buttons (`game.rs:13851`), and Escape handling (`game.rs:12843`). `Screen::Native`
behaves like `Screen::Stats`: the pointer is released, the HUD is hidden as for any screen, `native_playing()` is false, and
`pauses` decides whether the single-player clock stops. Escape sends `Closed` and returns to `Screen::Playing`.

The drawing API is what mods already use (`native_fill`, `native_text`, `native_text_width`) plus three small ones:
`native_sprite(batch, name, rect, argb)` for atlas sprites (widget textures, the button and slot sprites), `native_item(batch,
stack, x, y)` for an item icon (Horizon's `draw_item`, which needs `&mut self` today), and `native_scissor(batch, rect)` so a map
or list can clip to its window.

### 2. Reading what the map needs

A world map needs terrain colours without Horizon exposing the world. One read-only helper keeps the surface small:

```rust
/// Fills `out` (w × h, row-major, argb) with the top-down colour of each loaded column from (x0, z0), 0 where nothing is loaded.
fn native_surface_colors(&self, x0: i32, z0: i32, w: u32, h: u32, layer: NativeLayer, out: &mut [u32]);
pub(crate) enum NativeLayer { Surface, Below(i32) }
```

It reuses `horizon_client_render::minimap::{Palette, Layer}` and the chunk data the minimap already reads, so no new colour logic
is needed; unloaded columns are 0 and the mod caches what it has seen. Also `native_dimension()` (the id is in `NativePlayer`
already).

### 3. Slots

```rust
/// After a slot's item is drawn on a container screen: `index` is the slot's number, `rect` its 16 × 16 item area.
fn native_slot_draw(&mut self, batch: &mut HudBatch, index: u32, stack: &NativeStack, rect: [f32; 4]);
```

Called once per visible slot (inventory, creative, containers). With no mod it is one check per frame, not per slot. This is all
Item Highlighter-style mods and durability or count overlays need, with `native_fill`/`native_text`.

### 4. Widgets on Horizon's container screens

Follow the panel: the mod returns buttons, Horizon draws and hit-tests them and reports the click.

```rust
/// Extra buttons for the open container screen; `kind` is "inventory", "crafting", "chest", … and `window` its rectangle.
fn native_screen_buttons(&mut self, mod_id: &str, kind: &str, window: [f32; 4], buttons: &mut Vec<NativeButton>);
fn native_screen_button_clicked(&mut self, mod_id: &str, kind: &str, id: u32);
pub(crate) struct NativeButton { pub(crate) id: u32, pub(crate) rect: [f32; 4], pub(crate) label: String, pub(crate) tooltip: String }
```

### 5. The minimap

```rust
/// Over the minimap Horizon drew: `rect` is where it is on screen, `to_screen` maps a world (x, z) to a point in it.
fn native_minimap_overlay(&mut self, batch: &mut HudBatch, rect: [f32; 4], view: &NativeMinimapView);
pub(crate) struct NativeMinimapView { pub(crate) x: f64, pub(crate) z: f64, pub(crate) yaw: f32, pub(crate) rotate: bool, pub(crate) blocks_per_pixel: f32, pub(crate) round: bool }
```

The mod does the projection from those numbers (they are the `View` the minimap already composes); waypoint markers are then
`native_fill`s and entity dots come from an optional `native_entities_near(radius) -> Vec<NativeEntityDot>` (kind, x, z, hostile).

### 6. Heads

`native_head(batch, name_or_uuid, x, y, size)`: draws the 8 × 8 face and hat layer from the skin Horizon already loads for the
tab list and name tags, or the default skin. This is what Chat Heads needs beside a chat line (combined with the existing chat
hook points).

## Compatibility rules

- Added only; no existing name, parameter or call site changes.
- Empty default bodies, no `cfg`, no macros; one check when nothing uses them. Add each to `tests::hook_points_stay_hookable`.
- The launcher's `hooks` field (`"hooks": 4`) makes a bundle that uses only these build on any Horizon whose API is at least 4,
  exactly as for 2 and 3.
- Mods never hold references into the game between calls; screens keep their state in the mod's own module.

## Suggested order

1. `Screen::Native` with draw/event/open/close and `native_sprite`, `native_item`, `native_scissor` (unlocks Better/Search Stats, Controls
   screens, a map screen with no terrain).
2. `native_surface_colors` (the map's terrain).
3. `native_slot_draw`.
4. `native_screen_buttons`.
5. `native_minimap_overlay`, `native_entities_near`, `native_head`.

Each step is independently useful and independently releasable. Steps 1 and 2 together are enough for a Xaero's-World-Map-style
screen; the first mods I would write against them in horizon-mods-lib are a world map (`M` opens it, drag to pan, wheel to zoom,
waypoints from the existing `waypoints` mod drawn on it) and a Controls search screen.

## Open questions

- Should a mod-owned screen be allowed over the pause menu, or only from `Playing`?
- Is one `Screen::Native` enough, or should `Screen` carry the mod id and screen id so Escape handling and the debug overlay can
  say which screen is open?
- How much of the sprite atlas should `native_sprite` expose (all of it, or only `gui/sprites/*`)?
- Whether `native_slot_draw` should receive a mutable batch before or after the count text.
