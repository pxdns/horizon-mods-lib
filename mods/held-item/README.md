# Held Item (Horizon native mod)

Move, tilt and resize the item and hand you hold in first person: offsets, tilt and size, for either or both hands.

- `/helditem` switches it on or off. Every other option is a button on the mod page in Options > Mods that steps to the next choice.

A held-item editor like the ones PvP clients have (offsets, tilt and size for the hand and its item), written as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/held_item/`). It hooks only Horizon's native hook points, so the launcher builds it on any Horizon with `NATIVE_HOOKS_API` 369 or later, together with other native mods.

## Install

Download `held_item-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=held-item), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Held Item. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/held_item/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/held_item/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_held_item` and this crate's library |
| `mixins/held_item/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/held-item && cargo test          # the logic
python3 tools/mods.py check held-item    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package held-item  # target/held_item-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-held-item.yml` tests and packages the mod and publishes the release `held-item-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
