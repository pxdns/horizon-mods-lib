# Pearl Optimizer (Horizon native mod)

Shows your thrown ender pearl immediately on a server, before the server's pearl arrives.


Ported from **Pearl Optimizer (pearloptimizer)** (independent implementation from the mod's public description; no upstream code or assets) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/pearl_optimizer/`). It puts a HEAD hook on `Game::send` (to see the throw leave), one on `add_projectile` (the server's pearl arriving) and a tick that adds a pearl without an owner to the projectile list, the way Horizon adds a server's pearl, with the throw's speed and direction. None of these is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. Because the local pearl has no owner it never teleports or hurts you; it only flies. It was written from the original's public description, not its source, whose licence does not allow reuse. Some servers forbid client-side prediction mods; check the rules of the server you play on.

## Install

Download `pearl_optimizer-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=pearl-optimizer), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Pearl Optimizer. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/pearl_optimizer/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/pearl_optimizer/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_pearl_optimizer` and this crate's library |
| `mixins/pearl_optimizer/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/pearl-optimizer && cargo test          # the logic
python3 tools/mods.py check pearl-optimizer    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package pearl-optimizer  # target/pearl_optimizer-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-pearl-optimizer.yml` tests and packages the mod and publishes the release `pearl-optimizer-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
