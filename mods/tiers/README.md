# Tiers (Horizon native mod)

Look up a player's tiers on a tier website with /tier. The website interface is a guess; lookups are off until you switch them on.


Ported from **Tiers (tiers)** (independent implementation; no upstream code or assets (the original is GPL-3.0)) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/tiers/`). It makes web requests with the HTTP client Horizon already ships (`ureq`) and prints the reply in chat; that and `Game::options` are not part of the stable hook-point contract, so the mod is **pinned to one Horizon commit**. **The website's interface is a guess**, made without being able to reach the site or read the original's GPL-3.0 source, so it may not work until you give `/tier url` the right address. The reply is read loosely: any object with a `tier` number, named by `gamemode`, `mode`, `kit` or `name`, or by its key under `rankings`/`tiers`/`gamemodes`, is listed as `mode: tier N`. Name tags, the tab list, the search screen and the original's kit auto-detection are not included: Horizon has no name tag hook. Lookups are off by default because each one tells the website who you looked up.

## Install

Download `tiers-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=tiers), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Tiers. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/tiers/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/tiers/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_tiers` and this crate's library |
| `mixins/tiers/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/tiers && cargo test          # the logic
python3 tools/mods.py check tiers    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package tiers  # target/tiers-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-tiers.yml` tests and packages the mod and publishes the release `tiers-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
