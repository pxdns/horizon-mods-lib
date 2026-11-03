# Contributing

See [docs/contributing.md](docs/contributing.md). In short: write a port from the original's behaviour (never its code), keep `runtime.rs` pure and tested,
run `cargo fmt`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked` and `python3 tools/mods.py check <slug>`, add a release workflow, a
credits row and an icon.
