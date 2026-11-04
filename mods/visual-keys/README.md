# VisualKeys (Horizon native mod)

An on-screen keyboard that shows what every key is bound to and which keys are free.

- `/keyboard` hides or shows it.

Ported from **VisualKeys (visualkeys)** (independent implementation from the mod's public description; no upstream code or assets) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/visual_keys/`). It reads the key bindings (`Game::settings.keys`) and Horizon's list of mappings, which the hook-point contract does not promise, so it is **pinned to one Horizon commit**. It shows the keyboard on the HUD only: the original also draws it inside the controls screen, shows Malilib bindings and collapses categories, which need a hook in the options screen. Labels fit only on the wider keys. It was written from the original's public description, not its source, whose licence does not allow reuse.

## Install

Download `visual_keys-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=visual-keys), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → VisualKeys. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/visual_keys/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/visual_keys/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_visual_keys` and this crate's library |
| `mixins/visual_keys/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/visual-keys && cargo test          # the logic
python3 tools/mods.py check visual-keys    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package visual-keys  # target/visual_keys-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-visual-keys.yml` tests and packages the mod and publishes the release `visual-keys-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
