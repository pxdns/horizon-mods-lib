# Search Stats (Horizon native mod)

Adds a search box to the statistics screen that filters the Items and Mobs lists as you type.


Ported from **SearchStats (searchstats)** (Apache-2.0) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/search_stats/`). It replaces the Items list builder of Horizon's statistics screen (`stat_items`, rebuilt from its source with the filter added) with a cancellable HEAD hook, adds a RETURN modifier to the Mobs list builder (`stat_mobs`), and draws the box from a HEAD hook on `draw_stats_footer`, in `game/stats_screen.rs`. Typing arrives through the stable key hook as key names, so only the US letters, digits, space and minus are read. None of these functions is part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. The General tab is not filtered: its rows are drawn inline in the screen's drawing function, which this does not replace.

## Install

Download `search_stats-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=search-stats), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Search Stats. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/search_stats/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/search_stats/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_search_stats` and this crate's library |
| `mixins/search_stats/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/search-stats && cargo test          # the logic
python3 tools/mods.py check search-stats    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package search-stats  # target/search_stats-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-search-stats.yml` tests and packages the mod and publishes the release `search-stats-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
