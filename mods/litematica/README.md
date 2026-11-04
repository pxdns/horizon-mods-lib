# Litematica (Horizon native mod)

Read .litematic schematics: info, and the material list of blocks you need.


Ported from **Litematica (litematica)** (independent implementation of the public .litematic file layout; no upstream code or assets (the original is LGPL-3.0)) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/litematica/`). It reads files and uses Horizon's NBT reader (`horizon_world::nbt`), which the hook-point contract does not promise, so it is **pinned to one Horizon commit**. It is only the reading half of Litematica: **loading a schematic as a ghost in the world, showing differences, and placing it are not included**, because they need an overlay renderer in the world and edit tools that Horizon's hooks do not offer. It reads the file layout from the format's public description, and a file in a newer version of the format may not read.

## Install

Download `litematica-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=litematica), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Litematica. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/litematica/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/litematica/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_litematica` and this crate's library |
| `mixins/litematica/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/litematica && cargo test          # the logic
python3 tools/mods.py check litematica    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package litematica  # target/litematica-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-litematica.yml` tests and packages the mod and publishes the release `litematica-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
