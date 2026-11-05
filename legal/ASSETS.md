# Assets

## Rules

1. **No Mojang/Microsoft assets in any repo, release, site or mirror.** That means game jars, textures, sounds, models,
   fonts, language files, shaders and anything extracted from them. The game takes them from the player's own copy
   (see the Horizon README's "you need your own legally obtained copy"). `tools/check_public_tree.py` in the Horizon
   repo already rejects them on commit; keep that hook on.
2. **No Mojang/Microsoft logos or branding** as our own. The name is used only to say what the software works with
   ("a Minecraft: Java Edition client"), with the disclaimer in [NOTICES.md](NOTICES.md).
3. **Third-party icons and art** (Modrinth mod icons, pack art, skins, capes) stay credited to their authors and linked
   to the source page. If an author asks, remove it the same day (the catalogue generator has one icon map:
   `tools/gen/icons.py`).
4. **Our own assets** (the Horizon banner, site design, launcher art) need a line in the table below so we can show we
   made them.
5. **Packs, shaders and mods uploaded by players** stay on the player's machine. The site hosts none of them.

## Inventory (keep this current)

| Asset | Where | Owner | Licence / basis | Notes |
|---|---|---|---|---|
| Rust, JS, docs | all repos | us / contributors | repository licence | |
| Catalogue mod icons (186) | `catalogue/`, `public/` | each mod's author | shown with credit and link to the Modrinth page | remove on request |
| Horizon banner, site CSS and layout | `docs/media`, `site/` | us | ours | |
| Fonts used by the site | `site/` | check each | check each font's licence file | list the exact fonts here |
| Hosted Play page (`horizon-offline*`) | `public/horizon-offline/package-*.bin` | **Mojang/Microsoft** (the game data inside) | **none we hold** | contains Minecraft assets: see the scan below |

## The hosted Play page: check this

The Play page is a large single-file or split build. Check what is inside it:

1. Unpack or search it for Minecraft's `assets/minecraft/` paths, `.ogg` sound hashes, `block/*.png` names,
   `lang/en_us.json` text, `default.png`/`ascii.png` fonts.
2. If **any** of that is present, it is a copy of Mojang's assets on our server. Options, safest first:
   - serve only the engine and make the page ask the player for their own game files (the `.hwp` import path in
     `docs/browser-building.md`), or
   - take the page down until it does.
3. "We own the code" does not cover this: the code can be ours and the bundle still infringes.
4. Record the result here and date it.

## Where each licence question goes

| Question | Where to look |
|---|---|
| May we port this mod? | `docs/MOD-MATRIX.md` in the patcher repo, and the upstream's licence; ARR/GPL/NC projects are implemented from public descriptions only |
| May we show this icon? | Modrinth page's licence and the author's wishes |
| May the build include this font/sound/texture? | Only if it is ours or its licence allows redistribution; otherwise no |

## Scan result: hosted Play page (2026-11-05)

Checked `public/horizon-offline/package-0..16.bin` (the split web package, 115 MB):

- It is a gzip of a `horizon-web-pack` file (`HWPK` header, 336 MB unpacked, 28,073 entries, game version 26.3).
  Its own manifest says `"private": true`: it was made for private use.
- **It contains Minecraft's assets and data**: 16,621 files under `assets/minecraft/` (4,869 `.ogg` sounds,
  3,965 `.png` textures, 144 language files, 16 font files, shaders and model JSON) and 9,883 under
  `data/minecraft/` (recipes, loot tables, tags and the like).
- It is **committed to the public `horizon-mods-lib` repository** (commit `6c2d4e9`, path `public/horizon-offline/`)
  as well as being served by the site, so it is in the git history and in every fork and clone.

This is redistribution of Mojang/Microsoft material, whoever owns the code around it. The code is ours; the package
is not. See "What to do" in the next section.

## What to do

In order, with the reasons:

1. **Take the package off the live site.** Remove `horizon-offline/` and the single-file builds from `public/` and
   make `play.html` ask the player for their own `.hwp` (the file picker path already exists in the Horizon web
   build). The Play page then ships code only.
2. **Remove it from the repository's history**, not only from the tip (`git filter-repo --path
   public/horizon-offline --invert-paths`, then a force push). A normal delete leaves it in every clone. A history
   rewrite changes every commit id after the point it was added, so everyone with a clone must re-clone.
3. **Check forks and mirrors** (GitHub shows forks; ask each to remove it) and any CDN or Pages copy
   (`horimods.pages.dev`) you control.
4. **Do not re-add it** under another name. `tools/check_public_tree.py` in the Horizon repo shows how to reject
   these paths on commit; copy its idea here.
5. Keep the log: when found, when removed, who was told (`templates/incident-log.md`).
