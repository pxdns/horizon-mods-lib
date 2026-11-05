# Tweakeroo (Horizon native mod)

Four of Tweakeroo's tweaks: hand restock, hotbar cycle, fast right click and a tool break warning.


Ported from **Tweakeroo (tweakeroo)** (independent implementation from the mod's public description; no upstream code or assets (the original is LGPL-3.0)) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/tweakeroo/`). It reads the inventory and the held item and sets the game's use delay, and clicks slots with `Game::tweak_click`, the call Horizon's own inventory extras use. None of that is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. Restock and the cycle work when Horizon runs the world (single player, LAN host). **Only four of Tweakeroo's many tweaks are here**: flexible and accurate block placement, free camera (see Freecam), gamma override (built into Horizon), fast left click, the yeets and the rest are not. It was written from the original's public description, not its source.

## Install

Download `tweakeroo-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=tweakeroo), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Tweakeroo. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/tweakeroo/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/tweakeroo/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_tweakeroo` and this crate's library |
| `mixins/tweakeroo/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/tweakeroo && cargo test          # the logic
python3 tools/mods.py check tweakeroo    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package tweakeroo  # target/tweakeroo-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-tweakeroo.yml` tests and packages the mod and publishes the release `tweakeroo-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
