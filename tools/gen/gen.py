#!/usr/bin/env python3
"""Writes the mod projects (mods/<slug>/) and their workflows into the repository."""
import json
import shutil
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import common as c
from mods import MODS as BASE, COMMAND_TOGGLE, opts
from mods_raw import RAW
from mods_cam import CAM
from mods_chat import CHAT
from mods_draw import DRAW
from mods_hit import HIT
from mods_ping import PING
from mods_inv import INV
MODS = BASE + RAW + CAM + CHAT + DRAW + HIT + PING + INV

ROOT = Path(__file__).resolve().parents[2]
VERSION = "1.0.0"
HOOKNOTE = ("It hooks only Horizon's native hook points, so the launcher builds it on any Horizon with `NATIVE_HOOKS_API` 1 or later, together with other native mods.")
RAWNOTE = ("It uses the stable HUD hook point but **reads Horizon's own game state** (inventory, armor, effects, food), which the hook-point contract does not promise. "
           "It is therefore **pinned to one Horizon commit** (`ba6344a`) and has to be re-checked whenever Horizon changes those fields; `tools/mods.py check` does that.")


def fill(text, mod):
    text = text.replace("crate::game::@MOD@", mod.get("modpath", "crate::game::lib_" + mod["id"]))
    return (text.replace("@ID@", mod["id"]).replace("@MOD@", "lib_" + mod["id"])
            .replace("@CMD@", mod.get("command") or "").replace("@NAME@", mod["name"])
            .replace("@SLUG@", mod["slug"]).replace("@GAME@", c.GAME))


def helper(mod, h):
    if h in mod.get("helpers", {}):
        return fill(mod["helpers"][h], mod)
    if h == "hud":
        return fill(mod["hud"](), mod)
    if h == "command":
        return fill(mod.get("command_body") or COMMAND_TOGGLE, mod)
    if h == "list":
        return fill(c.LIST_RS.replace("@KEYS@", mod.get("keys_expr", "Vec::new()")), mod)
    if h == "settings":
        return fill(c.SETTINGS_RS, mod)
    if h == "click":
        return fill(c.CLICK_RS, mod)
    if h in ("key", "wheel", "zoom"):
        from mods import ZOOM_HELPERS
        return fill(ZOOM_HELPERS[h], mod)
    if h == "tick":
        return mod["extra_helpers"]["tick"]
    raise SystemExit(h)


def runtime(mod):
    head = f'''// {mod["name"]}, a native Horizon mod: {mod["description"]}
// This file is both the mod's Cargo library (its tests) and the module the bundle adds to the game
// (`game::lib_{mod["id"]}`). Gameplay logic lives here, testable; the hook helpers only read the game and draw.

/// The mod's id: its settings folder and its keys' lines.
pub const ID: &str = "{mod["id"]}";
pub const NAME: &str = "{mod["name"]}";
pub const VERSION: &str = "{VERSION}";
pub const DESCRIPTION: &str = "{mod["description"]}";
'''
    body = head
    if mod.get("options"):
        body += c.OPTIONS_RS.replace("@OPTIONS@", opts(mod["options"]))
    body += mod["runtime_extra"]
    tests = mod["tests"] + (c.OPTIONS_TESTS_RS if mod.get("options") else "")
    body += "\n#[cfg(test)]\nmod tests {\n    use super::*;\n" + tests + "}\n"
    return body


def readme(mod):
    cmd = f"- `/{mod['command']}` hides or shows it.\n" if mod.get("command") and not mod.get("command_body") else ""
    if mod["id"] == "waypoints":
        cmd = ("- `/wp add <name>` names the place you stand, `/wp list` lists them, `/wp del <name>` removes one.\n"
               "- `/wp track <name>` puts its distance and direction in the bottom-right corner; `/wp untrack` stops.\n")
    if mod.get("zoom"):
        key = "Zoom key (hold)" if "HOLD: bool = true" in mod["runtime_extra"] else "Zoom key (press to toggle)"
        default = "key.keyboard.c" if mod["id"] == "zoomify" else "key.keyboard.v"
        cmd = f"- {key}: `{default}` by default, changed in Options > Mods > {mod['name']}.\n- The mouse wheel changes the zoom while it is on.\n"
    return f'''# {mod["name"]} (Horizon native mod)

{mod["description"]}

{cmd}
Ported from **{mod["upstream"]}** ({mod["upstream_licence"]}) as an independent implementation for Horizon {c.GAME};
no upstream code is used. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/{mod["id"]}/`). {mod.get("raw_note", RAWNOTE) if mod.get("raw") else HOOKNOTE}

## Install

Download `{mod["id"]}-{VERSION}.hnmod` and its `.sha256` from this project's
[releases](https://github.com/pxdns/horizon-mods-lib/releases?q={mod["slug"]}), check the hash, unzip the bundle into a folder and build it
into a game with `horizon-launcher native-build FOLDER --instance NAME --trust-native-code`. Then switch it on or off in the game
under Options → Mods → {mod["name"]}. The [install guide](https://pxdns.github.io/horizon-mods-lib/docs/install.html) has the steps,
and Horizon's [native mods](https://github.com/horizon-rust/horizon/blob/main/docs/native-mods.md) page says what a build does and trusts.

## Files

| File | What it is |
|---|---|
| `mixins/{mod["id"]}/horizon-mixin.toml` | The bundle: one module and the HEAD hooks |
| `mixins/{mod["id"]}/runtime.rs` | The mod's logic, state and tests; added to the game as `game::lib_{mod["id"]}` and this crate's library |
| `mixins/{mod["id"]}/*.rs` | One small helper per hook |
| `compatibility.json` | The tested Horizon and horizon-mixins commits, and the hook-point API it needs |

## Build, test, release

```sh
cd mods/{mod["slug"]} && cargo test          # the logic
python3 tools/mods.py check {mod["slug"]}    # apply on the pinned Horizon and cargo check the game
python3 tools/mods.py package {mod["slug"]}  # target/{mod["id"]}-{VERSION}.hnmod
```

Pushing a new `version` in `horizon-mixin.toml` to `main` releases it:
`.github/workflows/release-{mod["slug"]}.yml` tests and packages the mod and publishes the release `{mod["slug"]}-v{VERSION}`
with the `.hnmod` and its SHA-256.

License: MIT.
'''


def write(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def main():
    (ROOT / ".github/workflows").mkdir(parents=True, exist_ok=True)
    for mod in MODS:
        d = ROOT / "mods" / mod["slug"]
        if d.exists():
            shutil.rmtree(d)
        bundle = d / "mixins" / mod["id"]
        write(d / "Cargo.toml", c.cargo_toml(mod["slug"], VERSION, mod["description"]).replace("{ID}", mod["id"]))
        write(d / "compatibility.json", c.compat_json(mod.get("raw", False)))
        write(d / "LICENSE", c.LICENSE)
        write(d / "README.md", readme(mod))
        write(bundle / "horizon-mixin.toml", mod["toml_text"] if mod.get("toml_text") else c.toml(mod["id"], mod["hooks"], "lib_" + mod["id"], VERSION))
        write(bundle / "runtime.rs", runtime(mod))
        for h in mod["hooks"]:
            write(bundle / f"{h}.rs", helper(mod, h))
        write(ROOT / ".github/workflows" / f"release-{mod['slug']}.yml", fill(c.RELEASE_YML, mod))
    write(ROOT / ".github/workflows/_release.yml", c.REUSABLE_YML.replace("@GAME@", c.GAME))
    write(ROOT / ".github/workflows/ci.yml", c.CI_YML.replace("@MODS@", ", ".join(m["slug"] for m in MODS)))
    write(ROOT / "LICENSE", c.LICENSE)
    print(json.dumps([m["slug"] for m in MODS]))


main()
