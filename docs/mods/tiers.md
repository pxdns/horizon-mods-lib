# Tiers

![Tiers icon](../../assets/icons/catalogue/tiers.png)

Look up a player's tiers on a tier website with /tier. The website interface is a guess; lookups are off until you switch them on.

**Version** 1.0.0 · **Game** 26.3 · **Pinned to Horizon ba6344a, reads game state** · **Licence** MIT · [Source](https://github.com/pxdns/horizon-mods-lib/tree/main/mods/tiers) · [Releases](https://github.com/pxdns/horizon-mods-lib/releases?q=tiers)

## Use it

| Command | What it does |
|---|---|
| `/tier <name>` | Asks the tier website about a player and prints their tiers per gamemode |
| `/tier me` | The same for you |
| `/tier url <template>` | Sets the address to ask; it must be `https://` and contain `{name}` |

**Lookups are off until you switch them on** (Options > Mods > Tiers): each one sends the player's name, and your address, to the website. **The website's interface in this mod is a guess** (`https://pvptiers.com/api/profile/{name}`), because it could not be reached or checked while the mod was written; if it is wrong, `/tier url` takes the right one, and the reply is read loosely (see below).

| Option | Default |
|---|---|
| Allow lookups (sends names to the website) | off |

Switch it on or off in **Options → Mods → Tiers**; its settings page has one button per option. Install steps are in
[Install a mod](../install.md).

## How it is built

- Hooks: `native_command`, `native_tick`, `native_mods`, `native_settings`, `native_settings_click`.
- Logic and tests: `mods/tiers/mixins/tiers/runtime.rs`.
- Pinned: Horizon `ba6344a`, horizon-mixins `d9c83d7`. It reads fields of the game that are not part of the stable hook-point contract, so it must be re-checked when Horizon changes them.

## Credit

Ported from **Tiers (tiers)** (independent implementation; no upstream code or assets (the original is GPL-3.0)); an independent implementation, with no upstream code or art. See [Credits](../credits.md)
and [Licensing](../licensing.md).
