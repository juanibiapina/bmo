# AGENTS.md

bmo is a host where agents build desktop apps: one Rust program that is the command line and the runtime, built on the `gpui-shell` crate.

- `just test` runs the tests that need no desktop. `just test-desktop` also runs the ones that open a window.
- Write the test first. Tests run the built `bmo` binary against a temporary home (`tests/support`), never against `~/.config/bmo`. To test what an app can do, give it work that paints the window green and take a `bmo shot` (`Bmo::shows_green`).
- What apps can import is declared in `src/blocks.d.ts` and registered in `src/blocks.rs`; the two must name the same functions. `src/guide.md` is the manual agents read through `bmo guide`, and `template/` is what `bmo new` writes.
- Bound a `bmo run` you start by hand with `timeout`: it blocks until its window closes.
