# Custom HUD (Horizon native mod)

One box with the lines you pick: FPS, coordinates, facing and clicks per second, on the side you choose.

- `/hud` hides or shows it.

Ported from **CustomHUD (profile idea)** (MIT) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/custom_hud/`). It hooks only Horizon's native hook points, so the launcher builds it on any Horizon with `NATIVE_HOOKS_API` 1 or later, together with other native mods.

## Install

Download `custom_hud-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=custom-hud), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Custom HUD. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/custom_hud/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/custom_hud/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_custom_hud` and this crate's library |
| `mixins/custom_hud/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/custom-hud && cargo test          # the logic
python3 tools/mods.py check custom-hud    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package custom-hud  # target/custom_hud-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-custom-hud.yml` tests and packages the mod and publishes the release `custom-hud-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
