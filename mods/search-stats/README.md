# SearchStats (Horizon native mod)

Adds a search bar to the statistics screen to quickly find specific entries.


Ported from **SearchStats (searchstats) by TerminalMC** (Apache-2.0) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/search_stats/`). It hooks only Horizon's native hook points, so the launcher builds it on any Horizon with `NATIVE_HOOKS_API` 1 or later, together with other native mods.

## Install

Download `search_stats-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=search-stats), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → SearchStats. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
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
