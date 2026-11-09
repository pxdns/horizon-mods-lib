# No Weather (Horizon native mod)

Keeps the sky clear in your own worlds: the game's rain and thunder never start. Servers keep their own weather.

- `/noweather` switches it on or off. The `Stops` button picks all weather or only thunder. It works where the game runs the world (single player and LAN hosts); a server's own weather is the server's.

A weather stopper written as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/no_weather/`). It hooks only Horizon's native hook points, so the launcher builds it on any Horizon with `NATIVE_HOOKS_API` 304 or later, together with other native mods.

## Install

Download `no_weather-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=no-weather), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → No Weather. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/no_weather/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/no_weather/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_no_weather` and this crate's library |
| `mixins/no_weather/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/no-weather && cargo test          # the logic
python3 tools/mods.py check no-weather    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package no-weather  # target/no_weather-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-no-weather.yml` tests and packages the mod and publishes the release `no-weather-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
