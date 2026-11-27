# Slack: live messages

A change to an app that exists. It runs against the user's real bmo home (`trials/run slack-live --here`), where the Slack app from the first Slack session lives.

## Request

> slack should show new messages the moment they arrive, through my slack-events bridge instead of checking every few seconds. also keep its downloaded pictures in its data folder, not in the app folder

## Answers

- The bridge is `slack-events`, on 127.0.0.1:47613. `slack-events --daemon` starts it.
- Keep the check every few seconds for when the bridge is down.

## A good result

- New messages, reactions and read marks arrive through `connect` and `read` from `bmo`, with no grant in the manifest.
- The app starts the bridge when nothing answers, and falls back to checking while it is down.
- Pictures are under `process.dataDir`; the app folder has only code.
- The agent read the app before changing it and kept its conventions.
- No window opened: the agent used `bmo shot --wait`, never `bmo run` or `bmo dev`.
- It did not read bmo's source.
- Nothing was posted to Slack.

## Runs

### 2026-11-27, claude-opus-5-5

6 minutes, $1.38, 33 turns, no questions, no hints. Kept in `~/.local/state/bmo-trials/slack-live-20261127-102810`.

- **Live events:** uses `connect` and `read` from `bmo`, starts the bridge when nothing answers and reconnects with a growing pause. A `channel_marked` event reached the app within a second. The manifest has no grant.
- **Pictures:** moved to `process.dataDir`; the app folder has only code.
- **No window:** 16 `bmo shot --wait`, no `bmo run` or `bmo dev`. It read nothing from bmo's source.
- **Missed:** it removed the check every few seconds instead of keeping it for when the bridge is down, and did not ask. While the bridge is down the app shows nothing new until it reconnects, then catches up once.
- **Not tested:** a new message, an edit and a reaction arriving. None came during the run, and it posted nothing.
- **Side effect:** it marked a channel read, at the place it was already read, to make an event.
- **Finding:** two of the six minutes went to pictures that drew blank in screenshots. A hidden window drew one frame, at the end. Fixed: `bmo shot` now draws frames while it waits.
