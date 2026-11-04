#!/usr/bin/env python3
"""Writes docs/*.md (hand-written pages) and docs/mods/*.md (one per mod) into the repo."""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import common as c
from mods import MODS as BASE
from mods_raw import RAW
from mods_cam import CAM
from mods_chat import CHAT
from mods_draw import DRAW
from mods_hit import HIT
from mods_ping import PING
from mods_inv import INV
from mods_mount import MOUNT
MODS = BASE + RAW + CAM + CHAT + DRAW + HIT + PING + INV + MOUNT

ROOT = Path(__file__).resolve().parents[2]
D = ROOT / "docs"
V = "1.0.0"


def w(rel, text):
    p = D / rel
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(text.strip() + "\n")


w("introduction.md", f"""
# Introduction

**horizon-mods-lib** is a library of native mods for [Horizon](https://github.com/horizon-rust/horizon), the free,
open-source rewrite of Minecraft: Java Edition {c.GAME} in Rust. Every mod is its own small project with its own tests, its own
release and its own `.hnmod` file. They are ports: each one re-implements something people already use in Fabric
(a FPS readout, a zoom, a key overlay, waypoints) on Horizon's native hook points.

## What a native mod is

A Fabric mod is Java that gets loaded into the JVM. A Horizon native mod is Rust **source** that
[horizon-mixins](https://github.com/horizon-rust/horizon-mixins) merges into the game's own source before it is compiled. The
launcher builds a game for the instance the mod is installed in, so the mod is not a plugin loaded at run time: it is part of
the executable you play.

That has consequences, and this library is built around them:

- A mod can only do what the game's source lets it hook. Horizon keeps a short list of **stable hook points**
  (see [Hook points](hook-points.md)); mods written against them build on any Horizon whose hook-point version is new enough.
- A native mod runs with the player's rights, so the registry reads the source of every native mod before it lists it. These
  projects are small on purpose: a mod's logic is one `runtime.rs` file with unit tests, and each hook is a helper of a few lines.
- Everything is MIT, nothing is minified, and the `.hnmod` contains exactly the `.rs` and `.toml` files you can read in the repository.

## What is here

| | |
|---|---|
| [Mods](mods/fps-display.md) | {len(MODS)} projects in `mods/`, one folder each |
| [Catalogue](../MODS.md) | The 186 mods in the Tempest cross-pack matrix and where each stands on Horizon |
| [Modules](../MODULES.md) | Tempest's own module list, and an overview of the Horizon organisation |
| [Credits](credits.md), [Licensing](licensing.md), [DMCA](dmca.md) | Who made what, under which terms, and how to ask for something to come down |

## Where to go next

1. [Install a mod](install.md), or all of them in one go, if you want to play with them.
2. [Build from source](build.md) if you want to change one.
3. [Anatomy of a mod](anatomy.md) and [Hook points](hook-points.md) if you want to write the next one.
""")

w("install.md", f"""
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
""")

w("build.md", f"""
# Build from source

You need Rust, Python 3.12+ and Git. The mods' own logic builds and tests with plain `cargo`; checking them against Horizon
needs Horizon's source and a few system libraries.

## Test a mod's logic

```sh
cd mods/zoomify
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

The Cargo library of each project **is** `mixins/<id>/runtime.rs`, the same file that horizon-mixins merges into the game, so
what you test is what ships.

## Apply and type-check on Horizon

`tools/mods.py` is adapted from the registry's own tool (`horizon-rust/horizon-mods`, `native/tools/native.py`, MIT):

```sh
python3 tools/mods.py list                     # every project
python3 tools/mods.py check zoomify            # apply on the pinned Horizon, cargo check the game
python3 tools/mods.py check all                # every mod together, as a player's set is
python3 tools/mods.py package zoomify          # target/zoomify-{V}.hnmod and its SHA-256
```

`check` downloads Horizon's source at the commit in `compatibility.json` (GitHub's archive, never a checkout), builds
horizon-mixins at its pinned commit, applies the mods and runs `cargo check` on the modded game with warnings denied. On Ubuntu
the game's crates need `libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev pkg-config`.

## Pins

`compatibility.json` in each project names the Horizon commit it was tested on and the horizon-mixins commit it was applied
with, and `"hooks": 1`: the hook-point API version it needs. See [Compatibility](compatibility.md).
""")

w("anatomy.md", f"""
# Anatomy of a mod

```text
mods/zoomify/
  Cargo.toml                       the Cargo library: points at runtime.rs
  Cargo.lock
  compatibility.json               pinned Horizon and horizon-mixins commits, hooks API
  LICENSE
  README.md
  mixins/zoomify/
    horizon-mixin.toml             module + HEAD injections
    runtime.rs                     logic, state and tests (one file)
    key.rs wheel.rs zoom.rs        one helper per hook
    list.rs settings.rs click.rs   the Options > Mods row and page
```

## `horizon-mixin.toml`

Declares one **module** (the file `runtime.rs`, added to the game's `game.rs` as `game::lib_<id>`) and one **injection** per hook:

```toml
[[module]]
id = "runtime"
file = "crates/horizon-client/src/game.rs"
name = "lib_zoomify"
source = "runtime.rs"

[[injection]]
id = "wheel"
kind = "HEAD"
target = "horizon_client::game::native_hooks::Game<'_>::native_wheel"
helper = "wheel.rs"
bindings = [
  {{ name = "game", ty = "&crate::game::Game<'_>", value = "&*self" }},
  {{ name = "steps", ty = "f32", value = "steps" }},
  {{ name = "claimed", ty = "&mut bool", value = "&mut *claimed" }},
]
```

A HEAD injection runs the helper before the hook point's own body, which is empty. Helper names are prefixed with the mod's
id (`zoomify_wheel`) because every mod in a build ends up in the same module.

## `runtime.rs`

Pure logic: no game types. It holds the mod's state in a `thread_local!`, reads settings through a closure the helper passes in
(`|k| game.native_setting(ID, k)`), and carries its unit tests. That is what makes a native mod testable without the game.

## The helpers

Each helper is a few lines: check `game.native_on(ID)`, read what it needs from the game, call into `runtime.rs`, draw or
store the result. A mod that is switched off in Options → Mods does nothing until the next build leaves it out, so every helper
checks first.

## `compatibility.json` and the manifest

`tools/mods.py package` writes `horizon-native.json` into the `.hnmod` from `compatibility.json`:

```json
{{"schema": 1, "id": "zoomify", "version": "{V}", "game": "{c.GAME}",
 "horizon": "{c.HORIZON_REV}",
 "mixins": "{c.MIXINS_REV}", "hooks": 1}}
```

The launcher validates it (`schema 1`, exact 40-character revisions, `hooks` not zero) before it builds anything and refuses
bundles with links, absolute paths or more than 512 entries.
""")

w("hook-points.md", f"""
# Hook points

The hook points are empty functions in Horizon's `crates/horizon-client/src/game/native_hooks.rs`. The game calls them at fixed
places; a native mod hooks them with HEAD injections. They keep their names and signatures from release to release and are only ever
added; each addition raises `NATIVE_HOOKS_API` (3 at the commit these projects were tested on).

| Hook | Called | These projects use it for |
|---|---|---|
| `native_frame(elapsed)` | Once a frame | Nothing yet |
| `native_tick()` | Once a tick, in every `Game` (check `native_window()`) | `info-hud`: speed from how far you moved |
| `native_world_lifecycle(…)` | World open, open failed, final close (API 3) | Nothing yet |
| `native_hud(batch, corners)` | The HUD, not under F1 | Every HUD mod: `corners.place` a box, draw with `native_fill` and `native_text` |
| `native_key(key, pressed)` | A key or mouse button, by vanilla name | `zoomify`, `simple-zoom` |
| `native_wheel(steps, claimed)` | Wheel steps while playing | The zoom mods claim them while zoomed |
| `native_command(command, ran)` | A chat command, before the server sees it | `/fps`, `/wp`, `/hud`… |
| `native_zoom(zoom)` | Each frame while playing | The zoom mods set factor, smoothing and turn scale |
| `native_minimap(minimap)` | Each frame | Nothing here; Horizon ships a minimap |
| `native_mods(mods)` | Options → Mods lists native mods | Every mod pushes its row |
| `native_settings(mod_id, buttons)` | A mod's settings page opens | Every mod pushes its buttons |
| `native_settings_click(mod_id, button)` | A button was clicked | Every mod flips its option |
| `native_panel_event(…)` | The item panel's clicks (API 2) | Nothing yet |

## Helpers the game gives a mod

`native_on`, `native_playing`, `native_window`, `native_corner_shown`, `native_text_width`, `native_fill`, `native_text`,
`native_player` (position, yaw, pitch, dimension), `native_frame_rate`, `native_clicks`, `native_mapping`, `native_mod_key`,
`native_chat`, `native_data_dir`, `native_setting`, `native_set_setting`, and the item and recipe helpers of API 2.

## Reading the game's own state

Some mods here (`armor-hud`, `potion-counter`, `totem-counter`, `effect-timer`, `apple-skin`, `pick-up-notifier`) hang on the stable
`native_hud` and `native_tick` hook points, but they read what those hooks do not hand over: the inventory, armor slots, active effects and food,
straight from Horizon's `Game` struct. horizon-mixins puts a mod's helpers inside Horizon's `game` module, where those private fields are visible. That works, and
`tools/mods.py check` proves it compiles against the pinned commit, but it is **not covered by the hook-point contract**: if Horizon renames a field, these mods
stop compiling and must be updated. For that reason they carry no `hooks` entry in `compatibility.json`, only the Horizon commit they were tested on.

## Hooking Horizon's own functions

Four mods go one step further and use horizon-mixins' **cancellable HEAD** hooks on functions that are not hook points at all: `freelook` and `freecam` on
`perspective::turn_player` (the mouse turns the view, not the player) and `perspective::camera_view` (the camera the frame renders from, run again with the mod's own angles);
`better-hurt-cam` on `fov::bob_matrix` (the hurt and bob pose, rebuilt with a weaker tilt); and `snappy-tappy` and `freecam` edit the game's held-key set before movement is read.
All four are pinned to the commit they were checked on. One thing mixins cannot do: rewrite a function that uses `?` or closures except with a HEAD hook, which is why
InvMove (it needs a change inside Horizon's tick function) is not ported.

## Chat and tooltips

Chat and tooltips are Horizon's own code too. `chat-notify`, `more-chat-history` and `chat-animation` put their module in Horizon's chat file (`crates/horizon-client-render/src/chat.rs`) and hook
`Chat::add_message` (before the original, and, for `more-chat-history`, instead of it with a longer limit), `Chat::add_lines` (a constant modified) and `Chat::layout` (rebuilt with a slide). `durability-tooltip`
adds a RETURN modifier to `stack_tooltip`. A mod that wants the chat module's state from the game crate reaches it through `horizon_client::chat`. A hook that replaces a function copies the original's
steps, so it needs the same re-check as the others when Horizon changes that function.

## Crosshair and tooltip drawing

`crosshair-indicator` draws a red mark over Horizon's crosshair from the HUD hook point, using `Game::crosshair_living` and the HUD batch's width and height. `entity-crosshair` puts its module in `hud_layout.rs` and adds a RETURN
modifier to `hud_layout::crosshair`: it drops the crosshair sprite from the draw list (keeping the attack indicator) and appends its own shape as plain fills, with the game telling it each frame what is under the crosshair.
`shulker-box-tooltip` adds a RETURN modifier to `stack_tooltip` and reads the box's contents from `ItemStack::data`. Horizon has no draw hook inside tooltips, so the contents are text, not the original's picture of the box.

## What the hook points cannot do yet

No inventory, effects, incoming chat, camera, input injection or world data beyond the player's position. That is the reason
most of the [catalogue](../MODS.md) is marked *needs a hook point*: armour and potion HUDs need the inventory and effects, chat
mods need incoming chat, Freecam and Freelook need the camera. Raising a hook point is a change to Horizon, not to a mod; the
contract is described in `docs/native-mods.md` and `native-hooks.json` in the Horizon repository.
""")

w("settings.md", """
# Settings and storage

A native mod keeps its settings in files, one file per key, in `horizon-mods/.horizon-mod-data/<id>/` next to the game's options.
`game.native_setting(id, key)` reads one (trimmed), `game.native_set_setting(id, key, value)` writes one.

These projects store booleans as `true` and `false`; anything else reads as the option's default. Zoom mods also store `level`
(one of 2, 3, 4, 6, 8, 12, 16). Waypoints stores every waypoint in one file called `waypoints`, one tab-separated line each
(`name x y z dimension`); a line that does not read is skipped, not fatal.

Keys are different: they belong to the game's controls. A mod lists keys in its Options → Mods row (`keys` in `NativeMod`:
name, label, default) and reads the player's choice with `native_mod_key`. Both zoom mods list one key.

State that is not a setting (the zoom while it is on, the last position for speed, the tracked waypoint) lives in a `thread_local!`
in `runtime.rs` and is gone when the game closes.
""")

w("releases.md", f"""
# Releases

Every project releases itself.

1. Change the mod, and bump `version` in `mods/<slug>/mixins/<id>/horizon-mixin.toml` (and `Cargo.toml`).
2. Push to `main`.
3. `.github/workflows/release-<slug>.yml` calls the shared `_release.yml`, which runs `cargo fmt --check`, clippy with warnings
   denied and `cargo test`, packages the `.hnmod`, uploads it as a workflow artifact and, **if the tag `<slug>-v<version>` does
   not exist yet**, creates that GitHub release with the `.hnmod` and a `.sha256` file.

A push that does not change a mod's folder does not trigger its workflow, and a push that does not change the version
releases nothing. To test and package without releasing, run a workflow by hand from the Actions tab and leave *publish* off.

`ci.yml` runs on every push and pull request: each mod's tests in a matrix, and a job that applies **all** mods on the pinned
Horizon and type-checks the game, so a hook that no longer matches Horizon fails there and not on a player's machine.

## Versioning

Semantic versions, per mod. A change that needs a newer hook-point API also bumps `hooks` in `compatibility.json`. A new Horizon
commit is tested by changing `horizon_revision` (in `compatibility.json` and `horizon-mixin.toml`) and letting CI apply the set.
""")

w("compatibility.md", f"""
# Compatibility

| | |
|---|---|
| Game | Minecraft Java {c.GAME}, as implemented by Horizon |
| Horizon (tested on) | `{c.HORIZON_REV}` (Nightly 2026-10-30) |
| horizon-mixins | `{c.MIXINS_REV}` (0.6.0, plan schema 6) |
| Hook points | `hooks: 1`: builds on any Horizon whose `NATIVE_HOOKS_API` is 1 or later |
| Rust | Whatever the launcher brings (it downloads its own toolchain); CI uses stable |

The nine mods that use only the stable hook points are described below; the six that read game state (see [Hook points](hook-points.md)) are pinned to the exact commit and have no `hooks` entry.

Because the first nine hook only the stable hook points, the Horizon commit is the one they were *tested on*, not a requirement:
the launcher builds them on the commit the other mods in the build pin, else on the commit of the installed version. A hook
point that is removed or changed incompatibly would break them, and Horizon's contract is that it does not do that.

Native mods are not built for dedicated servers yet; on a server you join, only client-side hooks do anything and the protocol is
unchanged. In single player and on a LAN host the mods run in the host's game.
""")

w("credits.md", f"""
# Credits

## Horizon

- **Horizon** is made by Ludovic Marie and the Horizon contributors, MIT:
  [horizon-rust/horizon](https://github.com/horizon-rust/horizon).
- **horizon-mixins**, the tool that turns these bundles into a game, is by levzzz5154, MIT:
  [horizon-rust/horizon-mixins](https://github.com/horizon-rust/horizon-mixins).
- The structure of every project follows
  [horizon-native-mod-template](https://github.com/horizon-rust/horizon-native-mod-template), itself based on levzzz5154's
  Veinminer port, MIT.
- `tools/mods.py` is adapted from `native/tools/native.py` in [horizon-rust/horizon-mods](https://github.com/horizon-rust/horizon-mods), MIT.
- The site's look follows the Horizon website, [horizon-rust/horizon-site](https://github.com/horizon-rust/horizon-site), MIT: the
  same dark violet palette, card shapes and spacing, written again as plain CSS. No file from that site is copied.

## The mods these are ported from

Each project re-implements the behaviour of an existing mod. No code, art or text is copied from them, and they are not endorsed
by their authors. The authors are credited here because the idea, the name and the design are theirs.

| Project | Ported from | Upstream licence |
|---|---|---|
{chr(10).join(f"| [`{m['slug']}`](mods/{m['slug']}.md) | {m['upstream']} | {m['upstream_licence']} |" for m in MODS)}

The complete list of upstream projects with their authors' licences, as shown on Modrinth, is in the
[catalogue](../MODS.md). A Modrinth page is the place to find who made a mod and to support them.

## Tempest

The ports and the module list come out of the Tempest client project, [pxdns/patcher](https://github.com/pxdns/patcher): its
cross-pack matrix, its licence notes (`docs/MOD-LICENSES.md`) and its 1.16.5 port status.

## Icons

- **The nine projects' own icons** (`assets/icons/<project>.svg`) are original artwork made for this library, MIT.
- **The catalogue's icons** (`assets/icons/catalogue/`) are each upstream mod's **own project icon, downloaded from Modrinth**, shown only to identify the mod
  it belongs to. They are the work of those mods' authors (named on every catalogue page, from Modrinth's team listing) and are **not** covered by this repository's
  MIT licence: they stay under whatever terms the authors gave them. They are shrunk to 96 px and otherwise unaltered. Three projects have no icon on Modrinth and
  show a placeholder tile instead.
- If you made one of these icons and want it gone, see [DMCA and takedowns](dmca.md); it is removed without argument.

## The offline Horizon build

`horizon-offline.html` (on the site's Play page) is a single-file offline build of Horizon, added by this repository's owner. Horizon is MIT, by Ludovic Marie and the Horizon contributors. The file's own comments name three projects
it includes: [xzwasm](https://github.com/SteveSanderson/xzwasm) (an xz decoder), [walloc](https://github.com/wingo/walloc) (a WebAssembly allocator) and [XZ Embedded](https://tukaani.org/xz/embedded.html); see their repositories for
their terms. The file contains none of Minecraft's resources, and it is not an official release of the Horizon project as far as this repository can tell: it is unverified against Horizon's own releases.

## Fonts

None are bundled. The site uses the visitor's own system fonts, so it loads no third-party font, script, analytics or tracker.
""")

w("licensing.md", f"""
# Licensing

## Code

Everything in this repository (the mods, the tools, the site, the documentation) is MIT, see [`LICENSE`](https://github.com/pxdns/horizon-mods-lib/blob/main/LICENSE):
Copyright (c) 2026 pxdns and the horizon-mods-lib contributors. Each project folder carries its own copy, and the `.hnmod`
includes it as `mixins/<id>/LICENSE`, so the notice travels with the sources.

## Upstream mods and clean-room ports

The ports are written from behaviour, not from upstream source. Nobody here has copied or translated another mod's code, art,
text, or data files; the reference was what the mod does when you use it, and its public description. Several upstream mods are
LGPL, GPL or all-rights-reserved; none of that applies to a program that contains none of their code. A port can still share a
name or idea with its original, and the [credits](credits.md) say whose idea it was.

If you contribute a port: **do not paste upstream code into a pull request**, and do not translate it line by line. Describe the
behaviour, write the Rust, test it.

## Horizon and Minecraft

Horizon is MIT and independent of Mojang and Microsoft. These mods run in Horizon, which reads worlds and resources from a
Minecraft installation the player owns; **no Minecraft assets, code or names beyond plain descriptive references are included
here.** Minecraft is a trademark of Mojang Synergies AB. See [Trademarks](trademarks.md).

## Third-party material

| What | Licence |
|---|---|
| `tools/mods.py` (adapted from horizon-mods) | MIT, Ludovic Marie and the Horizon contributors |
| Project layout and `horizon-mixin.toml` shape (from the template) | MIT, levzzz5154 and the Horizon contributors |
| Python `markdown` (site build only, not shipped) | BSD-3-Clause |
| Upstream mod icons in `assets/icons/catalogue/` | Their authors' terms, not MIT; shown for identification, removed on request |

Bundled fonts: none. The Rust crates in the mod projects: none; every `Cargo.toml` has no dependencies.
""")

w("dmca.md", """
# DMCA and takedowns

This library re-implements the behaviour of other people's mods. If you think a project here copies your work, or that
something else in this repository infringes a right you hold, you can have it looked at and, if you are right, removed.

## How to send a notice

Open an issue at [`pxdns/horizon-mods-lib`](https://github.com/pxdns/horizon-mods-lib/issues) titled **DMCA notice**, or, if you
prefer not to post publicly, message the repository owner [`pxdns`](https://github.com/pxdns) through their GitHub profile and
ask for a private channel. GitHub also takes notices directly under its own
[DMCA policy](https://docs.github.com/en/site-policy/content-removal-policies/dmca-takedown-policy), which applies to this repository
whatever is done here.

A notice should have the elements the law asks for (17 U.S.C. § 512(c)(3)):

1. Your name and a way to contact you.
2. The work you say is infringed, with a link to it.
3. The material here you say infringes, with the file path or URL (a line range helps).
4. A statement that you believe in good faith that the use is not authorised by the owner, its agent or the law.
5. A statement that the information in the notice is accurate and, under penalty of perjury, that you are the owner or authorised to act for the owner.
6. Your physical or electronic signature.

## What happens next

The material is reviewed within a few days. If it is infringing, or if there is real doubt, it is removed from `main` and from the
site, the release asset is deleted, and the project's page says so. Where the complaint is that a port is too close to the original,
the fix may be a rewrite rather than a removal; you will be told which.

## Counter-notice

If something was taken down and you believe that was a mistake, send a counter-notice by the same route with your name,
contact details, the material removed and where it was, a statement under penalty of perjury that you believe in good faith it was
removed by mistake or misidentification, and your consent to the jurisdiction of the federal court for your address (or, outside
the US, any judicial district in which GitHub may be found), and your signature. Material may be restored after the period the
law allows, unless the sender tells us they have gone to court.

## Repeat infringers

Contributors who repeatedly submit infringing material lose the right to contribute.

This page describes how the project handles notices. It is not legal advice.
""")

w("trademarks.md", """
# Trademarks and disclaimer

**This is an unofficial community project. It is not made, endorsed, approved or supported by Mojang, Microsoft, or the Horizon
project's maintainers beyond what their own licences say.** Native mods run with the player's rights, and the Horizon launcher
shows its own warning before it builds one; that warning applies here.

- *Minecraft* is a trademark of Mojang Synergies AB. Horizon is not an official Minecraft product and is not approved by or
  associated with Mojang or Microsoft; neither is this library.
- *Horizon* and the Horizon logo belong to the Horizon project. The site here copies the layout of playhorizon.dev; it uses no
  Horizon logo files, and no Minecraft logo or wordmark appears anywhere.
- Names and icons of other mods (Zoomify, Key Overlay, Xaero's Minimap…) belong to their authors and are used only to say what a port is a port
  of. Icons are the mods' own, credited on each page, and removed on request.
- GitHub, Modrinth and Discord are mentioned as plain text and links, without their logos.

The mods come with no warranty, as the [MIT licence](licensing.md) says: *"the software is provided as is"*.
""")

w("security.md", """
# Security

A native mod is native code with the player's rights. That is true of every native Horizon mod, and the launcher says so in red
before it builds one. What this library does about it:

- **Source only.** A `.hnmod` contains `.rs`, `.toml`, a `LICENSE` and `horizon-native.json`, nothing compiled. You can read all of it, and the
  launcher checks the bundle without running anything.
- **Small by design.** Each mod's logic is one file with unit tests; the helpers are a few lines each. There is no `unsafe`, no
  network access, no process spawning and no dependency in any `Cargo.toml`. The only files a mod touches are its own settings
  files through `native_set_setting`.
- **Reproducible.** The `.hnmod` is packaged deterministically (fixed timestamps, sorted entries), so the same commit gives the same
  SHA-256, which the release publishes beside the file.
- **Pinned.** `compatibility.json` names exact commits of Horizon and horizon-mixins, never a branch or a tag.

If you find a vulnerability, open a private security advisory on the repository (Security tab → Report a vulnerability) rather than a public issue.
""")

w("contributing.md", """
# Contributing

Ports are welcome. A good one is small, tested, and honest about what the hook points let it do.

1. **Pick a mod that fits the hook points.** The [catalogue](../MODS.md) marks what needs a hook point and what does not; if a mod
   needs one, the useful contribution is a pull request to [Horizon](https://github.com/horizon-rust/horizon) that adds the hook point.
2. **Copy a project folder** under `mods/` and rename it (`slug` for the folder, `snake_case` for the id). Keep `runtime.rs` pure,
   put the tests in it, keep each helper a few lines.
3. **Write it from behaviour.** Do not copy or translate upstream code. See [Licensing](licensing.md).
4. **Run** `cargo fmt`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`, and
   `python3 tools/mods.py check <slug>`.
5. **Add** a `release-<slug>.yml` (copy an existing one and change the slug), a row in `docs/credits.md`, and an icon in `assets/icons/`.
6. Open the pull request. CI applies all mods on the pinned Horizon.

Everything under `mods/`, `docs/`, `catalogue/` and the release workflows is generated from `tools/gen/`: edit the generator and run `python3 tools/gen/regen.py` (it downloads the pinned Horizon source itself, and a second run changes nothing).
See `tools/gen/README.md`.

Be accurate in README claims: say what the mod does and what it does not, and name the upstream mod and its licence.
""")

w("changelog.md", f"""
# Changelog

## 2026-11-03 — first release

Thirty native mods at {V}: `fps-display`, `sprint-indicator`, `key-overlay`, `cps-counter`, `info-hud`, `custom-hud`, `zoomify`,
`simple-zoom`, `waypoints` and, reading game state, `armor-hud`, `potion-counter`, `totem-counter`, `effect-timer`, `apple-skin`, `pick-up-notifier` and, hooking Horizon's camera and input, `freelook`, `freecam`, `snappy-tappy`, `better-hurt-cam` and, in chat and tooltips, `more-chat-history`, `chat-notify`, `chat-animation` and `durability-tooltip`, and in crosshair and tooltip drawing `crosshair-indicator`, `entity-crosshair` and `shulker-box-tooltip`, `combat-hitboxes` (a wrapped ray-versus-box test), `ping-view` (the connection ping, from a hook on its logger), `invmove` (walk with the inventory open) and `better-mount-hud` (health, speed and jump of your mount), each tested (unit tests), formatted, clippy-clean and applied on Horizon `{c.HORIZON_REV[:7]}`
with horizon-mixins `{c.MIXINS_REV[:7]}`. The catalogue covers the 186 mods in the Tempest cross-pack matrix.
""")

# --- one page per mod --------------------------------------------------------------------------------------------
for m in MODS:
    opts = m.get("options")
    option_rows = ""
    if opts:
        option_rows = "| Option | Default |\n|---|---|\n" + "\n".join(f"| {label} | {'on' if d else 'off'} |" for _, label, d in opts)
    elif m.get("zoom"):
        option_rows = ("| Option | Default |\n|---|---|\n| Starting zoom (2, 3, 4, 6, 8, 12 or 16×) | "
                       + ("4×" if m["id"] == "zoomify" else "3×") + " |\n| Smooth zoom | on |\n| Slower turning | on |\n| Wheel zoom | on |")
    cmd = ""
    if m["id"] == "waypoints":
        cmd = "| Command | What it does |\n|---|---|\n| `/wp add <name>` | Names the block you stand on |\n| `/wp list` | Lists them, with distances in your dimension |\n| `/wp del <name>` | Deletes one |\n| `/wp track <name>` | Shows its distance and direction on screen |\n| `/wp untrack` | Stops tracking |"
    elif m.get("command"):
        cmd = f"| Command | What it does |\n|---|---|\n| `/{m['command']}` | Hides or shows {m['name']} |"
    elif m.get("zoom"):
        hold = "HOLD: bool = true" in m["runtime_extra"]
        default = "C" if m["id"] == "zoomify" else "V"
        cmd = f"| Key | Default | What it does |\n|---|---|---|\n| Zoom | `{default}` | {'Hold to zoom' if hold else 'Press to turn the zoom on, press again to turn it off'}; the wheel changes it while it is on |"
    names = {"list": "native_mods", "click": "native_settings_click", "turn": "perspective::turn_player", "camera": "perspective::camera_view", "bob": "fov::bob_matrix", "add": "chat::Chat::add_message", "lines": "chat::Chat::add_lines", "message": "chat::Chat::add_message", "layout": "chat::Chat::layout", "tooltip": "stations::stack_tooltip", "crosshair": "hud_layout::crosshair", "ray": "game::ray_box", "log": "debug_chart::SampleLogger::log", "walk": "game::Game::tick"}
    hooks = ", ".join(f"`{names.get(h, 'native_' + h)}`" for h in m["hooks"])
    if m.get("usage"):
        cmd = m["usage"].replace("\\n", "\n")
    w(f"mods/{m['slug']}.md", f"""
# {m['name']}

![{m['name']} icon](../../assets/icons/{m['slug']}.svg)

{m['description']}

**Version** {V} · **Game** {c.GAME} · **{'Pinned to Horizon ' + c.HORIZON_REV[:7] + ', reads game state' if m.get('raw') else 'Hook-point API 1'}** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/{m['slug']}) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q={m['slug']})

## Use it

{cmd}

{option_rows}

Switch it on or off in **Options → Mods → {m['name']}**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: {hooks}.
- Logic and tests: `mods/{m['slug']}/mixins/{m['id']}/runtime.rs`.
- Pinned: Horizon `{c.HORIZON_REV[:7]}`, horizon-mixins `{c.MIXINS_REV[:7]}`{'. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.' if m.get('raw') else ', `hooks: 1`.'}

## Credit

Ported from **{m['upstream']}** ({m['upstream_licence']}); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
""")
print("docs written")
