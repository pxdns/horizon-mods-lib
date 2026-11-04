# Crystal Optimizer (Horizon native mod)

Makes End crystals you hit disappear at once on a server, and brings one back if the server did not break it.


Ported from **Marlow's Crystal Optimizer (marlowcrystal)** (MIT) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/crystal_optimizer/`). It puts a HEAD hook on `Game::send` (to see an attack on a crystal leave), one on `remove_remote_projectile` (the server removing a crystal), and a tick that takes the hit crystal out of `Game::crystals` and puts it back if it is still there after 30 ticks. None of these is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. It only changes what you see on a server, and only for crystals the client knows are not invulnerable. The original's prediction of placing crystals is not included. Some servers forbid client-side prediction mods; check the rules of the server you play on.

## Install

Download `crystal_optimizer-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=crystal-optimizer), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Crystal Optimizer. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/crystal_optimizer/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/crystal_optimizer/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_crystal_optimizer` and this crate's library |
| `mixins/crystal_optimizer/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/crystal-optimizer && cargo test          # the logic
python3 tools/mods.py check crystal-optimizer    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package crystal-optimizer  # target/crystal_optimizer-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-crystal-optimizer.yml` tests and packages the mod and publishes the release `crystal-optimizer-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
