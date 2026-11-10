# Clumps (Horizon native mod)

Clumps XP orbs together to reduce lag.


Ported from **Clumps (clumps) by Jared (blamejared)** (MIT) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/clumps/`). It hooks only Horizon's native hook points, so the launcher builds it on any Horizon with `NATIVE_HOOKS_API` 1 or later, together with other native mods.

## Install

Download `clumps-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=clumps), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Clumps. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/clumps/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/clumps/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_clumps` and this crate's library |
| `mixins/clumps/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/clumps && cargo test          # the logic
python3 tools/mods.py check clumps    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package clumps  # target/clumps-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-clumps.yml` tests and packages the mod and publishes the release `clumps-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
