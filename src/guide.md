# Building a desktop app with bmo

bmo runs a folder of JavaScript as a native macOS app. An app draws its window
with layout elements, reaches the outside through the `bmo` module, and is
driven by the keyboard. Apps are trusted: files, programs and the network work
without grants.

## The loop

Apps live in `{{apps}}/<name>/`. Work only inside the app's folder.

1. `bmo new <name>` creates the app from a working starting point. Names are
   lowercase letters, digits and dashes.
2. Edit `main.js`. Read the starting `main.js` and `keys.js` first: they show
   the conventions below working.
3. `bmo check <name>` after every change. It loads the app and draws it once,
   with no window, and reports syntax, import and first-draw errors. It does
   not run code after an `await`.
4. `bmo shot <name>` draws a fresh hidden copy of the app, waits half a
   second, and prints what the app logged and then the path of a PNG. It
   fails when the app logged an ERROR, such as a rejection nobody caught;
   read its whole output, not the last lines. Open the
   PNG and look at it. `--keys "j j ?"` presses keys first, so a screen that
   needs a key press can be captured. `--wait 5` waits five seconds instead,
   before the keys and again after them, for work such as a network load or
   a live connection. `--as help` saves it as
   `<name>-help.png`, to keep several. `--dark` and `--light` pick the
   appearance.
5. The app is done when the check passes, every `bmo shot` succeeds, and
   you have looked at a screenshot of the first screen and of every screen
   you added, the help sheet included. Press every key that starts work
   after an `await` (a load, a save, a move to another item) in a shot with
   a `--wait` long enough for the work to end: that code runs nowhere else. Check one screenshot in
   `--light` too.

`console.log` appears in the output of `bmo shot`: use it to see what code
after an `await` did. `bmo install <name>` puts the app in ~/Applications
under the id `bmo.<name>`; it starts from the source folder, so later edits
need no reinstall. Install when the person asks.

`bmo shot` is the only way you look at the app. `bmo run` and `bmo dev` open
a real window on the person's screen: run them only when the person asks to
see the app. `bmo dev <name>` reloads on save and `bmo stop <name>` closes
it. `bmo list` shows the apps.

## An app

- `main.js` default-exports a class that extends `View`.
- `keys.js` is the app's own keyboard plumbing: key names, the footer, the
  help sheet and a one-line text box. Change it when the app needs to.
- `gpui-shell.json` holds the `name` the person sees and an optional
  `"window": { "width": 1000, "height": 700 }`. Leave `id` alone. bmo
  ignores `capabilities` there: nothing needs a grant.
- `bmo.json` holds what the app asks of bmo. A new app has
  `{ "title_bar": false }`: a window with no title bar and no window
  buttons, so `q` is how it closes. Set it to `true` when the person wants
  the bar back.
- `gpui-kit.d.ts` is generated on every check: the exact API of every element
  and module. Search it before using something this guide does not show.

## Drawing

Chained builders with GPUI's snake_case names: `div()`, `h_flex()`,
`v_flex()`; `.size_full()`, `.flex_1()`, `.min_h(0)`, `.gap(n)`, `.p(n)`,
`.w(n)`, `.h(n)`, `.items_center()`, `.justify_between()`; `.child("text")`,
`.children([...])`, `.when(condition, (el) => el.…)`; `.text_size(n)`,
`.font_semibold()`, `.truncate()`, `.text_color(c)`; `.bg(c)`, `.border_1()`,
`.border_color(c)`, `.rounded(n)`; `.id("unique")` then `.on_click(...)` and
`.hover((style) => style.bg(c))`; `.overflow_y_scrollbar()` on an element
with an id. A long list is a `v_virtual_list`, as in the starting app.
`TextView.markdown(id, text)` from `gpui-base` draws markdown, and
`image(path)` from `gpui-kit` draws a picture from any absolute path.

- Every color comes from `theme()` in the `bmo` module, read inside `render`.
  A literal color breaks light or dark.
- Assign state in `init(_props, cx)`, not as class fields: a class field runs
  after `init` and overwrites it.
- After changing state in a handler or a task, call `cx.notify()`.
- A `cx` is valid only for the call that got it. For work with an `await`,
  use `cx.spawn(async (cx) => { … })` and the `cx` it hands you. A helper
  that spawns must pass that new `cx` on to the work it runs: a closure
  that kept the caller's `cx` throws "cx is no longer valid".
- Build elements inside `render`; never keep one on `this`.
- Controls from `gpui-base` (`Button`, `Input`, `Checkbox`) draw nothing by
  themselves: give them padding, colors and corners.
- `cx.open_url(url)` opens a link, `cx.write_to_clipboard(text)` copies, and
  `process.exit(0)` from `"process"` quits.

## The `bmo` module

```ts
{{blocks}}
```

Also there: `localStorage` (kept per app), `fs/promises` for any file and
`os.homedir()`. Keep downloaded pictures and other caches under
`process.dataDir`, the app's own data folder, and leave the app's folder
for its code.

The typings in `gpui-kit.d.ts` come from the runtime, and some of what they
say is not true under bmo. Where they speak of `capabilities`, grants or a
file having to be inside the app's folder, ignore it. Four things they list
stay closed, each with a `bmo` call that replaces it:

| Closed | Use |
|---|---|
| the global `fetch` | `fetch` and `download` from `bmo` |
| `process.run` | `exec` from `bmo` |
| the `net` module | `connect`, `read`, `write`, `close` from `bmo` |
| the `websocket` module | nothing yet: run a small local program that holds the socket and reach it with `connect` |

## Conventions

Every app is keyboard first. Follow these unless the person asks otherwise,
each where it fits: a document viewer has no list, but it still scrolls,
searches and shows its keys the same way.

| What | Keys |
|---|---|
| Move in a list | j / k, with ↓ / ↑ as aliases; g and G for first and last |
| Scroll what is being read | J / K a line, ctrl-d / ctrl-u half a page |
| Between panes and views | h / l between panes, Tab between views |
| Search or filter | / opens a box; ↵ keeps the result, Esc clears and leaves |
| Help | ? opens a searchable sheet of every key |
| Leave | q or Esc goes back one step; at the top q quits and Esc does nothing |
| Open and copy | o or ↵ opens the item, Y copies its link |
| Write | i starts writing, ↵ sends, shift-↵ adds a line |
| Change things | c creates, r replies or renames, d or e dismisses, z undoes, ctrl-r reloads |
| Send to an agent | p to the focused session, P picks one, n starts a new one, s shows it |

- One `KEYS` table says what every key does. The handler, the footer and the
  help sheet read it, so a key is described once.
- The footer shows the main keys of the current screen and, on the right, a
  short note when something happened or failed.
- Clicks work, but nothing needs the pointer.
- A change shows at once. If it fails, put the old state back and say so in
  the footer.
- The window draws from what it stored last time, then refreshes.
- An app that browses a collection has the list on the left and the selected
  item on the right. An app built around one document gives it the window.
- Show loading, empty and failed states in words, in the middle of the pane.

## Limits to work around

- The keyboard cannot be moved onto an `Input` from code. For a search or
  filter box, use `TextBox` from `keys.js`, which reads key presses while
  the app keeps the keyboard.
- Only a `v_virtual_list` can be scrolled from code, with a
  `VirtualListScrollHandle`'s `scroll_to_item`, as the starting app does for
  its list. It needs the height of every row up front. A `list` (rows of
  unknown height) and a plain scrolling pane move only by the wheel: a
  `list` ignores `track_scroll`, though `.with_follow_mode("tail")` keeps it
  at its end. To read long content by keys, draw it as a `v_virtual_list` of
  blocks (paragraphs, comments, messages) with a height worked out for each,
  and keep the index of the block at the top.
- Tab and shift-Tab never reach `on_key_down`: the window uses them to move
  focus first. Bind them as actions and send them to the same handler:
  `cx.bind_keys([{ keystroke: "tab", action: "app::Next", context: "App" }])`
  in `init`, then `.key_context("App").on_action("app::Next", (_event, cx) =>
  this.key({ key: "tab", modifiers: {} }, cx))` on the root element.
- A length given as a number is pixels, but `line_height` takes `"19px"`.
