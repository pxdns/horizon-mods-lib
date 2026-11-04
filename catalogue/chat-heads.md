# Chat Heads

![Chat Heads icon](../assets/icons/catalogue/chat-heads.png)

**Status:** Needs a hook point - **Upstream licence:** `Mozilla Public License 2.0` - **26.3 build:** `chat_heads-1.2.9-fabric-26.3` - [Modrinth](https://modrinth.com/mod/chat-heads)

## Where it stands

Needs the sender's skin face drawn beside each chat line. Horizon draws heads for the player list through its skin slots (`game/scoreboard.rs`), but the chat layout is in the render crate with no way to ask for them; it would take a rebuilt layout (as Chat Animation has) plus a lookup of the sender's slot. Not attempted.

A port needs data or a hook that Horizon's stable hook points do not give a mod. It can still be written as a raw-hook port pinned to one Horizon commit; that is tracked in the roadmap.

## Credit

Chat Heads belongs to Fourmisain, dzwdz; see its Modrinth page. No code or text from it is copied here. The icon above is the project's own icon from Modrinth, by Fourmisain, dzwdz, shown only to identify the mod; it is not covered by this repository's licence and comes down on request (see [DMCA and takedowns](../docs/dmca.md)).
Back to the [catalogue](../MODS.md).
