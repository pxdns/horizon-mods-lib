# SnappyTappy (Horizon native mod)

Last-input priority for movement keys: pressing D while holding A moves you right at once, then back to A when you let go.


Ported from **SnappyTappy (snappytappy) by cutebow** (MIT) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/snappy_tappy/`). It edits the `Game`'s held-key set before movement is read, and says ON or OFF through the game's action bar (`Game.titles`), which is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**.

## Install

Download `snappy_tappy-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=snappy-tappy), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → SnappyTappy. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/snappy_tappy/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/snappy_tappy/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_snappy_tappy` and this crate's library |
| `mixins/snappy_tappy/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/snappy-tappy && cargo test          # the logic
python3 tools/mods.py check snappy-tappy    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package snappy-tappy  # target/snappy_tappy-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-snappy-tappy.yml` tests and packages the mod and publishes the release `snappy-tappy-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
