# Install a mod

Native mods are built into your game, so installing one means building one. The first build downloads Rust and can take 20 to
30 minutes; later builds are faster ([Horizon's native mods guide](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md)
has the measured numbers).

## The quick way: all of them at once

One download has every mod, unpacked and ready to build:

```sh
curl -LO https://github.com/pxdns/horizon-mods-lib/releases/download/all-mods/horizon-mods-all.zip
curl -LO https://github.com/pxdns/horizon-mods-lib/releases/download/all-mods/horizon-mods-all.zip.sha256
sha256sum -c horizon-mods-all.zip.sha256
unzip horizon-mods-all.zip -d horizon-mods && cd horizon-mods
./install.sh MyInstance          # Windows: powershell -ExecutionPolicy Bypass -File install.ps1 MyInstance
```

`install.sh` asks you to type `yes` (these are native mods: source compiled into your game, running with your rights) and then runs
`horizon-launcher native-build ./mods --instance MyInstance --trust-native-code`. The zip is rebuilt on every change to a mod, so the link above is
always the newest. The `README.txt` inside lists what is in it, and says how to leave a mod out: delete its folder under `mods/` before you build.

Two pairs overlap, so use one of each: `zoomify` and `simple-zoom` both ask for a zoom (the first to answer wins), and `freelook` and `freecam` both take over mouse look and
the camera. Everything in the zip was checked together on one Horizon commit.

Want only some? Use the steps below.

## 1. Get the `.hnmod`

Every mod has a GitHub release named `<project>-v<version>` with the `.hnmod` and its SHA-256 next to it. Download both and
check the hash:

```sh
sha256sum -c zoomify_1.0.0.hnmod.sha256
```

The tag and file name are `zoomify-v1.0.0` and `zoomify-1.0.0.hnmod`; the hash file names the exact file it covers.

## 2. Unpack it

A `.hnmod` is a zip: `horizon-native.json` and `mixins/<id>/…` with the sources. Unpack it into its own folder:

```sh
mkdir zoomify && cd zoomify && unzip ../zoomify-1.0.0.hnmod
```

Read the files first. That is the point of a source-only bundle: `runtime.rs` is the whole logic, the other `.rs` files are one
hook each.

## 3. Build it into your game

Native mods are experimental in Horizon and the launcher asks you to trust the code. From the command line:

```sh
horizon-launcher native-build ./zoomify --instance MyInstance --trust-native-code
```

The folder can hold one unpacked bundle or several, so you can build a set of mods into one game. The launcher builds on the
Horizon commit the mods name, or on the commit of the version you play when the mods name none. All mods in one build must
agree on that commit; mods that use only the hook points (these all do) do not pin one.

## 4. Switch it on

In the game, open **Options → Mods**. Each mod has a row with its description, a settings page and, where it has keys
(the two zoom mods), the keys listed first.

## Getting it into the launcher's browser

The launcher's Mods page lists what is in [`horizon-rust/horizon-mods`](https://github.com/horizon-rust/horizon-mods), after a
maintainer has read the source. These projects are not listed there unless their maintainers submitted them; read that
repository's `MOD-POLICY.md` before doing so.
