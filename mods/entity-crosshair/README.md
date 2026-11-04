# Entity Crosshair (Horizon native mod)

Replaces the crosshair with a style and colour of your choice, red over living entities.


Ported from **Entity Crosshair (EntityCrosshair)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/entity_crosshair/`). Its module lives in Horizon's `hud_layout.rs` (the render crate) and a RETURN modifier on `hud_layout::crosshair` swaps the crosshair sprite for the mod's own shape, keeping the attack indicator; the game tells it each frame what is under the crosshair. None of that is part of the stable hook-point contract, so it is **pinned to one Horizon commit**.

## Install

Download `entity_crosshair-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=entity-crosshair), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Entity Crosshair. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/entity_crosshair/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/entity_crosshair/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_entity_crosshair` and this crate's library |
| `mixins/entity_crosshair/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/entity-crosshair && cargo test          # the logic
python3 tools/mods.py check entity-crosshair    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package entity-crosshair  # target/entity_crosshair-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-entity-crosshair.yml` tests and packages the mod and publishes the release `entity-crosshair-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
