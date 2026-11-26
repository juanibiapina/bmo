# Markdown viewer

## Request

> build me a markdown viewer app

## Answers, if asked

- Which file: the one I pass when I open it, or let me pick from the folder I'm in.
- Editing: no, reading only.

## A good result has

- The document has the window; no empty list beside it.
- Headings, lists, code blocks, links, quotes and tables read well.
- J / K and ctrl-d / ctrl-u (or j / k, as there is no list) scroll; g and G go to the ends.
- / searches in the document; ? shows every key; q quits.
- A way to choose a file with the keyboard, and the last file opens on the next start.
- ctrl-r reloads the file; a missing or unreadable file is said in words.
- Only theme colors; right in dark and light.

## Runs

### 1: 2026-11-27, bmo after the first Hacker News run

- **Numbers:** 5 minutes, $1.88, 36 turns, claude-opus-5-5. No question asked, no hint given.
- **Path:** skill, `bmo guide`, `bmo new markdown`, then built. Checked and looked at eight screenshots, light and dark, kept with `--as`.
- **Result:** a viewer for a folder of markdown files: file list on the left, the rendered file on the right, a filter, a key to open another folder, a key to hide the list, reload on change, the last file remembered. Not yet used by Juan, so not yet a pass.

Findings:

1. **Code blocks and table headers were filled with the link blue.** bmo mapped its accent color onto the runtime's accent, which the markdown renderer uses as a quiet fill. The agent read bmo's own source to find out, and overrode the color in the app.
2. **No way to scroll a pane from code,** again. The document moves a block at a time, and the app cuts long code blocks, tables and lists into pieces of 18 lines so each fits the window. A cut code block shows a seam.
3. **It kept the list beside the document.** The request can be read as a viewer for a folder, and the app has a key to hide the list, but the starting app's shape may have decided this. No change until another run shows the same pull.

Changes made: the runtime's accent is now a quiet surface. Finding 2 is the same open gap as in the Hacker News run.
