# bmo

A host where coding agents build native desktop apps. You ask an agent for an app in a sentence; it builds one that runs in its own window, under its own macOS id, driven by the keyboard.

bmo is one program: the command line the agent drives and the runtime the apps run on. It is built on [gpui-shell](https://gpui-kit.com/shell/).

- **Building blocks, not a framework.** Apps draw with layout elements and reach the outside through one `bmo` module: colors, programs, HTTP, files, and [Herdr](https://herdr.dev) agent sessions.
- **No permissions to manage.** Apps are trusted, as scripts on your own machine are.
- **A loop the agent can close alone.** `bmo check` reports errors and `bmo shot` saves a screenshot, both without a window on your desktop.
- **Conventions built in.** Every new app starts with vim-style keys, a footer of key hints and a searchable help sheet.

## Install

bmo builds against a fork of gpui-kit that must be checked out beside it, at `../gpui-kit` ([juanibiapina/gpui-kit](https://github.com/juanibiapina/gpui-kit)).

```sh
cargo install --path . --locked
```

Every bmo command writes its skill to `~/.agents/skills/bmo/SKILL.md`, where coding agents look for skills, so the skill always matches the installed bmo. It sends the agent to `bmo guide`, the manual for that version. `BMO_SKILLS` picks another skills folder.

## Use

Ask your agent: "build me a Hacker News reader app".

Apps live in `~/.config/bmo/apps/<name>/`. `bmo` with no arguments lists the commands. A running app reads its code when it starts. `bmo reload <name>` checks the source and tells the open window to load it again, in place; an app that is not running starts with the newest code.
