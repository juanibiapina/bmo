# Hacker News reader

## Request

> build me a Hacker News reader app

## Answers, if asked

- Which stories: the front page.
- Comments: yes, I want to read them in the app.
- Account, voting, posting: no.

## A good result has

- The front page as a list on the left: title, points, comment count, site, age.
- The selected story on the right, with its comments readable and threaded.
- j / k move through stories without a pause; the list keeps the selection in view.
- o or ↵ opens the story's link in the browser; a key opens the discussion on Hacker News; Y copies the link.
- J / K and ctrl-d / ctrl-u scroll the comments.
- ctrl-r reloads; / filters stories; ? shows every key; q quits.
- Loading, empty and failed states said in words.
- The last front page shows at once on the next start, then refreshes.
- Only theme colors; right in dark and light.
- No send-to-agent keys unless asked, or p / P / n that work.

## Runs

### 1: 2026-11-27, bmo at the first guide and starting app

- **Numbers:** 7 minutes, $1.85, 40 turns, claude-opus-5-5. No question asked, no hint given.
- **Path:** read the skill, ran `bmo guide`, `bmo new hn`, read the starting app, then built. Checked and looked at nine screenshots before saying done, light and dark.
- **Result:** five feeds as tabs, stories on the left, threaded comments on the right, fold and jump keys, a filter, stored lists that draw at once, read stories dimmed. Only theme colors. Not yet used by Juan, so not yet a pass.

Findings:

1. **No way to scroll a pane from code.** The agent searched the types, then the Cargo registry and the workspace, for a scroll handle (6 turns), and ended up moving a selection between comments. A comment taller than the pane needs the mouse wheel. The same gap blocks J / K in any reading view.
2. **The help sheet overflowed** with 25 keys: the last group was cut off. The agent rewrote the sheet's layout in `keys.js` (6 turns).
3. **`bmo shot` writes one file per app.** The agent copied the file after each shot to keep nine screens, and tried `bmo shot --help` and `bmo help shot` to find an option.
4. `line_height(19)` is not accepted; it takes `"19px"`. One stumble, no change.

Changes made: the help sheet lays its groups out in columns; `bmo shot --as <label>` names the file; any command answers `--help`; the guide says how to scroll long content by keys. Finding 1 needs a change to gpui-shell itself and is open.
