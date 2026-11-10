# Crosshair Indicator (Horizon native mod)

Shows corner brackets around your crosshair when you're aiming at a player.


Ported from **Crosshair Indicator (crosshairindicator) by SwimmingDog1 and MGC8** (CC0-1.0) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/crosshair_indicator/`). It asks the game what the crosshair is on with `native_crosshair_target` (**`NATIVE_HOOKS_API` 400**, added on the client-modpack-port branch of the Horizon fork) and draws in the HUD hook point, through the crosshair's own inverting blend. It is not part of upstream Horizon's hook points yet, so it needs a Horizon that has API 400.

## Install

Download `crosshair_indicator-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=crosshair-indicator), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Crosshair Indicator. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/crosshair_indicator/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/crosshair_indicator/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_crosshair_indicator` and this crate's library |
| `mixins/crosshair_indicator/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/crosshair-indicator && cargo test          # the logic
python3 tools/mods.py check crosshair-indicator    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package crosshair-indicator  # target/crosshair_indicator-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-crosshair-indicator.yml` tests and packages the mod and publishes the release `crosshair-indicator-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
