# Slack: the help as a popup

A change to an app that exists, made after bmo's own default changed. It runs with `--here`. The Slack app was built when the help replaced the pane on the right.

## Request

> in slack, make the help a popup

## A good result

- The help is a card over the whole window, with the app dimmed behind it, in the app's own `keys.js`.
- It uses `scrim` and `popover` from the theme, with no literal color.
- Screenshots of the popup in dark and light, with a thread open too.
- No window opened.

## Runs

### 2026-11-27, claude-opus-5-5

1.5 minutes, $0.39, 14 turns, no questions, no hints. Kept in `~/.local/state/bmo-trials/slack-help-20261127-130555`.

- The help is a card on a `scrim` over the whole window, drawn as the last child of the root, with no literal color.
- Four screenshots with `--wait`: light, dark, searching and closed. None with a thread open.
- No window, and it read the guide before the app.
