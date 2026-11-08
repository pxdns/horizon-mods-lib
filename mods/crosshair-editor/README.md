# Crosshair Editor (Horizon native mod)

Replace the crosshair with your own: shape, size, gap, thickness, colour (or rainbow), an outline and a centre dot.

- `/crosshair` switches it on or off. Every other option is a button on the mod page in Options > Mods that steps to the next choice.

A crosshair editor like the ones PvP clients have, written as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/crosshair_editor/`). It hooks only Horizon's native hook points, so the launcher builds it on any Horizon with `NATIVE_HOOKS_API` 139 or later, together with other native mods.

## Install

Download `crosshair_editor-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=crosshair-editor), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Crosshair Editor. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/crosshair_editor/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/crosshair_editor/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_crosshair_editor` and this crate's library |
| `mixins/crosshair_editor/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/crosshair-editor && cargo test          # the logic
python3 tools/mods.py check crosshair-editor    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package crosshair-editor  # target/crosshair_editor-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-crosshair-editor.yml` tests and packages the mod and publishes the release `crosshair-editor-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
