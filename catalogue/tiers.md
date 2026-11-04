# Tiers

![Tiers icon](../assets/icons/catalogue/tiers.png)

**Status:** Needs a hook point - **Upstream licence:** `GNU General Public License v3.0 only` - **26.3 build:** `tiers-0.9.0+26.3` - [Modrinth](https://modrinth.com/mod/tiers)

## Where it stands

Needs the PvPTiers service's lookup interface, which could not be reached from the build environment (the site is blocked there) and is only described in the original's GPL-3.0 source, which was not read. A lookup also sends each player's name to that service, so it would be opt-in. Horizon has no hook in the name tag drawing; the display would be a command and a HUD line. Not ported until a sample response from the service is available.

A port needs data or a hook that Horizon's stable hook points do not give a mod. It can still be written as a raw-hook port pinned to one Horizon commit; that is tracked in the roadmap.

## Credit

Tiers belongs to Flavio6561; see its Modrinth page. No code or text from it is copied here. The icon above is the project's own icon from Modrinth, by Flavio6561, shown only to identify the mod; it is not covered by this repository's licence and comes down on request (see [DMCA and takedowns](../docs/dmca.md)).
Back to the [catalogue](../MODS.md).
