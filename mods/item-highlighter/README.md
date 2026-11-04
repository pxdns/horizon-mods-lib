# Item Highlighter (Horizon native mod)

Marks newly picked-up items with a twinkling star in your inventory.


Ported from **Item Highlighter (highlighter)** (independent implementation from the mod's public description; no upstream code or assets) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/item_highlighter/`). It puts a cancellable HEAD hook on `Game::draw_slot_item`, which draws one slot's item on a container screen: the hook draws the slot as Horizon would and then the star over it. It reads the inventory each tick to see what was picked up. Neither is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. Stars mark a kind of item, not one slot; the original's clearing on hover and its resource-pack look are not included. It was written from the original's public description, not its source, whose licence does not allow reuse.

## Install

Download `item_highlighter-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=item-highlighter), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Item Highlighter. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/item_highlighter/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/item_highlighter/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_item_highlighter` and this crate's library |
| `mixins/item_highlighter/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/item-highlighter && cargo test          # the logic
python3 tools/mods.py check item-highlighter    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package item-highlighter  # target/item_highlighter-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-item-highlighter.yml` tests and packages the mod and publishes the release `item-highlighter-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
