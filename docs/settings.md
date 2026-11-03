# Settings and storage

A native mod keeps its settings in files, one file per key, in `horizon-mods/.horizon-mod-data/<id>/` next to the game's options.
`game.native_setting(id, key)` reads one (trimmed), `game.native_set_setting(id, key, value)` writes one.

These projects store booleans as `true` and `false`; anything else reads as the option's default. Zoom mods also store `level`
(one of 2, 3, 4, 6, 8, 12, 16). Waypoints stores every waypoint in one file called `waypoints`, one tab-separated line each
(`name x y z dimension`); a line that does not read is skipped, not fatal.

Keys are different: they belong to the game's controls. A mod lists keys in its Options → Mods row (`keys` in `NativeMod`:
name, label, default) and reads the player's choice with `native_mod_key`. Both zoom mods list one key.

State that is not a setting (the zoom while it is on, the last position for speed, the tracked waypoint) lives in a `thread_local!`
in `runtime.rs` and is gone when the game closes.
