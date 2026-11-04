# Crafting Tweaks (Horizon native mod)

Keys that rotate, balance and clear the crafting grid.


Ported from **Crafting Tweaks (craftingtweaks)** (independent implementation from the mod's public description; no upstream code or assets) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/crafting_tweaks/`). It reads the crafting grid (`Game::craft_grid`) and clicks slots with `Game::tweak_click`, the call Horizon's own inventory extras use for an ordinary click, so every action is the clicks a player could make. Neither is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. It works when Horizon runs the world (single player, LAN host), not on a server, and not for the original's buttons, right-click-for-a-full-stack on the result, hold-W transfer or compress keys. Stacks are matched by item id. It was written from the original's public description, not its source, whose licence does not allow reuse.

## Install

Download `crafting_tweaks-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=crafting-tweaks), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Crafting Tweaks. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/crafting_tweaks/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/crafting_tweaks/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_crafting_tweaks` and this crate's library |
| `mixins/crafting_tweaks/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/crafting-tweaks && cargo test          # the logic
python3 tools/mods.py check crafting-tweaks    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package crafting-tweaks  # target/crafting_tweaks-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-crafting-tweaks.yml` tests and packages the mod and publishes the release `crafting-tweaks-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
