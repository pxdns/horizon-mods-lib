# Apple Skin (Horizon native mod)

Your hunger and saturation as numbers, and what the food in your hand would restore.

- `/food` hides or shows it.

Ported from **AppleSkin (appleskin)** (The Unlicense) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/apple_skin/`). It uses the stable HUD hook point but **reads Horizon's own game state** (inventory, armor, effects, food), which the hook-point contract does not promise. It is therefore **pinned to one Horizon commit** (`ba6344a`) and has to be re-checked whenever Horizon changes those fields; `tools/mods.py check` does that.

## Install

Download `apple_skin-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=apple-skin), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Apple Skin. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/apple_skin/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/apple_skin/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_apple_skin` and this crate's library |
| `mixins/apple_skin/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/apple-skin && cargo test          # the logic
python3 tools/mods.py check apple-skin    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package apple-skin  # target/apple_skin-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-apple-skin.yml` tests and packages the mod and publishes the release `apple-skin-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
