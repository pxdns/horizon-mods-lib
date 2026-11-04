# Shield Statuses (Horizon native mod)

Your shield's state as a coloured line: ready, blocking, or disabled with the seconds left.

- `/shield` hides or shows it.

Ported from **Shield Statuses (shieldstatus)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/shield_statuses/`). It reads Horizon's own shield state (`Game::shield_cooldown`, the item in use and `Game::blocks_attacks`), which the hook-point contract does not promise, so it is **pinned to one Horizon commit**. It shows your own shield only: the original colours other players' shields, which needs their state and an entity drawing hook.

## Install

Download `shield_statuses-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=shield-statuses), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Shield Statuses. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/shield_statuses/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/shield_statuses/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_shield_statuses` and this crate's library |
| `mixins/shield_statuses/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/shield-statuses && cargo test          # the logic
python3 tools/mods.py check shield-statuses    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package shield-statuses  # target/shield_statuses-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-shield-statuses.yml` tests and packages the mod and publishes the release `shield-statuses-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
