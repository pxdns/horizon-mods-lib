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
| Hosted Play page (`horizon-offline*`) | `public/` | **to check** | **see below** | |

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
