# InvMove (Horizon native mod)

Keep walking, jumping and sneaking while your inventory is open.


Ported from **InvMove (invmove)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/invmove/`). It puts a cancellable HEAD hook on `Game::tick`, which runs the tick again with the screen set to the in-game one while a movement key is held and your inventory is open, and puts the screen back afterwards. Only the survival inventory is covered, not chat, the creative inventory (its search box takes typing) or other screens. `Game::tick` is not part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**.

## Install

Download `invmove-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=invmove), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → InvMove. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/invmove/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/invmove/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_invmove` and this crate's library |
| `mixins/invmove/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/invmove && cargo test          # the logic
python3 tools/mods.py check invmove    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package invmove  # target/invmove-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-invmove.yml` tests and packages the mod and publishes the release `invmove-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
