#!/usr/bin/env python3
"""Builds the horizon-mods-lib site into public from docs/*.md, MODS.md, MODULES.md and the mod projects.

    python3 -m venv .venv && .venv/bin/pip install markdown && .venv/bin/python site/build.py (output: public/)

Marketing pages are written here; documentation pages are the repository's own Markdown, so they read the same on GitHub.
"""
import hashlib
import html
import json
import posixpath
import re
import shutil
import tomllib
from pathlib import Path

import markdown

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "public"
REPO = "https://github.com/pxdns/horizon-mods-lib"
SITE = ""  # deployed on Vercel; every link in the site is relative, so no base URL is needed

DATA = json.loads((ROOT / "site/data/mods.json").read_text())
COUNTS = DATA["counts"]
TOTAL = len(DATA["mods"])
BUILDS = TOTAL - COUNTS["no-263"]


def project(slug):
    root = ROOT / "mods" / slug
    bundle = next(p for p in (root / "mixins").iterdir() if (p / "horizon-mixin.toml").is_file())
    manifest = tomllib.loads((bundle / "horizon-mixin.toml").read_text())
    cargo = tomllib.loads((root / "Cargo.toml").read_text())
    runtime = (bundle / "runtime.rs").read_text()
    name = re.search(r'pub const NAME: &str = "([^"]+)"', runtime).group(1)
    return dict(slug=slug, id=manifest["mod"]["id"], version=manifest["mod"]["version"], name=name,
                description=cargo["package"]["description"], hooks=[i["id"] for i in manifest.get("injection", [])],
                tests=runtime.count("#[test]"), lines=runtime.count("\n"))


PROJECTS = [project(p.name) for p in sorted((ROOT / "mods").iterdir()) if (p / "compatibility.json").is_file()]
ORDER = ["fps-display", "sprint-indicator", "key-overlay", "cps-counter", "info-hud", "custom-hud", "zoomify", "simple-zoom", "waypoints",
         "armor-hud", "potion-counter", "totem-counter", "effect-timer", "apple-skin", "pick-up-notifier",
         "freelook", "freecam", "snappy-tappy", "better-hurt-cam",
         "more-chat-history", "chat-notify", "chat-animation", "durability-tooltip",
         "crosshair-indicator", "entity-crosshair", "shulker-box-tooltip", "combat-hitboxes"]
PROJECTS.sort(key=lambda p: ORDER.index(p["slug"]) if p["slug"] in ORDER else 99)

DOC_NAV = [
    ("Getting started", ["docs/introduction.md", "docs/install.md", "docs/build.md"]),
    ("Mods", [f"docs/mods/{p['slug']}.md" for p in PROJECTS]),
    ("Reference", ["docs/hook-points.md", "docs/anatomy.md", "docs/settings.md", "docs/releases.md", "docs/compatibility.md"]),
    ("Catalogue", ["MODS.md", "MODULES.md"]),
    ("Project", ["docs/credits.md", "docs/licensing.md", "docs/dmca.md", "docs/trademarks.md", "docs/security.md",
                 "docs/contributing.md", "docs/changelog.md"]),
]
SPECIAL = {"MODS.md": "docs/catalogue.html", "MODULES.md": "docs/modules.html"}
SHORT = {"MODS.md": "Status of every mod", "MODULES.md": "Modules and Horizon"}
TOP = [("Home", "index.html"), ("Mods", "mods/index.html"), ("Why native", "why.html"), ("From Fabric", "from-fabric.html"),
       ("Trust", "trust.html"), ("Roadmap", "roadmap.html"), ("Play", "play.html"), ("Docs", "docs/introduction.html")]


def out_path(src):
    if src in SPECIAL:
        return SPECIAL[src]
    return src[:-3] + ".html"


def rel(from_page, to):
    d = posixpath.dirname(from_page) or "."
    return posixpath.relpath(to, d)


def fix_links(body, src, page):
    """Rewrites links of a Markdown page: .md to .html, repo directories and images to site paths."""
    base = posixpath.dirname(src)

    def sub(m):
        attr, url = m.group(1), m.group(2)
        if re.match(r"^(https?:|mailto:|#)", url):
            return m.group(0)
        path, _, frag = url.partition("#")
        target = posixpath.normpath(posixpath.join(base, path)) if path else ""
        if target in SPECIAL or target.endswith(".md"):
            new = rel(page, out_path(target))
        elif target.startswith("assets/"):
            new = rel(page, target)
        elif re.match(r"^mods/[\w-]+/?$", target):
            new = rel(page, f"docs/mods/{target.split('/')[1]}.html")
        else:
            new = f"{REPO}/tree/main/{target}" if target else url
        return f'{attr}="{new}{"#" + frag if frag else ""}"'

    return re.sub(r'(href|src)="([^"]+)"', sub, body)


def shell(page, title, body, active=None, description="", doc=False, root=None):
    if root is None:
        root = rel(page, "index.html")
        root = root[:-len("index.html")] if root.endswith("index.html") else root
        root = "" if root in ("", ".") else root
    nav = "".join(f'<a href="{root}{u}"{" aria-current=page" if u == active else ""}>{html.escape(n)}</a>' for n, u in TOP)
    desc = html.escape(description or "Native mods for Horizon, ported to Rust: one small tested project per mod, each released automatically.")
    return f'''<!doctype html>
<html lang="en" data-root="{root or './'}">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{html.escape(title)}</title>
<meta name="description" content="{desc}">
<meta name="theme-color" content="#100a1f">
<link rel="icon" href="{root}assets/icons/horizon-mods-lib.svg" type="image/svg+xml">
<link rel="stylesheet" href="{root}assets/site.css">
</head>
<body>
<header class="top"><div class="wrap">
<a class="brand" href="{root}index.html"><img src="{root}assets/icons/horizon-mods-lib.svg" alt="" width="28" height="28">horizon-mods-lib</a>
<nav aria-label="Main">{nav}</nav><span class="grow"></span>
<div class="search"><input id="q" type="search" placeholder="Search  /" aria-label="Search the site" autocomplete="off"><ul id="results" hidden></ul></div>
<a class="gh" href="{REPO}">GitHub</a>
</div></header>
{body}
<footer class="site"><div class="wrap">
<div class="cols">
<div><h4>Library</h4><a href="{root}mods/index.html">The mods</a><a href="{root}docs/catalogue.html">Status of every mod</a><a href="{root}roadmap.html">Roadmap</a></div>
<div><h4>Docs</h4><a href="{root}docs/install.html">Install a mod</a><a href="{root}docs/hook-points.html">Hook points</a><a href="{root}docs/releases.html">Releases</a></div>
<div><h4>Project</h4><a href="{root}docs/credits.html">Credits</a><a href="{root}docs/licensing.html">Licensing</a><a href="{root}docs/dmca.html">DMCA and takedowns</a><a href="{root}docs/trademarks.html">Trademarks and disclaimer</a></div>
<div><h4>Horizon</h4><a href="https://github.com/horizon-rust">The organisation</a><a href="https://github.com/horizon-rust/horizon">Horizon</a><a href="https://github.com/horizon-rust/horizon-mods">The registry</a></div>
</div>
<p>horizon-mods-lib is an unofficial community project, MIT licensed. It is not made or endorsed by the Horizon project, Mojang or
Microsoft. Minecraft is a trademark of Mojang Synergies AB. Mod names belong to their authors and are used only to say what a port is a port of.</p>
</div></footer>
<script src="{root}assets/site.js" defer></script>
</body>
</html>
'''


def icon_of(slug):
    """The mod's own SVG when it has one, else its catalogue icon."""
    if (ROOT / "assets/icons" / f"{slug}.svg").exists():
        return f"{slug}.svg"
    for m in DATA["mods"]:
        if m["project"] == slug:
            return m["icon"]
    return "horizon-mods-lib.svg"


def card(p, root=""):
    return (f'<a class="card" href="{root}docs/mods/{p["slug"]}.html"><img src="{root}assets/icons/{icon_of(p["slug"])}" alt="" width="48" height="48">'
            f'<h3>{html.escape(p["name"])}</h3><p>{html.escape(p["description"])}</p>'
            f'<div class="meta">{p["id"]} {p["version"]} · {p["tests"]} tests</div></a>')


def write(page, text):
    path = OUT / page
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)


def strip_tags(s):
    return re.sub(r"\s+", " ", re.sub(r"<[^>]+>", " ", html.unescape(s))).strip()


SEARCH = []

# ------------------------------------------------------------------------------------------------ documentation pages
flat = [s for _, group in DOC_NAV for s in group]
md = markdown.Markdown(extensions=["tables", "fenced_code", "toc", "attr_list"], extension_configs={"toc": {"permalink": False, "toc_depth": "2-3"}})


def catalogue_pages():
    """One page per catalogued mod (catalogue/<slug>.md): same chrome as the docs, not in the sidebar."""
    for path in sorted((ROOT / "catalogue").glob("*.md")):
        src = f"catalogue/{path.name}"
        page = out_path(src)
        text = path.read_text()
        md.reset()
        body = fix_links(md.convert(text), src, page)
        title = re.match(r"# (.+)", text).group(1)
        side = ""
        for group, items in DOC_NAV:
            side += f"<h5>{group}</h5>"
            for s2 in items:
                label = SHORT.get(s2) or re.match(r"# (.+)", (ROOT / s2).read_text()).group(1)
                cur = " aria-current=page" if s2 == "MODS.md" else ""
                side += f'<a href="{rel(page, out_path(s2))}"{cur}>{html.escape(label)}</a>'
        doc = f'''<div class="docs">
<aside class="side" aria-label="Documentation"><details><summary>Documentation</summary><div class="links">{side}</div></details></aside>
<main class="doc">
<div class="crumbs"><a href="{rel(page, 'docs/introduction.html')}">Docs</a> / <a href="{rel(page, 'docs/catalogue.html')}">Catalogue</a></div>
{body}
<p class="edit"><a href="{REPO}/edit/main/{src}">Edit this page on GitHub</a></p>
</main><aside class="toc"></aside></div>'''
        write(page, shell(page, f"{title} · horizon-mods-lib", doc, active="docs/introduction.html", description=strip_tags(body)[:150]))
        SEARCH.append(dict(title=title, url=page, text=strip_tags(body)[:300], excerpt=strip_tags(body)[:90]))


def docs():
    for i, src in enumerate(flat):
        page = out_path(src)
        text = (ROOT / src).read_text()
        md.reset()
        body = md.convert(text)
        title = re.match(r"# (.+)", text).group(1)
        body = fix_links(body, src, page)
        def flatten(tokens):
            for t in tokens:
                yield t
                yield from flatten(t.get("children", []))

        toc = "".join(f'<a class="{"sub" if t["level"] > 2 else ""}" href="#{t["id"]}">{html.escape(t["name"])}</a>'
                      for t in flatten(md.toc_tokens))
        side = ""
        for group, items in DOC_NAV:
            side += f"<h5>{group}</h5>"
            for s in items:
                label = SHORT.get(s) or re.match(r"# (.+)", (ROOT / s).read_text()).group(1)
                cur = " aria-current=page" if s == src else ""
                side += f'<a href="{rel(page, out_path(s))}"{cur}>{html.escape(label)}</a>'
        pager = '<nav class="pager" aria-label="Previous and next">'
        if i > 0:
            ps = flat[i - 1]
            pl = SHORT.get(ps) or re.match(r"# (.+)", (ROOT / ps).read_text()).group(1)
            pager += f'<a class="prev" href="{rel(page, out_path(ps))}"><small>Previous</small>{html.escape(pl)}</a>'
        if i < len(flat) - 1:
            ns = flat[i + 1]
            nl = SHORT.get(ns) or re.match(r"# (.+)", (ROOT / ns).read_text()).group(1)
            pager += f'<a class="next" href="{rel(page, out_path(ns))}"><small>Next</small>{html.escape(nl)}</a>'
        pager += "</nav>"
        group = next(g for g, items in DOC_NAV if src in items)
        extra = ""
        if src == "MODS.md":
            chips = '<button data-s="all" aria-pressed="true">All</button>' + "".join(
                f'<button data-s="{DATA["labels"][k].lower()}" aria-pressed="false">{DATA["labels"][k]} ({n})</button>'
                for k, n in COUNTS.items() if n)
            extra = (f'<div class="filters" id="filters">{chips}<input id="filter-text" type="search" placeholder="Filter by name or licence" '
                     f'aria-label="Filter the catalogue"><span class="count" id="count"></span></div>')
            body = body.replace("<h2 id=\"ported-5\"", extra + "<h2 id=\"ported-5\"", 1)
        doc = f'''<div class="docs">
<aside class="side" aria-label="Documentation"><details open><summary>Documentation</summary><div class="links">{side}</div></details></aside>
<main class="doc">
<div class="crumbs"><a href="{rel(page, 'docs/introduction.html')}">Docs</a> / {html.escape(group)}</div>
{body}
{pager}
<p class="edit"><a href="{REPO}/edit/main/{src}">Edit this page on GitHub</a></p>
</main>
<aside class="toc" aria-label="On this page"><h5>On this page</h5>{toc}</aside>
</div>'''
        write(page, shell(page, f"{title} · horizon-mods-lib", doc, active="docs/introduction.html",
                          description=strip_tags(body)[:150]))
        SEARCH.append(dict(title=title, url=page, text=strip_tags(body)[:4000], excerpt=strip_tags(body)[:90]))


# ------------------------------------------------------------------------------------------------ marketing pages
def read(path):
    return (ROOT / path).read_text()


def pages():
    n = len(PROJECTS)
    tests = sum(p["tests"] for p in PROJECTS)
    # ---- home
    home = f'''<main>
<section class="hero"><div class="wrap">
<h1 class="big rise" style="--delay:60ms">Your favourite mods, <span class="text-accent">rewritten in Rust.</span></h1>
<p class="lede rise" style="--delay:120ms">Horizon is a rewrite of Minecraft: Java in Rust, and its mods are not plugins: they are source that gets merged into the
game and compiled with it. This is a library of them. {n} are done, each one a tiny project with its own tests, its own
release and a file you can read in five minutes.</p>
<div class="btns rise" style="--delay:180ms"><a class="btn primary" href="https://github.com/pxdns/horizon-mods-lib/releases/download/all-mods/horizon-mods-all.zip">Download all {n} mods</a><a class="btn" href="play.html">Play Horizon</a><a class="btn" href="mods/index.html">See the mods</a><a class="btn" href="docs/install.html">Install guide</a><a class="btn" href="from-fabric.html">How a Fabric mod becomes Rust</a></div>
<div class="stats rise" style="--delay:260ms">
<div class="stat"><b>{n}</b><span>mods shipped</span></div>
<div class="stat"><b>{tests}</b><span>unit tests across them</span></div>
<div class="stat"><b>0</b><span>dependencies in any mod</span></div>
<div class="stat"><b>{BUILDS}</b><span>of {TOTAL} catalogued mods have a 26.3 build</span></div>
</div></div></section>

<section class="block"><div class="wrap">
<h2 class="sec">A mod is part of the game, not a guest in it</h2>
<p class="sub">Fabric loads Java into a running JVM. Horizon has no JVM. A native mod is Rust that
<a href="https://github.com/horizon-rust/horizon-mixins">horizon-mixins</a> merges into Horizon's source before the compiler sees it, and the
launcher builds a game for your instance with the mods already inside.</p>
<ol class="steps">
<li><b>Hook</b><span>Horizon keeps a short list of stable hook points: empty functions the game calls every frame, tick, key press, HUD draw.</span></li>
<li><b>Merge</b><span>A <code>horizon-mixin.toml</code> says which hook runs which helper, and adds the mod's <code>runtime.rs</code> to the game as a module.</span></li>
<li><b>Build</b><span>The launcher compiles the modded source for your instance. First build 20 to 30 minutes, later ones faster.</span></li>
<li><b>Play</b><span>Switch the mod on or off in Options, Mods. It has a settings page and, if it needs one, a key.</span></li>
</ol></div></section>

<section class="block"><div class="wrap">
<h2 class="sec">The {n} mods</h2>
<p class="sub">Each folder in <code>mods/</code> is a complete project: logic, tests, pins, licence, a release workflow. They are ports, written from how the originals behave and not from their code.</p>
<div class="grid">{"".join(card(p) for p in PROJECTS)}</div>
<p style="margin-top:1.5rem"><a href="docs/catalogue.html">All {TOTAL} mods in the catalogue, with where each one stands &rarr;</a></p>
</div></section>

<section class="block"><div class="wrap split">
<div><h2 class="sec">What we do not pretend</h2>
<p class="sub">{COUNTS["hook"]} of the {TOTAL} catalogued mods are wanted but cannot be written yet, because the hook points do not give a mod an inventory, effects, incoming chat or the camera. Nothing here is marked ported
unless a project builds, passes its tests and type-checks against a real Horizon.</p>
<div class="btns"><a class="btn" href="roadmap.html">What Horizon needs to expose</a><a class="btn" href="trust.html">How it is kept honest</a></div></div>
<div class="note"><b>Verified today:</b> all {n} mods apply on Horizon <code>ba6344a</code> with horizon-mixins <code>d9c83d7</code> and the game type-checks with warnings denied. They are
unit-tested, formatted and clippy-clean. They have not been played in a live game by this project's CI, which cannot run a GPU build.</div>
</div></section>
</main>'''
    write("index.html", shell("index.html", "horizon-mods-lib · native mods for Horizon, ported to Rust", home, active="index.html"))

    # ---- mods gallery
    gallery = f'''<main><section class="hero" style="padding-bottom:2rem"><div class="wrap">
<span class="eyebrow">The library</span>
<h1 class="big" style="max-width:22ch">{n} mods, <span class="text-accent">one project each.</span></h1>
<p class="lede">Every card is a folder in <code>mods/</code> with its own logic, tests and release. Click through for what it does, its settings and who it is ported from.</p>
</div></section>
<section class="block" style="border:0"><div class="wrap"><div class="grid">{"".join(card(p, "../") for p in PROJECTS)}</div>
<h2 class="sec" style="margin-top:3rem">Beyond these</h2>
<p class="sub">The catalogue lists all {TOTAL} mods of the Tempest cross-pack matrix: {COUNTS["ported"] + COUNTS["partial"]} ported or partly ported, {COUNTS["hook"]} waiting on a hook point, {COUNTS["pack"]} that are really packs, {COUNTS["engine"]} that Horizon's engine does itself, {COUNTS["library"]} libraries and {COUNTS["no-263"]} with no 26.3 build.</p>
<a class="btn primary" href="../all.html">See every mod</a> <a class="btn" href="../docs/catalogue.html">Open the catalogue table</a></div></section></main>'''
    write("mods/index.html", shell("mods/index.html", "The mods · horizon-mods-lib", gallery, active="mods/index.html"))

    # ---- why native
    why = '''<main><section class="hero" style="padding-bottom:1.5rem"><div class="wrap">
<span class="eyebrow">Why native</span>
<h1 class="big" style="max-width:20ch">Mods you can <span class="text-accent">read, test and trust.</span></h1>
<p class="lede">A plugin system is a promise that code you did not read will behave. Horizon's native mods make a smaller promise and keep it: the bundle is source, and the source is short.</p>
</div></section>
<section class="block"><div class="wrap prose-wide">
<h2 class="sec">There is nothing to load</h2>
<p>A Fabric mod is a jar the game opens at start-up, patches with bytecode tricks and runs inside the same JVM. Horizon is native
Rust, so a mod cannot be loaded that way, and does not need to be. <strong>horizon-mixins</strong> works on the game's source tree: it parses the Rust,
applies the injections a mod declares, and the compiler sees one program. A hook is a function call that was written into the game before it
was built. No reflection, no class loader, nothing that decides at run time what runs.</p>
<h2 class="sec">The bundle is the source</h2>
<p>A <code>.hnmod</code> is a zip of <code>.rs</code> and <code>.toml</code> files and a licence. Nothing compiled is in it. The launcher checks it
without running anything, refuses links, odd paths and anything beyond the bundle layout, and then builds it for your instance behind a red warning,
because native code has the player's rights. Horizon's registry lists a native mod only after a maintainer has read its source. You can do the same in
the time it takes to drink a coffee: here, a mod's logic is one file of a few hundred lines at most.</p>
<h2 class="sec">Logic that does not need the game to test</h2>
<p>Each project's Cargo library <em>is</em> its <code>runtime.rs</code>. The game's types never appear in it; the hook helpers read what they need and pass plain
values in. That is why a zoom's wheel limits, a waypoint's bearing maths and a speedometer's smoothing have unit tests that run in under a second, and
why a bug in them is a failed test instead of a crashed game.</p>
<h2 class="sec">Small on purpose</h2>
<p>There are no dependencies in any of these projects. There is no <code>unsafe</code>, no network access and no process spawning. A mod that is switched off in Options
does nothing at all until the next build leaves it out. Settings are one file per key in the mod's own folder.</p>
<h2 class="sec">What it costs</h2>
<div class="note"><b>Build time.</b> Every set of mods is a build of the game: the first one downloads a toolchain and takes 20 to 30 minutes, a rebuild of one changed helper takes a few
minutes. <b>Reach.</b> A native mod can only do what a hook point allows. <b>Servers.</b> Native mods are not built for dedicated servers yet; on a server you join, only client-side hooks do anything.
<b>Maturity.</b> Native mods are experimental in Horizon. We say so on every page that matters.</div>
<div class="btns" style="margin-top:2rem"><a class="btn primary" href="from-fabric.html">See how a port goes</a><a class="btn" href="docs/hook-points.html">The hook points</a></div>
</div></section></main>'''
    write("why.html", shell("why.html", "Why native mods · horizon-mods-lib", why, active="why.html"))

    # ---- from fabric
    wheel = read("mods/zoomify/mixins/zoomify/wheel.rs").strip()
    key = read("mods/zoomify/mixins/zoomify/key.rs").strip()
    esc = html.escape
    bars = "".join(f'<div class="bar"><span>{DATA["labels"][k]}</span><i style="width:{max(2, round(COUNTS[k] / TOTAL * 100 * 3))}%"></i><em>{COUNTS[k]}</em></div>'
                   for k in ["ported", "partial", "hook", "pack", "engine", "library", "no-263"])
    fab = f'''<main><section class="hero" style="padding-bottom:1.5rem"><div class="wrap">
<span class="eyebrow">Porting</span>
<h1 class="big" style="max-width:21ch">From Fabric to Horizon, <span class="text-accent">one behaviour at a time.</span></h1>
<p class="lede">The Tempest project has a matrix of {TOTAL} Fabric mods across five game versions. This is what happens when you take that list to a game written in Rust.</p>
</div></section>
<section class="block"><div class="wrap">
<h2 class="sec">Start with the list</h2>
<p class="sub">Horizon implements Minecraft 26.3, so the first filter is blunt: {COUNTS["no-263"]} of the {TOTAL} mods have no 26.3 build and are out. Of the remaining {BUILDS}, most are libraries, performance mods a native engine does not need, or packs. What is left is a short list of mods that
are about what you see and press.</p>
<div class="bars">{bars}</div></div></section>
<section class="block"><div class="wrap split">
<div><h2 class="sec">Rule one: copy the behaviour, never the code</h2>
<p class="sub">Upstream mods are LGPL, GPL, MIT, all-rights-reserved. A port that contains none of their code owes them credit and nothing else, and it owes the reader an honest account of what it does. So each port starts from using the original, writing down what it does, and writing Rust. The <a href="docs/credits.html">credits</a> name every original and its licence.</p>
<h2 class="sec" style="margin-top:2rem">Rule two: the logic gets tests</h2>
<p class="sub">Zoomify's wheel clamps between 1.25x and 16x. A test fails if it does not. Waypoints compute a vanilla yaw to a block; one test pins north, south, east and west. The hook helpers cannot be tested without the game, so they are kept to a few lines, and CI type-checks them against the real Horizon.</p>
<h2 class="sec" style="margin-top:2rem">Rule three: say what is missing</h2>
<p class="sub">Xaero's minimap is only partly ported: Horizon ships its own minimap, and there is no map-screen hook, so what is left is the waypoints idea. The catalogue says so.</p></div>
<div><h2 class="sec">A port, end to end</h2>
<p class="sub">Zoomify holds a key and zooms. On Horizon that is three hooks. The key hook is this much code:</p>
<pre class="code"><code>{esc(key)}</code></pre>
<p class="sub">And the wheel claims its steps while the zoom is on, so the hotbar does not move:</p>
<pre class="code"><code>{esc(wheel)}</code></pre>
<p class="sub">Everything else (the levels, the easing flag, the turn scaling, the settings buttons) lives in <code>runtime.rs</code> and has seven tests.</p></div>
</div></section></main>'''
    fab = fab.replace('<pre class="code"><code>', '<pre style="background:#0b0716;border:1px solid var(--line);border-radius:1rem;padding:1rem 1.2rem;overflow:auto;font-size:.82rem"><code>')
    write("from-fabric.html", shell("from-fabric.html", "From Fabric to Horizon · horizon-mods-lib", fab, active="from-fabric.html"))

    # ---- trust
    trust = f'''<main><section class="hero" style="padding-bottom:1.5rem"><div class="wrap">
<span class="eyebrow">Trust</span>
<h1 class="big" style="max-width:20ch">Built so that you <span class="text-accent">do not have to take our word.</span></h1>
<p class="lede">A native mod runs with your rights. Here is everything this library does about that, and what it asks of you.</p></div></section>
<section class="block"><div class="wrap"><div class="grid">
<div class="card"><h3>Source, not binaries</h3><p>A <code>.hnmod</code> holds Rust and TOML. Read it before you build it; the launcher validates it without running anything.</p></div>
<div class="card"><h3>Pinned and reproducible</h3><p>Exact Horizon and horizon-mixins commits in every project. The package is deterministic: same commit, same SHA-256, published beside the file.</p></div>
<div class="card"><h3>Clean-room ports</h3><p>Written from behaviour. No upstream code, art or text, and the <a href="docs/credits.html">credits</a> name every original and its licence.</p></div>
<div class="card"><h3>Icons, credited</h3><p>Catalogue icons are each mod's own, from Modrinth, named to their authors on every page and gone on request. The nine projects have their own drawn icons.</p></div>
<div class="card"><h3>Nothing phones home</h3><p>No network access in any mod, no dependencies, no analytics on this site, no third-party fonts or scripts.</p></div>
<div class="card"><h3>A way to say stop</h3><p>If something here is yours and should not be, the <a href="docs/dmca.html">DMCA page</a> says how to ask, and what happens next.</p></div>
</div>
<div class="note" style="margin-top:2rem"><b>The honest limits.</b> This is an unofficial project, not endorsed by the Horizon project, Mojang or Microsoft. Native mods are experimental in Horizon. The mods are unit-tested and type-checked against Horizon in CI, and have not been played through a real window by CI. Read the <a href="docs/security.html">security notes</a> and the <a href="docs/trademarks.html">disclaimer</a>.</div>
</div></section></main>'''
    write("trust.html", shell("trust.html", "Trust · horizon-mods-lib", trust, active="trust.html"))


    # ---- play
    offline = ROOT / "site/static/horizon-offline.html"
    size_mb = offline.stat().st_size / 1_000_000
    digest = hashlib.sha256(offline.read_bytes()).hexdigest()
    play = f'''<main><section class="hero" style="padding-bottom:1.5rem"><div class="wrap">
<h1 class="big" style="max-width:20ch">Play Horizon <span class="text-accent">in your browser.</span></h1>
<p class="lede">A full build of Horizon with its game files included: open it and it runs, with the game data served from this same site. It needs a browser with WebGPU.</p>
<div class="btns"><a class="btn primary" href="horizon-offline.html">Play Horizon (offline)</a><a class="btn" href="horizon-offline.html" download>Download the file</a></div>
</div></section>
<section class="block"><div class="wrap">
<div class="card" style="max-width:46rem"><h3 style="margin-top:0">Horizon (offline)</h3>
<p>The page ({size_mb:.1f} MB) plus its wasm and resource package in <code>horizon-offline/</code> (about 115 MB, fetched from this site only).</p>
<div class="meta" style="word-break:break-all">horizon-offline.html &middot; SHA-256 {digest}</div></div>
</div></section>
<section class="block"><div class="wrap split">
<div><h2 class="sec">How it starts</h2>
<ol class="steps" style="grid-template-columns:1fr">
<li><b>Open the file</b><span>It asks for a folder of game resources: the one that contains <code>boot.tar.gz</code>, <code>manifest.txt</code> and the <code>res</code> folder.</span></li>
<li><b>Select it</b><span>Your browser may ask you to confirm an upload; per the page, nothing leaves your computer. The resources are saved in this browser the first time, so you pick the folder once.</span></li>
<li><b>Choose what to keep</b><span>You pick what is saved in the browser (sounds are optional; only UK English is kept). Game saves are kept in the browser too.</span></li>
</ol></div>
<div><h2 class="sec">What it needs</h2>
<div class="note" style="margin-bottom:1.5rem"><b>WebGPU.</b> The file says so itself when it cannot find one. It runs in Chrome and Edge 113 or later on Windows, macOS and ChromeOS; Chrome 121 or later on Android; Safari 26 on macOS, iOS and iPadOS; and Firefox 141 or later on Windows. On Linux, Chrome, Edge and Brave need WebGPU switched on with <code>--enable-unsafe-webgpu --enable-features=Vulkan</code>.</div>
<h2 class="sec">What to know</h2>
<div class="note"><b>You bring the resources.</b> The file contains none of Minecraft's resources: the game reads the folder you select, which you prepare from your own copy of Minecraft. <b>No mods built in.</b> The mods in this library are compiled into a game by Horizon's launcher (<a href="docs/install.html">install guide</a>); this single file is a separate build, so do not expect them in it. <b>Unofficial.</b> This site is a community project; Horizon is not approved by or associated with Mojang or Microsoft.</div>
<p class="sub" style="margin-top:1.2rem">Horizon is MIT licensed. See <a href="docs/credits.html">credits</a> for what the file's own comments name, and the <a href="docs/trademarks.html">disclaimer</a>.</p></div>
</div></section></main>'''
    write("play.html", shell("play.html", "Play Horizon in your browser · horizon-mods-lib", play, active="play.html"))

    # ---- roadmap
    groups = {}
    for m in DATA["mods"]:
        if m["status"] == "hook":
            groups.setdefault(m["note"], []).append(m["name"])
    items = ""
    for note, names in sorted(groups.items(), key=lambda kv: -len(kv[1])):
        sample = ", ".join(sorted(re.sub(r"[-_]?(fabric|mc)?[\d.+]+.*$", "", x, flags=re.I) or x for x in names)[:8])
        items += f'<div class="card"><h3>{len(names)} mods</h3><p><b style="color:#fff">{html.escape(note)}</b></p><div class="meta">{html.escape(sample)}…</div></div>'
    road = f'''<main><section class="hero" style="padding-bottom:1.5rem"><div class="wrap">
<span class="eyebrow">Roadmap</span>
<h1 class="big" style="max-width:20ch">What is waiting on <span class="text-accent">Horizon, not on us.</span></h1>
<p class="lede">{COUNTS["hook"]} catalogued mods cannot be written yet. The reason is never effort: it is that a hook point does not exist. Hook points are only ever added, so each one that lands unblocks a group of them.</p>
</div></section>
<section class="block"><div class="wrap"><div class="grid">{items}</div>
<h2 class="sec" style="margin-top:3rem">How a hook point gets added</h2>
<p class="sub">It is a change to Horizon, not to a mod: an empty function the game calls at a fixed place, a bump of <code>NATIVE_HOOKS_API</code>, and an entry in <code>native-hooks.json</code>.
The most useful contribution this library can make is a pull request to <a href="https://github.com/horizon-rust/horizon">horizon-rust/horizon</a> that adds one, with a mod here that uses it.</p>
<div class="btns"><a class="btn primary" href="docs/hook-points.html">Hook points today</a><a class="btn" href="docs/contributing.html">Contributing</a></div></div></section></main>'''
    write("roadmap.html", shell("roadmap.html", "Roadmap · horizon-mods-lib", road, active="roadmap.html"))
    for page, title in [("index.html", "Home"), ("why.html", "Why native mods"), ("from-fabric.html", "From Fabric to Horizon"),
                        ("trust.html", "Trust"), ("roadmap.html", "Roadmap"), ("play.html", "Play"), ("mods/index.html", "The mods")]:
        t = strip_tags((OUT / page).read_text().split("<main>", 1)[1].split("</main>", 1)[0])
        SEARCH.append(dict(title=title, url=page, text=t[:4000], excerpt=t[:90]))


def everything():
    """Every catalogued mod as a card with its placeholder icon, filterable by status."""
    chips = '<button data-s="all" aria-pressed="true">All</button>' + "".join(
        f'<button data-s="{k}" aria-pressed="false">{DATA["labels"][k]} ({n})</button>' for k, n in COUNTS.items() if n)
    cards = ""
    for m in sorted(DATA["mods"], key=lambda r: r["name"].lower()):
        page = m["page"]
        target = f'docs/mods/{m["project"]}.html' if m["project"] else f"catalogue/{page}.html"
        cards += (f'<a class="card" data-status="{m["status"]}" data-name="{html.escape(m["name"].lower())}" href="{target}">'
                  f'<img src="assets/icons/{m["icon"]}" alt="" width="48" height="48">'
                  f'<h3>{html.escape(m["name"])}</h3><p>{html.escape(DATA["labels"][m["status"]])}</p></a>')
    body = f'''<main><section class="hero" style="padding-bottom:1.5rem"><div class="wrap">
<span class="eyebrow">The whole list</span>
<h1 class="big" style="max-width:22ch">All {TOTAL} mods, <span class="text-accent">and where each stands.</span></h1>
<p class="lede">Every mod in the Tempest cross-pack matrix has a page. Ported ones open their project; the rest say exactly why not yet. Icons are each mod's own, credited on its page.</p>
<div class="filters" id="grid-filters">{chips}<input id="grid-text" type="search" placeholder="Filter by name" aria-label="Filter mods"><span class="count" id="grid-count"></span></div>
</div></section>
<section class="block" style="border:0;padding-top:0"><div class="wrap"><div class="grid" id="grid">{cards}</div></div></section></main>'''
    write("all.html", shell("all.html", "All mods · horizon-mods-lib", body, active="mods/index.html"))
    SEARCH.append(dict(title="All mods", url="all.html", text="every mod in the catalogue", excerpt="Every mod and its status"))


def main():
    if OUT.exists():
        shutil.rmtree(OUT)
    OUT.mkdir(parents=True)
    shutil.copytree(ROOT / "assets/icons", OUT / "assets/icons")
    shutil.copy(ROOT / "site/assets/site.css", OUT / "assets/site.css")
    shutil.copy(ROOT / "site/assets/site.js", OUT / "assets/site.js")
    for extra in sorted((ROOT / "site/static").glob("*")):
        if extra.is_dir():
            shutil.copytree(extra, OUT / extra.name)
        else:
            shutil.copy(extra, OUT / extra.name)
    # A project whose Root Directory is `public` reads its headers from here.
    cfg = json.loads((ROOT / "vercel.json").read_text())
    for key in ("outputDirectory", "buildCommand", "installCommand"):
        cfg.pop(key, None)
    (OUT / "vercel.json").write_text(json.dumps(cfg, indent=2) + "\n")
    docs()
    catalogue_pages()
    pages()
    everything()
    (OUT / "search.json").write_text(json.dumps(SEARCH))
    (OUT / ".nojekyll").write_text("")
    (OUT / "404.html").write_text(shell("404.html", "Not found · horizon-mods-lib",
                                        '<main><section class="hero"><div class="wrap"><h1 class="big">Nothing here.</h1><p class="lede"><a href="/index.html">Back to the library.</a></p></div></section></main>',
                                        root="/"))
    print(f"{len(list(OUT.rglob('*.html')))} pages")


main()
