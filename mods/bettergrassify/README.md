# BetterGrassify (Horizon native mod)

Connected grass: a grass block's side shows the grass top where the neighbouring step down is grass too.


Ported from **BetterGrassify (bettergrass)** (Apache-2.0 for the idea; no upstream code is used (its sprite code is LGPL from another project)) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/bettergrassify/`). It lives in the chunk mesher (`crates/horizon-client-render/src/mesher.rs` and its `lighting.rs`) as two hooks: a HEAD hook on `lighting::ambient_occlusion`, which runs once per face with smooth lighting and sees the block, the cell beside it and the face, notes whether this is the side of a grass block with grass one step down; and a cancellable HEAD hook on `Mesher::quad`, which every face passes through. A grass side is two quads, the base (untinted) and the tinted overlay; the hook holds the first until the second arrives, drops the overlay and draws the base with the grass top's texture in the overlay's tinted colours. (It used to hook `Mesher::face`; that function now takes a tuple pattern as a parameter, which horizon-mixins cannot bind.) None of this is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. **I could not look at the result**: it is checked to compile and its state is tested, but how it renders was not seen. It works with smooth lighting on (the lighting hook only runs then). Only the "fancy" mode for plain grass blocks is done; snowy grass, the "fast" mode and better snow are not.

## Install

Download `bettergrassify-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=bettergrassify), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → BetterGrassify. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/bettergrassify/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/bettergrassify/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_bettergrassify` and this crate's library |
| `mixins/bettergrassify/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/bettergrassify && cargo test          # the logic
python3 tools/mods.py check bettergrassify    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package bettergrassify  # target/bettergrassify-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-bettergrassify.yml` tests and packages the mod and publishes the release `bettergrassify-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
