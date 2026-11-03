# Better Hurt Cam

![Better Hurt Cam icon](../../assets/icons/better-hurt-cam.svg)

Tone the camera shake down when you are hit, and switch the death tilt off.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/better-hurt-cam) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=better-hurt-cam)

## Use it

Nothing to press: when you take damage the camera shakes at a fraction of Horizon's own strength (the first button cycles 0, 25, 50, 75, 100 %), and the roll over when you die can be switched off.



Switch it on or off in **Options → Mods → Better Hurt Cam**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `fov::bob_matrix`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/better-hurt-cam/mixins/better_hurt_cam/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **BetterHurtCam (betterhurtcam), and the death-tilt part of No Death Animation** (see the upstream projects); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
