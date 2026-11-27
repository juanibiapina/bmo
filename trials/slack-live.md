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
