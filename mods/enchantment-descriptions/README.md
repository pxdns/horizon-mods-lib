# Enchantment Descriptions (Horizon native mod)

Adds a line to enchanted items' tooltips saying what each enchantment does.


Ported from **Enchantment Descriptions (enchdesc)** (see the upstream project; the descriptions are written for this mod) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/enchantment_descriptions/`). It adds a RETURN modifier to `Game::stack_tooltip` and reads the item's enchantments (`ItemStack::data`), neither of which is part of the stable hook-point contract, so it is **pinned to one Horizon commit**. The lines are added under the tooltip, named, rather than between the enchantment lines, so they never land under the wrong enchantment. Enchantments from data packs have no description and get none.

## Install

Download `enchantment_descriptions-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=enchantment-descriptions), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Enchantment Descriptions. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/enchantment_descriptions/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/enchantment_descriptions/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_enchantment_descriptions` and this crate's library |
| `mixins/enchantment_descriptions/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/enchantment-descriptions && cargo test          # the logic
python3 tools/mods.py check enchantment-descriptions    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package enchantment-descriptions  # target/enchantment_descriptions-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-enchantment-descriptions.yml` tests and packages the mod and publishes the release `enchantment-descriptions-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
