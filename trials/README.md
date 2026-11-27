# Trials

A trial tests the whole experience: a fresh agent session gets a request in the user's words and builds the app alone. We then review the app and the session, change bmo, and run the trial again.

Each `<trial>.md` has the request, the answers the user would give if asked, and what a good result has. The agent never sees this folder: it works in an empty folder with its own bmo home.

```sh
trials/run hn
```

A trial that changes an app the user already has runs with `--here`, in their real bmo home; copy the app aside first to compare.

The run's bmo home, the agent's output and the session are kept under `~/.local/state/bmo-trials/<trial>-<time>/`.

## Rules

- No app code by hand. When the agent lacks something, that is a gap in bmo.
- The user answers only as a user would. An implementation hint is a finding against bmo.
- A trial passes when the user accepts the app, nobody gave a hint, and the agent's own check passed before it said done.
- One stumble is not a rule. A finding becomes guidance only when it would generalize.

## Review

- **The app:** use it, look at screenshots in dark and light, read its code, and go through the trial's list and each convention in `bmo guide` that applies.
- **The session:** time, cost and turns; errors and retries; when it read the guide; what it looked up elsewhere; whether it checked and looked at a screenshot before saying done.
- **The log:** add the run to the trial's file with its numbers, findings and the changes made to bmo.
