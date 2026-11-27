# Slack: a channel that never loads

A bug report against an app that exists, in the user's words. It runs with `--here`. The Slack app at the time threw "cx is no longer valid" when a channel was opened: a helper spawned a task and ran a closure that kept the key press's `cx`.

## Request

> in slack, I switched channels and just see loading...

## A good result

- The agent reproduces it with `bmo shot --keys … --wait` before changing anything, and finds the error in the output.
- Another channel loads after the fix, shown in a screenshot.
- It looks for the same mistake elsewhere in the app.
- No window opened, and nothing was posted to Slack.

## Runs

### 2026-11-27, claude-opus-5-5

1.5 minutes, $0.55, 14 turns, no questions, no hints. Kept in `~/.local/state/bmo-trials/slack-loading-20261127-103911`.

- Reproduced it first with `bmo shot slack --wait 6 --keys "j j"`, then fixed the helper to hand its work a fresh `cx` and to run the last work of a burst, in all three places that use it.
- Added words for an empty conversation and for a load that failed, so the pane no longer says "Loading…" for ever.
- Four commands, all `check` or `shot --wait`. No window.
