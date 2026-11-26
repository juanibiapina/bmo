# bmo

A host where coding agents build native desktop apps. An agent turns a request into an app that runs in its own window, under its own macOS id.

bmo is one program: the command line an agent drives and the runtime the apps run on. It is built on [gpui-shell](https://gpui-kit.com/shell/).

## Install

```sh
cargo install --path .
```

## Use

Apps live in `~/.config/bmo/apps/<name>/`.

| Command | What it does |
|---|---|
| `bmo new <name>` | Creates an app |
| `bmo check <name>` | Reports errors without opening a window |
| `bmo dev <name>` | Runs the app on the AeroSpace workspace `sandbox`, reloading on save |
| `bmo shot <name>` | Saves a screenshot of the running app and prints its path |
| `bmo stop <name>` | Stops the running app |
| `bmo install <name>` | Installs the app in `~/Applications` with the id `bmo.<name>` |
| `bmo list` | Lists the apps |

An installed app starts from its source folder, so a change shows the next time it starts.
