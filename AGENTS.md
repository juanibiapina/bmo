# AGENTS.md

bmo is a host where agents build desktop apps: one Rust program that is the command line and the runtime, built on the `gpui-shell` crate.

- `just test` runs the tests that need no desktop. `just test-desktop` also runs the ones that open a window, which need AeroSpace and a logged-in macOS session.
- Write the test first. Tests run the built `bmo` binary against a temporary home (`tests/support`), never against `~/.config/bmo`.
- A command that bmo does not define itself is passed to gpui-shell's host, so `check`, `run`, `list` and `types` come from there.
- Bound every command you run by hand with `timeout`: gpui-shell's `run` blocks until its window closes.
