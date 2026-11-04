# F5 Split (Horizon native mod)

Separate keys for the back and the front third-person view, as a toggle or while held.


Ported from **F5 Split (f5split)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/f5_split/`). It sets the game's camera type (`Game::camera_type`), a private field, from the key hook, which is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**. Custom field of view per view and the original's smart return are not included.

## Install

Download `f5_split-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=f5-split), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → F5 Split. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/f5_split/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/f5_split/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_f5_split` and this crate's library |
| `mixins/f5_split/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/f5-split && cargo test          # the logic
python3 tools/mods.py check f5-split    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package f5-split  # target/f5_split-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-f5-split.yml` tests and packages the mod and publishes the release `f5-split-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
