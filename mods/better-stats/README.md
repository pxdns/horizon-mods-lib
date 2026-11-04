# Better Stats (Horizon native mod)

Search, rank and export your statistics from chat: the biggest entries of each kind, a search across all of them, and a text or CSV file.


Ported from **Better Statistics Screen (betterstats)** (independent implementation from the mod's public description; no upstream code or assets) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/better_stats/`). It reads the statistics Horizon keeps (`Game::stats`) and calls `load_stats`, which the hook-point contract does not promise, so it is **pinned to one Horizon commit**. It is the original's searching, ranking and exporting, as chat commands. The redesigned screen (a visual grid, rendered mobs, grouping, tabs of saved files and sharing) is not included; Search Stats adds the search box to Horizon's own screen. This mod was written from the original's public description, not its source, whose licence does not allow reuse.

## Install

Download `better_stats-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=better-stats), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Better Stats. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/better_stats/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/better_stats/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_better_stats` and this crate's library |
| `mixins/better_stats/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/better-stats && cargo test          # the logic
python3 tools/mods.py check better-stats    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package better-stats  # target/better_stats-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-better-stats.yml` tests and packages the mod and publishes the release `better-stats-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
