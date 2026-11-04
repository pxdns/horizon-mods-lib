# Shulker Box Tooltip (Horizon native mod)

Shows what is inside a shulker box in its tooltip: totals by item, and a graphical 9 by 3 grid of the items.


Ported from **Shulker Box Tooltip (shulkerboxtooltip)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/shulker_box_tooltip/`). It adds a RETURN modifier to `Game::stack_tooltip` (the totals) and a cancellable HEAD hook on `bundle::tooltip_layout`, the function Horizon uses to lay out a bundle's item grid in its tooltip, which it answers for a shulker box with a 9 by 3 grid; Horizon then draws the real item icons and counts itself. It reads the item's container contents (`ItemStack::data`). None of this is part of the stable hook-point contract, so it is **pinned to one Horizon commit**.

## Install

Download `shulker_box_tooltip-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=shulker-box-tooltip), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Shulker Box Tooltip. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/shulker_box_tooltip/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/shulker_box_tooltip/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_shulker_box_tooltip` and this crate's library |
| `mixins/shulker_box_tooltip/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/shulker-box-tooltip && cargo test          # the logic
python3 tools/mods.py check shulker-box-tooltip    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package shulker-box-tooltip  # target/shulker_box_tooltip-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-shulker-box-tooltip.yml` tests and packages the mod and publishes the release `shulker-box-tooltip-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
