# Legal support

Working documents for running Horizon and its mods library without stepping on anyone's copyright, and for
handling a takedown notice (DMCA or otherwise) if one arrives.

> **This is a working draft, not legal advice.** Nobody here is a lawyer. Have a lawyer (ideally one who knows
> open source and games) read `DMCA-POLICY.md`, the templates and `ASSETS.md` before you rely on them. The
> statute references are the ones the author knows; check them against the current text.

## The position these documents assume

- **We own the code.** Every line of Horizon's and the mods' Rust, JS and docs is written by us (or by contributors
  who licensed it to us), under the repository's licence. Nothing is copied from Mojang's code or from another mod
  whose licence forbids it (`tools/gen` records the licence of each upstream a mod was ported from).
- **We do not own the assets.** Textures, sounds, models, language files, fonts and the Minecraft name and logo
  belong to Mojang/Microsoft or to other artists. Mod icons on the catalogue pages belong to their authors.
- So the exposure is **assets**, not code. The safe rule is that the project never hosts or ships Mojang's assets,
  and credits and links what it shows from others.

## Files

| File | What it is |
|---|---|
| [ASSETS.md](ASSETS.md) | What may and may not be in the repos and on the sites; the inventory to keep up to date |
| [CHECKLIST.md](CHECKLIST.md) | Run before publishing a release or a site build |
| [DMCA-POLICY.md](DMCA-POLICY.md) | The notice-and-takedown policy to publish on the site |
| [NOTICES.md](NOTICES.md) | Disclaimer text ("not an official Minecraft product") for READMEs, sites and the game |
| [templates/takedown-triage.md](templates/takedown-triage.md) | First steps when a notice arrives |
| [templates/response-to-notice.md](templates/response-to-notice.md) | Reply to a complainant |
| [templates/counter-notice-handling.md](templates/counter-notice-handling.md) | If a user disputes a takedown |
| [templates/incident-log.md](templates/incident-log.md) | One log entry per notice |

## The one thing to fix first

If any public page serves a build that **contains Mojang's assets** (for example a hosted single-file game with the
textures and sounds inside), that is redistribution, whoever owns the code around it. `ASSETS.md` lists how to
check for it and the options: serve only the code and make the player load their own copy, or take the page down.
