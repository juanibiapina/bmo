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
