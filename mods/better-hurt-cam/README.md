# Better Hurt Cam (Horizon native mod)

Tone the camera shake down when you are hit, and switch the death tilt off.


Ported from **BetterHurtCam (betterhurtcam), and the death-tilt part of No Death Animation** (see the upstream projects) as an independent implementation for Horizon 26.3;
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/better_hurt_cam/`). It replaces Horizon's camera-bob function (`Game::bob_matrix`) with a cancellable HEAD hook that rebuilds the same matrix with a weaker hurt tilt, which is not part of the stable hook-point contract, so it is **pinned to one Horizon commit**.

## Install

Download `better_hurt_cam-1.0.0.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q=better-hurt-cam), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → Better Hurt Cam. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/better_hurt_cam/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/better_hurt_cam/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_better_hurt_cam` and this crate's library |
| `mixins/better_hurt_cam/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/better-hurt-cam && cargo test          # the logic
python3 tools/mods.py check better-hurt-cam    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package better-hurt-cam  # target/better_hurt_cam-1.0.0.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-better-hurt-cam.yml` tests and packages the mod and publishes the release `better-hurt-cam-v1.0.0`
with the `.hnmod` and its SHA-256.

License: MIT.
