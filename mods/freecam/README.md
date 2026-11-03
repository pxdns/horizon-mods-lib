# Freecam (Horizon native mod)

Detach the camera and fly it around while your player stands still.


Ported from **Freecam (freecam)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/freecam/`). It hooks Horizon's mouse-look function (`turn_player`) and render camera (`camera_view`) with cancellable HEAD hooks, and reads and edits the held-key sets of the `Game`, none of which is part of the stable hook-point contract, so it is **pinned to one Horizon commit**. It assumes the default W A S D, Space and Shift movement keys. Your clicks still act as your player's while the camera is detached.

## Install

Download `freecam-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=freecam), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Freecam. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/freecam/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/freecam/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_freecam` and this crate's library |
| `mixins/freecam/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/freecam && cargo test          # the logic
python3 tools/mods.py check freecam    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package freecam  # target/freecam-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-freecam.yml` tests and packages the mod and publishes the release `freecam-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
