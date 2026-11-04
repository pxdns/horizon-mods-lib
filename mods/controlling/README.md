# Controlling (Horizon native mod)

Find key bindings by action or by key, and list conflicts and unbound actions.


Ported from **Controlling (controlling)** (MIT) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/controlling/`). It reads the key bindings (`Game::settings.keys`) and Horizon's list of mappings, which the hook-point contract does not promise, so it is **pinned to one Horizon commit**. It is chat commands, not the original's changes to the Key Binds screen, which Horizon draws in a layout function that cannot be extended; its search by action name is already built in.

## Install

Download `controlling-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=controlling), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Controlling. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/controlling/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/controlling/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_controlling` and this crate's library |
| `mixins/controlling/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/controlling && cargo test          # the logic
python3 tools/mods.py check controlling    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package controlling  # target/controlling-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-controlling.yml` tests and packages the mod and publishes the release `controlling-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
