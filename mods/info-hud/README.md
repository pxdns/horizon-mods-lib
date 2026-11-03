# Info HUD (Horizon native mod)

Your speed, the way you face and your dimension in a corner of the screen, in the spirit of MiniHUD's info lines.

- `/info` hides or shows it.

Ported from **MiniHUD (info lines)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/info_hud/`) that hooks only Horizon's native hook points, so the launcher builds it on any Horizon with
`NATIVE_HOOKS_API` 1 or later, together with other native mods.

## Install

Download `info_hud-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=info-hud), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Info HUD. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/info_hud/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/info_hud/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_info_hud` and this crate's library |
| `mixins/info_hud/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/info-hud && cargo test          # the logic
python3 tools/mods.py check info-hud    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package info-hud  # target/info_hud-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-info-hud.yml` tests and packages the mod and publishes the release `info-hud-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
