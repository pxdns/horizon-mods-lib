# Entity Texture Features (Horizon native mod)

Random texture variants and emissive layers for mobs, from the textures a resource pack provides.


Ported from **Entity Texture Features (etf)** (independent implementation from the mod's public description; no upstream code or assets) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/entity_texture_features/`). It lives in `crates/horizon-client/src/mobs/render.rs`: cancellable HEAD hooks on `body` (which texture a mob's body uses) and `layers` (its extra layers) that call the original and then change its answer. They are not part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. **I could not look at the result**: it is checked to compile and its logic is tested, but a mob with variants was not seen on screen. It covers numbered variants (`name2`, `name3`...) and `_e` emissive textures that sit in the resource pack's `textures/entity`; the original's `.properties` rule files (by biome, name, health, time), `optifine/random` folders, player skins and block entity textures are not included.

## Install

Download `entity_texture_features-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=entity-texture-features), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Entity Texture Features. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/entity_texture_features/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/entity_texture_features/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_entity_texture_features` and this crate's library |
| `mixins/entity_texture_features/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/entity-texture-features && cargo test          # the logic
python3 tools/mods.py check entity-texture-features    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package entity-texture-features  # target/entity_texture_features-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-entity-texture-features.yml` tests and packages the mod and publishes the release `entity-texture-features-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
