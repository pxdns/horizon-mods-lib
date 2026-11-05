# Clumps (Horizon native mod)

Groups nearby experience orbs into a single orb, which cuts the entity count and is collected at once.


Ported from **Clumps (clumps)** (MIT) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/clumps/`). It replaces `Game::merge_orbs` (in `game.rs`; it was `entities::merge_orbs` before Horizon moved it), the experience-orb merge Horizon runs in single player and as a LAN host, with a cancellable HEAD hook that merges every orb touching another (not only equal sizes at matching ids) into one orb whose value is the sum of theirs, so saving and loading keep the total. It is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. Experience on a multiplayer server is handled by the server, which this does not change. Unlike the original it does not remember the sizes it merged, so Mending repairs once from the total.

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
