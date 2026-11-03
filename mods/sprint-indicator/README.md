# Sprint Indicator (Horizon native mod)

Shows Sprinting, Walking or Sneaking in a corner of the screen while you move.

- `/sprint` hides or shows it.

Ported from **Sprint Indicator (sprintindicator)** (see the upstream project) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/sprint_indicator/`) that hooks only Horizon's native hook points, so the launcher builds it on any Horizon with
`NATIVE_HOOKS_API` 1 or later, together with other native mods.

## Install

Download `sprint_indicator-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=sprint-indicator), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Sprint Indicator. The [install guide](https://github.com/pxdns/horizon-mods-lib/blob/main/docs/install.md) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/sprint_indicator/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/sprint_indicator/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_sprint_indicator` and this crate's library |
| `mixins/sprint_indicator/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/sprint-indicator && cargo test          # the logic
python3 tools/mods.py check sprint-indicator    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package sprint-indicator  # target/sprint_indicator-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-sprint-indicator.yml` tests and packages the mod and publishes the release `sprint-indicator-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
