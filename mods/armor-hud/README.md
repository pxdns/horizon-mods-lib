# Armor HUD (Horizon native mod)

Your armor pieces and how much durability each has left, in a corner of the screen, and your armor value on the armor bar, with extra rows past 20 points.

- `/armor` hides or shows it.

Ported from **uku's Armor HUD and Detail Armor Bar Reconstructed** (see the upstream projects) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/armor_hud/`). It uses the stable HUD hook point but **reads Horizon's own game state** (inventory, armor, effects, food), which the hook-point contract does not promise. It is therefore **pinned to one Horizon commit** (`ba6344a`) and has to be re-checked whenever Horizon changes those fields; `tools/mods.py check` does that.

## Install

Download `armor_hud-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=armor-hud), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Armor HUD. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/armor_hud/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/armor_hud/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_armor_hud` and this crate's library |
| `mixins/armor_hud/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/armor-hud && cargo test          # the logic
python3 tools/mods.py check armor-hud    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package armor-hud  # target/armor_hud-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-armor-hud.yml` tests and packages the mod and publishes the release `armor-hud-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
