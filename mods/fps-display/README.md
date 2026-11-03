# FPS Display (Horizon native mod)

Frames per second in a corner of the screen, coloured by how smooth they are.

- `/fps` hides or shows it.

Ported from **FPS - Display (fpsdisplay)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/fps_display/`) that hooks only Horizon's native hook points, so the launcher builds it on any Horizon with
`NATIVE_HOOKS_API` 1 or later, together with other native mods.

## Install

Download `fps_display-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=fps-display), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → FPS Display. The [install guide](https://github.com/pxdns/horizon-mods-lib/blob/main/docs/install.md) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/fps_display/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/fps_display/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_fps_display` and this crate's library |
| `mixins/fps_display/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/fps-display && cargo test          # the logic
python3 tools/mods.py check fps-display    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package fps-display  # target/fps_display-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-fps-display.yml` tests and packages the mod and publishes the release `fps-display-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
