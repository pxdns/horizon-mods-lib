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
