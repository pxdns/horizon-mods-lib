# Ping View (Horizon native mod)

Shows your ping to the server: the latest reply, the average of the last 20 and the best and worst.

- `/ping` hides or shows it.

Ported from **Ping View (pingview)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/ping_view/`). Horizon keeps the connection's ping samples in a `SampleLogger` that has no getter, so the mod's module lives in `debug_chart.rs` and a HEAD hook on `SampleLogger::log` copies the samples of the one logger the game tells it about (the ping logger, found by its address). That is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**.

## Install

Download `ping_view-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=ping-view), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Ping View. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/ping_view/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/ping_view/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_ping_view` and this crate's library |
| `mixins/ping_view/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/ping-view && cargo test          # the logic
python3 tools/mods.py check ping-view    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package ping-view  # target/ping_view-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-ping-view.yml` tests and packages the mod and publishes the release `ping-view-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
