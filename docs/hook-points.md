# Hook points

The hook points are empty functions in Horizon's `crates/horizon-client/src/game/native_hooks.rs`. The game calls them at fixed
places; a native mod hooks them with HEAD injections. They keep their names and signatures from release to release and are only ever
added; each addition raises `NATIVE_HOOKS_API` (3 at the commit these projects were tested on).

| Hook | Called | These projects use it for |
|---|---|---|
| `native_frame(elapsed)` | Once a frame | Nothing yet |
| `native_tick()` | Once a tick, in every `Game` (check `native_window()`) | `info-hud`: speed from how far you moved |
| `native_world_lifecycle(…)` | World open, open failed, final close (API 3) | Nothing yet |
| `native_hud(batch, corners)` | The HUD, not under F1 | Every HUD mod: `corners.place` a box, draw with `native_fill` and `native_text` |
| `native_key(key, pressed)` | A key or mouse button, by vanilla name | `zoomify`, `simple-zoom` |
| `native_wheel(steps, claimed)` | Wheel steps while playing | The zoom mods claim them while zoomed |
| `native_command(command, ran)` | A chat command, before the server sees it | `/fps`, `/wp`, `/hud`… |
| `native_zoom(zoom)` | Each frame while playing | The zoom mods set factor, smoothing and turn scale |
| `native_minimap(minimap)` | Each frame | Nothing here; Horizon ships a minimap |
| `native_mods(mods)` | Options → Mods lists native mods | Every mod pushes its row |
| `native_settings(mod_id, buttons)` | A mod's settings page opens | Every mod pushes its buttons |
| `native_settings_click(mod_id, button)` | A button was clicked | Every mod flips its option |
| `native_panel_event(…)` | The item panel's clicks (API 2) | Nothing yet |

## Helpers the game gives a mod

`native_on`, `native_playing`, `native_window`, `native_corner_shown`, `native_text_width`, `native_fill`, `native_text`,
`native_player` (position, yaw, pitch, dimension), `native_frame_rate`, `native_clicks`, `native_mapping`, `native_mod_key`,
`native_chat`, `native_data_dir`, `native_setting`, `native_set_setting`, and the item and recipe helpers of API 2.

## Reading the game's own state

Some mods here (`armor-hud`, `potion-counter`, `totem-counter`, `effect-timer`, `apple-skin`, `pick-up-notifier`) still hang on the stable
`native_hud` and `native_tick` hook points, but they read what those hooks do not hand over: the inventory, armor slots, active effects and food,
straight from Horizon's `Game` struct. horizon-mixins puts a mod's helpers inside Horizon's `game` module, where those private fields are visible. That works, and
`tools/mods.py check` proves it compiles against the pinned commit, but it is **not covered by the hook-point contract**: if Horizon renames a field, these mods
stop compiling and must be updated. For that reason they carry no `hooks` entry in `compatibility.json`, only the Horizon commit they were tested on.

## What the hook points cannot do yet

No inventory, effects, incoming chat, camera, input injection or world data beyond the player's position. That is the reason
most of the [catalogue](../MODS.md) is marked *needs a hook point*: armour and potion HUDs need the inventory and effects, chat
mods need incoming chat, Freecam and Freelook need the camera. Raising a hook point is a change to Horizon, not to a mod; the
contract is described in `docs/native-mods.md` and `native-hooks.json` in the Horizon repository.
