# Anatomy of a mod

```text
mods/zoomify/
  Cargo.toml                       the Cargo library: points at runtime.rs
  Cargo.lock
  compatibility.json               pinned Horizon and horizon-mixins commits, hooks API
  LICENSE
  README.md
  mixins/zoomify/
    horizon-mixin.toml             module + HEAD injections
    runtime.rs                     logic, state and tests (one file)
    key.rs wheel.rs zoom.rs        one helper per hook
    list.rs settings.rs click.rs   the Options > Mods row and page
```

## `horizon-mixin.toml`

Declares one **module** (the file `runtime.rs`, added to the game's `game.rs` as `game::lib_<id>`) and one **injection** per hook:

```toml
[[module]]
id = "runtime"
file = "crates/horizon-client/src/game.rs"
name = "lib_zoomify"
source = "runtime.rs"

[[injection]]
id = "wheel"
kind = "HEAD"
target = "horizon_client::game::native_hooks::Game<'_>::native_wheel"
helper = "wheel.rs"
bindings = [
  { name = "game", ty = "&crate::game::Game<'_>", value = "&*self" },
  { name = "steps", ty = "f32", value = "steps" },
  { name = "claimed", ty = "&mut bool", value = "&mut *claimed" },
]
```

A HEAD injection runs the helper before the hook point's own body, which is empty. Helper names are prefixed with the mod's
id (`zoomify_wheel`) because every mod in a build ends up in the same module.

## `runtime.rs`

Pure logic: no game types. It holds the mod's state in a `thread_local!`, reads settings through a closure the helper passes in
(`|k| game.native_setting(ID, k)`), and carries its unit tests. That is what makes a native mod testable without the game.

## The helpers

Each helper is a few lines: check `game.native_on(ID)`, read what it needs from the game, call into `runtime.rs`, draw or
store the result. A mod that is switched off in Options → Mods does nothing until the next build leaves it out, so every helper
checks first.

## `compatibility.json` and the manifest

`tools/mods.py package` writes `horizon-native.json` into the `.hnmod` from `compatibility.json`:

```json
{"schema": 1, "id": "zoomify", "version": "1.0.0", "game": "26.3",
 "horizon": "ba6344a59cb9e755b730dcf02285fc0d33b97d3c",
 "mixins": "d9c83d7096b7ea5b83678da2efdfd6161f74997e", "hooks": 1}
```

The launcher validates it (`schema 1`, exact 40-character revisions, `hooks` not zero) before it builds anything and refuses
bundles with links, absolute paths or more than 512 entries.
