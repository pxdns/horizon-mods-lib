#!/usr/bin/env python3
"""MODS.md, MODULES.md and site/data/mods.json from pxdns/patcher's docs/MOD-MATRIX.md and friends."""
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
INPUTS = HERE / "inputs"  # snapshots of the Tempest project's mod matrix, licence notes and module list
import os
_CREDITS_FILE = ROOT / "site/data/icon-credits.json"
CREDITS = json.load(open(_CREDITS_FILE)) if _CREDITS_FILE.exists() else {}

# Modrinth project id -> (status, project slug in this repo, note)
PORTED = {
    "DIlqwRFH": ("ported", "fps-display", "FPS Display"),
    "KKLrfZbI": ("ported", "sprint-indicator", "Sprint Indicator"),
    "Upv1bw1P": ("ported", "key-overlay", "Key Overlay"),
    "w7ThoJFB": ("ported", "zoomify", "Zoomify"),
    "EYKTeOIO": ("ported", "simple-zoom", "Simple Zoom"),
    "kqJFAPU9": ("ported", "better-mount-hud", "Better Mount HUD"),
    "REfW2AEX": ("ported", "invmove", "InvMove"),
    "gvl7lVcV": ("ported", "ping-view", "Ping View"),
    "1bokaNcj": ("partial", "minimap-plus", "The map display (corner, size, zoom keys, big map, turning, round) on Horizon's own minimap, plus the waypoints mod; waypoints are not marked on the map and there is no entity radar."),
    "NcUtCpym": ("partial", "minimap-plus", "A big-map key that widens Horizon's minimap to its largest view; there is no full-screen map screen to scroll, since Horizon has no hook for a new screen."),
    "JzdjByS4": ("ported", "potion-counter", "Potion Counter (raw-hook port: reads the inventory)"),
    "T9R7YTnA": ("ported", "totem-counter", "Totem Counter (raw-hook port: reads the inventory)"),
    "T9FDHbY5": ("ported", "effect-timer", "Effect Timer (raw-hook port: reads the active effects)"),
    "x02cBj9Y": ("ported", "effect-timer", "Timer text and the duration bars under the effect icons"),
    "EsAfCjCV": ("ported", "apple-skin", "Numbers, plus the held-food tint and saturation line drawn on the food bar"),
    "ZX66K16c": ("ported", "pick-up-notifier", "Pick Up Notifier (raw-hook port: watches the inventory)"),
    "wF189hn9": ("ported", "armor-hud", "Armor HUD (raw-hook port: reads the worn armor)"),
    "XeEZ3fK2": ("ported", "freecam", "Freecam (raw-hook port: detached camera)"),
    "g4pR0Fmy": ("ported", "freelook", "Freelook (raw-hook port: mouse look and render camera)"),
    "dWOPoahl": ("ported", "snappy-tappy", "Snappy Tappy (raw-hook port: held-key set)"),
    "o4y0N2hu": ("ported", "better-hurt-cam", "Better Hurt Cam (raw-hook port: camera bob and hurt tilt)"),
    "DnNYdJsx": ("ported", "chat-animation", "Chat Animation (raw-hook port: rebuilds Chat::layout)"),
    "Iudurxl8": ("ported", "chat-notify", "Chat Notify (raw-hook port: hooks Chat::add_message)"),
    "8qkXwOnk": ("ported", "more-chat-history", "More Chat History (raw-hook port: raises the chat limits)"),
    "smUP7V3r": ("ported", "durability-tooltip", "Durability Tooltip (raw-hook port: returns from stack_tooltip)"),
    "zLAdIOH7": ("ported", "crosshair-indicator", "Crosshair Indicator (raw-hook port: draws over the crosshair)"),
    "HBz1jJcy": ("ported", "entity-crosshair", "Entity Crosshair (raw-hook port: replaces the crosshair shape)"),
    "2M01OLQq": ("ported", "shulker-box-tooltip", "Totals by item and a graphical 9 by 3 grid with real item icons"),
    "mOgUt4GM": ("builtin", "", "Horizon lists native mods in its own Options > Mods screen, with a settings page for each."),
    "geGZinIB": ("ported", "combat-hitboxes", "Combat Hitboxes (raw-hook port: wraps the ray-versus-box test)"),
    "qQyHxfxd": ("builtin", "", "Horizon already sends chat unsigned (its `send_chat` signs nothing), which is what this mod does for Fabric."),
    "yjgIrBjZ": ("builtin", "", "Horizon has its own account sign-in (a device-code sign-in in its settings)."),
    "aC3cM3Vq": ("builtin", "", "Horizon ships Mouse Tweaks as one of its own extras (Options > Extras)."),
    "K0AkAin6": ("builtin", "", "Horizon ships Client Sort as one of its own extras (sort key in Extras)."),
    "wdLuzzEP": ("builtin", "", "Horizon has a Fullbright extra of its own."),
    "2VF6JEnL": ("partial", "multi-key-bindings", "Extra keys for an action, made with /multikey commands and honoured wherever the action is; no buttons on the Key Binds page, no modifier combinations"),
    "xv94TkTM": ("partial", "controlling", "Find bindings by action or key, list conflicts and unbound actions as /keybinds commands; Horizon's own Key Binds page already searches by action name"),
    "Wb5oqrBJ": ("ported", "chat-heads", "The sender's face in front of each chat line, through Horizon's inline player heads"),
    "ozpC8eDC": ("partial", "crystal-optimizer", "A crystal you hit vanishes at once on a server and returns if the server does not break it; no crystal placement prediction"),
    "HEds12HI": ("partial", "visual-keys", "The keyboard on the HUD with every key coloured by its binding, plus /keyboard free and /keyboard <key>; not inside the controls screen, no Malilib bindings, no category collapse"),
    "DMu0oBKf": ("partial", "crafting-tweaks", "Rotate, balance and clear as keys, for single player and LAN hosts; no buttons, full-stack result click, hold-W transfer or compress keys"),
    "cVNW5lr6": ("ported", "item-highlighter", "A twinkling star on slots holding items picked up since the inventory was last closed"),
    "n6PXGAoM": ("partial", "better-stats", "Searching, ranking and exporting as /statistics commands, with Search Stats for the search box; not the redesigned grid screen, rendered mobs, grouping or sharing"),
    "ExI7GmJi": ("partial", "search-stats", "A search box that filters the Items and Mobs lists; the General tab is not filtered"),
    "Wnxd13zP": ("ported", "clumps", "Orbs of any size merge into one orb worth all of them, collected at once"),
    "GfoMZ9Ly": ("partial", "shield-statuses", "Your own shield as ready, blocking or disabled with the seconds left; other players' shields need an entity drawing hook"),
    "MOqt4Z5n": ("partial", "chat-patches", "Timestamps and an option to drop immediate repeats; not copying, hiding or compact counters"),
    "a9H9ZwuV": ("ported", "f5-split", "Separate back and front view keys, toggle or hold; no per-view field of view"),
    "xLSpHE43": ("partial", "enchantment-descriptions", "The descriptions under the tooltip; no coloured enchantment names"),
    "2RKFTmiB": ("builtin", "", "Horizon already keeps a tooltip on screen: its tooltip placement moves it back from the screen's edges."),
    "UMxybHE8": ("partial", "info-hud", "Speed, facing, dimension and angles in Info HUD, coordinates in Custom HUD; no light level, biome or chunk overlays"),
    "QrKEZXN9": ("partial", "crosshair-indicator", "A mark over the crosshair when it is over a living entity; none of the other conditions"),
    "VyxkIxCd": ("builtin", "", "Horizon has a Fullbright extra of its own."),
    "UVtY3ZAC": ("ported", "enchantment-descriptions", "A gray line per enchantment under the tooltip saying what it does"),
    "r6VxNyu0": ("ported", "better-hurt-cam", "The death tilt can be switched off in Better Hurt Cam"),
    "Si9Uim4y": ("ported", "armor-hud", "Pieces and durability, the armor value on the bar, and extra rows past 20 points"),
}

LIBRARY = ["cloth-config", "balm", "bookshelf", "puzzleslib", "collective", "konkrete", "malilib", "midnightlib", "fabric-api",
           "fabric-language-kotlin", "forgeconfigapi", "fzzy_config", "yet_another_config", "zconfig", "supermartijn642configlib",
           "tcdcommons", "ukulib", "libjf", "walksylib", "text placeholder", "oneconfig", "compose-bundle", "pricklemc",
           "adventure-platform", "better-compatability", "clienttweaks", "prickle", "essential_", "resourcify"]
ENGINE = ["scalablelux", "sodium", "lithium", "c2me", "concurrent chunk", "krypton", "ferritecore", "modernfix", "vmp-", "nvidium", "immediatelyfast",
          "scalablux", "spark-", "servercore", "alternate-current", "badoptimizations", "zfastnoise", "entity culling", "entityculling",
          "more culling", "moreculling", "debugify", "packet fixer", "memoryleakfix", "fastquit", "crashassistant", "iris-fabric",
          "silicon", "continuity", "fast ip ping", "dynamic fps", "e4mc", "voicechat", "inventorytotem"]
PACK = ["complementary", "miniature-shader", "default-dark-mode", "unique dark", "skyboxify", "glow_trims", "visual_armor_trims",
        "emissive trims", "red trapped", "short swords", "low fire", "low shield", "sodiumtranslations", "real-arrow-tip",
        "eating animation", "eclipseui", "cobweb outline", "waveycapes", "capes", "skinlayers3d", "betterhurtcam", "shieldfixes",
        "notenoughanimations", "visuality", "dark-loading-screen", "remove reloading screen", "block outline", "anchor optimizer",
        "centered-crosshair", "nofog", "clearwaterlava", "sodium-shadowy"]
REASONS = [
    (("chat", "nochat", "visualkeys"), "Needs incoming-chat or screen hooks that Horizon does not expose yet."),
    (("armor", "potion", "totem", "status-effect", "statuseffect", "appleskin", "pickup", "detailarmor", "durability", "shulker", "tooltip"),
     "Needs inventory, effect or item data that the hook points do not give a mod yet."),
    (("freecam", "freelook", "gamma", "fullbright", "crosshair", "hitbox", "invmove", "snappy", "mouse", "controlling", "multi-key", "clientsort",
      "crafting"), "Needs input or camera hooks that Horizon does not expose yet."),
    (("minimap", "map", "litematica", "tiers"), "Needs a screen or world-data hook point."),
]

DEFAULT_REASON = "Needs a hook point Horizon does not have yet (the current hook points are listed in the docs)."


SPECIFIC = {
    "BVzZfTc1": "Chooses entity textures by rules from resource packs; Horizon's mob renderers pick their texture inside the renderer with no hook for the choice. Not attempted.",
    "m5T5xmUy": "Swaps a grass block's side texture for its top texture next to other grass. Horizon's mesher (`mesher.rs`, `Mesher::face`) draws a face from a texture layer that a rebuilt `face` could swap, but the result cannot be seen or checked here. Not attempted.",
    "bEpr0Arc": "Schematic loading, rendering and placement: its own file formats, a world overlay renderer and placement tools. Not attempted; it is far larger than the other ports.",
    "aSymSNSe": "Horizon's single-player pearl logic runs for any pearl entity it simulates (it teleports the player and deals the fall damage when the pearl lands, `creatures.rs`), so a local pearl made to look instant on a server would also teleport and hurt the player on this client. A purely visual projectile does not exist to hook. Not attempted. The original is All Rights Reserved, so it was not read.",
    "bubOj4AD": "Looks players up on a third-party tier website and draws the result on name tags. Native mods here make no network requests of their own, and there is no hook in the name tag drawing. Not attempted.",
}

# No 26.3 build of the upstream project exists, which does not stop a native port; these are the reasons the entries are still not ported.
NO263_REASONS = {
    "r0v8vy1s": "A redstone engine: world logic, which Horizon implements itself and no hook point reaches.",
    "zqU1uktf": "Draws entity hitboxes in the world; there is no entity rendering hook.",
    "8shC1gFX": "Replaces the F3 debug screen, which Horizon draws in its own code; there is no hook there.",
    "M08ruV16": "Keeps chunks beyond the server's view distance: chunk storage and the network layer, no hook.",
    "otVJckYQ": "Custom item textures by rules: item rendering has no hook for that choice.",
    "TQTTVgYE": "Server-side rules and tools; Horizon's native mods are client-side.",
    "MOqt4Z5n": "A set of chat features (timestamps, hiding, copying); not started. The chat hooks used by More Chat History reach some of it.",
    "4alTd2ag": "Predicts eating on the client before the server confirms; needs entity and packet hooks.",
    "ZkHPmyRx": "Predicts end crystal hits on the client; needs entity and packet hooks.",
    "ETlrkaYF": "Window mode: Horizon owns its window and has its own fullscreen setting.",
    "FH4YDbIf": "Changes how the enchantment glint is drawn; item rendering has no hook for it.",
    "gD9OBTtF": "Scales the shield in the hand; the held-item transform has no hook.",
    "OVuFYfre": "Block entity rendering; no hook there.",
    "hbUrCPGQ": "Moves vanilla HUD elements; Horizon lays them out in functions that can only be replaced whole.",
    "wt23fpWX": "A framework for server-side bot players; not a client mod.",
    "hYTvGCAq": "Scales items in the hand and in the world; item rendering has no hook for it.",
    "yBW8D80W": "A dynamic lighting engine; lighting is Horizon's own and has no hook.",
    "uLbm7CG6": "Language file loading performance and fallbacks; Horizon loads languages its own way.",
    "1cENlqGZ": "Predicts mace smash hits on the client; needs entity and packet hooks.",
    "NRjRiSSD": "Fixes memory leaks in the Java game; Horizon is Rust.",
    "QdG47OkI": "Fixes gaps in block models; the mesher has no hook.",
    "hasdd01q": "World generation performance; Horizon has its own generator.",
    "Kw7Sm3Xf": "Performance and protocol changes for multiplayer servers; engine and network work.",
    "RSeLon5O": "Particle culling and spawn reduction; the particle system has no hook.",
    "nrikgvxm": "Changes rain particles; the particle system has no hook.",
    "3IuO68q1": "A resource-pack feature GUI for OptiFine alternatives; Horizon loads packs itself.",
    "WTVzusYD": "NVIDIA Reflex latency control through the GPU driver; Horizon renders with wgpu.",
    "fuuu3xnx": "A library for searchable lists in Fabric mod screens.",
    "GfoMZ9Ly": "Ported as Shield Statuses for your own shield only.",
    "diCZrsch": "Lets sliders be dragged past their ends; Horizon's option sliders have no hook.",
    "1cfO6J6t": "Changes the totem pop animation; the item activation animation has no hook.",
    "t5wuYk45": "A large collection of client tweaks (placement, restock, hotbar swap); not started, and most of it needs input and inventory hooks.",
    "JYQhtZtO": "A different renderer; Horizon renders with wgpu.",
    "c5VHeSai": "Predicts experience bottle throws on the client; needs entity and packet hooks.",
    "WwbubTsV": "Restores option files that the Java game resets; Horizon keeps its own options.",
    "4I1XuqiY": "Custom entity models in the OptiFine format; mob rendering has no hook.",
    "fxxUqruK": "A level-of-detail renderer for distant terrain; rendering and chunk storage, no hook.",
}


def classify(name, mid, has263):
    low = name.lower()
    if mid in PORTED:
        s, slug, note = PORTED[mid]
        return s, slug, note
    if not has263:
        return "no-263", "", NO263_REASONS.get(mid, "No Fabric build for 26.3, the game version Horizon implements.")
    if any(k in low for k in LIBRARY):
        return "library", "", "A library other Fabric mods load; Horizon's native mods do not use Fabric libraries."
    if any(k in low for k in ENGINE):
        return "engine", "", "Performance, server or engine work. Horizon is a native rewrite, so the engine already does its own."
    if any(k in low for k in PACK):
        return "pack", "", "Resource-pack, shader or rendering-asset territory: the registry lists those as packs, not native mods."
    if mid in SPECIFIC:
        return "hook", "", SPECIFIC[mid]
    for keys, reason in REASONS:
        if any(k in low for k in keys):
            return "hook", "", reason
    return "hook", "", DEFAULT_REASON


def licences():
    out = {}
    meta = json.load(open(INPUTS / "mods_metadata.json"))
    for k, v in meta.items():
        out[k] = (v.get("license") or "see project", v.get("slug"), v.get("title"))
    for line in open(INPUTS / "MOD-LICENSES.md"):
        m = re.match(r"\| (.+?) \| \[(\w{8})\]\(https://modrinth.com/mod/([\w-]+)\) \| `([^`]+)`", line)
        if m and m.group(2) not in out:
            out[m.group(2)] = (m.group(4), m.group(3), m.group(1))
    return out


def rows():
    lic = licences()
    result = []
    for line in open(INPUTS / "MOD-MATRIX.md"):
        if not (line.startswith("| ") and "modrinth:" in line):
            continue
        c = [x.strip() for x in line.strip().strip("|").split("|")]
        m = re.match(r"(.*) \(`modrinth:(\w+)`\)", c[0])
        name, mid = m.group(1), m.group(2)
        has263 = c[5] != "-"
        status, project, note = classify(name, mid, has263)
        licence, slug, title = lic.get(mid, ("see project", None, None))
        result.append(dict(name=title or name, raw=name, id=mid, slug=slug, licence=licence,
                           build=c[5].strip("`") if has263 else "-", status=status, project=project, note=note))
    return result


LABEL = {
    "ported": "Ported", "partial": "Partly ported", "builtin": "Built into Horizon", "hook": "Needs a hook point", "library": "Library",
    "engine": "Engine or server", "pack": "Pack or asset", "no-263": "No 26.3 build",
}
ORDER = ["ported", "partial", "builtin", "hook", "pack", "engine", "library", "no-263"]


def table(items):
    out = ["| Mod | Modrinth | Upstream licence | 26.3 build | Status | Note |", "|---|---|---|---|---|---|"]
    for r in sorted(items, key=lambda r: r["name"].lower()):
        link = f"[{r['slug'] or r['id']}](https://modrinth.com/mod/{r['slug'] or r['id']})"
        status = LABEL[r["status"]]
        if r["project"]:
            status += f" ([`{r['project']}`](mods/{r['project']}/))"
        out.append(f"| [{r['name']}](catalogue/{r['page']}.md) | {link} | `{r['licence']}` | `{r['build']}` | {status} | {r['note']} |")
    return "\n".join(out)


def slugify(r):
    base = (r["slug"] or r["id"]).lower()
    return re.sub(r"[^a-z0-9]+", "-", base).strip("-") or r["id"].lower()


INITIALS = re.compile(r"[A-Za-z0-9]+")
TAKES = {
    "hook": "A port needs data or a hook that Horizon's stable hook points do not give a mod. It can still be written as a raw-hook port pinned to one Horizon commit; that is tracked in the roadmap.",
    "pack": "This is a resource pack, shader, theme or model asset. The Horizon registry lists those as packs, not native mods, so there is no Rust to write.",
    "engine": "This changes how the game engine performs or how a server runs. Horizon is a native rewrite with its own engine, so there is nothing to patch in.",
    "library": "This is a library that other Fabric mods load. Native Horizon mods are self-contained Rust and do not use Fabric libraries.",
    "no-263": "The upstream project has no build for Minecraft 26.3, the version Horizon implements. That does not stop a native port by itself; the reason it is not ported is on its page.",
    "partial": "Only part of this fits the hook points; see the project for what is in and what is missing.",
    "builtin": "Horizon already does this itself, so a mod would only duplicate it.",
    "ported": "Ported: a project in this repository re-implements it.",
}


def icon_svg(name, status):
    hue = sum(map(ord, name)) * 37 % 360
    words = INITIALS.findall(name)
    letters = ("".join(w[0] for w in words[:2]) or "?").upper()
    ring = {"ported": "#7ee0a8", "partial": "#ffd37a"}.get(status, "#6b4bff")
    return f"""<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64" role="img" aria-label="{name} placeholder icon">
  <rect x="1" y="1" width="62" height="62" rx="14" fill="hsl({hue} 38% 17%)" stroke="{ring}" stroke-opacity=".6" stroke-width="2"/>
  <text x="32" y="40" text-anchor="middle" font-family="ui-sans-serif,system-ui,sans-serif" font-size="24" font-weight="700" fill="hsl({hue} 80% 85%)">{letters}</text>
</svg>
"""


def write_pages(items):
    out = ROOT / "catalogue"
    icons = ROOT / "assets/icons/catalogue"
    # The committed icons in assets/icons/catalogue are the source of truth: nothing here deletes or refetches them
    # (tools/gen/icons.py does that, on purpose). Only the pages and the placeholder tiles are rewritten.
    out.mkdir(parents=True, exist_ok=True)
    icons.mkdir(parents=True, exist_ok=True)
    for f in out.glob("*.md"):
        f.unlink()
    for f in icons.glob("*.svg"):
        f.unlink()
    seen = {}
    for r in items:
        s = slugify(r)
        seen[s] = seen.get(s, 0) + 1
        if seen[s] > 1:
            s += "-" + r["id"].lower()
        r["page"] = s
        if (icons / f"{s}.png").exists():
            r["icon"] = f"catalogue/{s}.png"
            real = True
        else:
            (icons / f"{s}.svg").write_text(icon_svg(r["name"], r["status"]))
            r["icon"] = f"catalogue/{s}.svg"
            real = False
        cr = CREDITS.get(r["id"], {})
        who = ", ".join(cr.get("authors") or []) or "its authors"
        icon_note = (f"The icon above is the project's own icon from Modrinth, by {who}, shown only to identify the mod; it is not covered by this repository's licence "
                     f"and comes down on request (see [DMCA and takedowns](../docs/dmca.md)).") if real else "This project has no icon on Modrinth, so the tile above is a placeholder drawn for this library."
        proj = f"\n**Project:** [`{r['project']}`](../mods/{r['project']}/) - [docs](../docs/mods/{r['project']}.md)\n" if r["project"] else ""
        title = r["name"]
        (out / f"{s}.md").write_text(f"""# {title}

![{title} icon](../assets/icons/{r['icon']})

**Status:** {LABEL[r['status']]} - **Upstream licence:** `{r['licence']}` - **26.3 build:** `{r['build']}` - [Modrinth](https://modrinth.com/mod/{r['slug'] or r['id']})
{proj}
## Where it stands

{r['note']}

{TAKES[r['status']]}

## Credit

{title} belongs to {who}; see its Modrinth page. No code or text from it is copied here. {icon_note}
Back to the [catalogue](../MODS.md).
""")


def main():
    items = rows()
    write_pages(items)
    counts = {k: sum(1 for r in items if r["status"] == k) for k in LABEL}
    summary = " · ".join(f"**{counts[k]}** {LABEL[k].lower()}" for k in ORDER if counts[k])
    md = f"""# Mod catalogue

Every mod in the Tempest project's cross-pack matrix ([`docs/MOD-MATRIX.md`](https://github.com/pxdns/patcher/blob/main/docs/MOD-MATRIX.md)
in `pxdns/patcher`): **{len(items)} mods**, {summary}.

Horizon implements Minecraft **26.3**, so the 26.3 column decides what can be ported at all. A native Horizon mod can
only use what Horizon's [native hook points](docs/hook-points.md) expose (frame, tick, HUD, key, wheel, command, zoom,
minimap, mods list, settings and the world lifecycle, plus the item panel and recipes), which is why most Fabric mods that
rewrite rendering, networking or inventories wait for more hook points. The status below says which case each mod is in;
nothing is marked ported unless a project in [`mods/`](mods/) builds and passes its tests.

## Status

| Status | Meaning |
|---|---|
| Ported | A project in `mods/` implements it for Horizon, independently of the upstream code |
| Partly ported | Only the part the hook points allow is in a project |
| Built into Horizon | Horizon ships it as one of its own extras |
| Needs a hook point | Wanted, but Horizon does not expose what it needs yet |
| Pack or asset | Belongs in the registry as a resource, shader or data pack |
| Engine or server | Performance or server work that Horizon's own engine replaces |
| Library | A helper other Fabric mods load; native mods do not use it |
| No 26.3 build | The upstream mod has no build for Horizon's game version |

Upstream licences are copied from each project's Modrinth page. They are listed for credit and to show that this repository
copies no upstream code or art: every port is a clean re-implementation of the behaviour.

"""
    for k in ORDER:
        group = [r for r in items if r["status"] == k]
        if group:
            md += f"## {LABEL[k]} ({len(group)})\n\n{table(group)}\n\n"
    (ROOT / "MODS.md").write_text(md)
    data = {"counts": counts, "labels": LABEL, "mods": items}
    (ROOT / "site").mkdir(exist_ok=True)
    (ROOT / "site/data").mkdir(exist_ok=True)
    (ROOT / "site/data/mods.json").write_text(json.dumps(data, indent=1))
    print(counts, len(items))

    # MODULES.md: Tempest's own module list and where each stands on Horizon.
    sup = json.load(open(INPUTS / "supports.json"))
    mods26 = sorted((INPUTS / "modules-26.2.txt").read_text().split())
    plan = {
        "zoomify": ("ported", "zoomify"), "sprint-display": ("ported", "sprint-indicator"), "customhud": ("ported", "custom-hud"),
        "fullbright": ("builtin", ""), "freecam": ("hook", ""), "invmove": ("hook", ""), "no-death-animation": ("hook", ""),
        "item-highlighter": ("hook", ""), "morechathistory": ("hook", ""),
    }
    lines = ["| Tempest module | Horizon |", "|---|---|"]
    for name in sorted(set(sup["modules"]) | set(mods26)):
        st, proj = plan.get(name, ("hook", ""))
        if st == "ported":
            cell = f"Ported: [`{proj}`](mods/{proj}/)"
        elif st == "builtin":
            cell = "Horizon has its own fullbright setting; no mod needed"
        else:
            cell = "Needs a hook point"
        lines.append(f"| `{name}` | {cell} |")
    mm = f"""# Modules

## Tempest's modules

The Tempest client (`pxdns/patcher`) groups its features into modules. This is the full list from the 26.2 module docs
and the 1.16.5 port table, with where each one stands on Horizon. The 1.16.5 browser port keeps three modules working
(Fullbright, InvMove, No Death Animation) plus the item-scale, Clean F3 and hit-crosshair hooks; see
[`docs/1.16.5/PORT-STATUS.md`](https://github.com/pxdns/patcher/blob/main/docs/1.16.5/PORT-STATUS.md).

{chr(10).join(lines)}

Two projects here have no Tempest module behind them: `cps-counter` and `info-hud` are small utilities written for this
library (`info-hud` follows MiniHUD's info lines, `cps-counter` the usual PvP readout).

## The Horizon organisation

[`github.com/horizon-rust`](https://github.com/horizon-rust) holds eight public repositories:

| Repository | What it is |
|---|---|
| [`horizon`](https://github.com/horizon-rust/horizon) | The game, launcher, server and tools: a rewrite of Minecraft Java 26.3 in Rust (MIT) |
| [`horizon-mods`](https://github.com/horizon-rust/horizon-mods) | The registry of reviewed mods, modpacks and resource and shader packs the launcher lists; also Horizon's own native mods |
| [`horizon-site`](https://github.com/horizon-rust/horizon-site) | The website at playhorizon.dev |
| [`horizon-mixins`](https://github.com/horizon-rust/horizon-mixins) | Compile-time source mixins for Horizon, by levzzz5154 (MIT); the tool every native mod is built with |
| [`horizon-native-mod-template`](https://github.com/horizon-rust/horizon-native-mod-template) | The template these projects follow |
| [`horizon-server`](https://github.com/horizon-rust/horizon-server) | A Minecraft Java 26.3 server written in Rust |
| [`kira-pumpkin`](https://github.com/horizon-rust/kira-pumpkin) | Kira, the Discord bot, answering in a Pumpkin server's chat |
| [`.github`](https://github.com/horizon-rust/.github) | The organisation profile |

Horizon's own native mods in `horizon-mods/native/` are `coords-hud`, `fps-overlay`, `keystrokes`, `minimap`, `zoom`,
`recipes`, `smp` and `voicechat-server`. This library adds independent mods beside them; none of them replaces those.

Horizon is an unofficial project. It is not approved by or associated with Mojang or Microsoft.
"""
    (ROOT / "MODULES.md").write_text(mm)


main()
