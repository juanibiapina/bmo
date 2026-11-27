// Keyboard plumbing every bmo app starts with: key names, key chips, the
// footer and the help popup. The app owns a KEYS table (see main.js) and this
// file draws the footer and the help popup from it, so a key is described once.
// It is the app's own code: change it when the app needs something else.

import { div } from "gpui-kit";
import { h_flex, v_flex } from "gpui-base";

const NAMED = ["down", "up", "left", "right", "enter", "escape", "tab", "space", "backspace"];
const SHOWN = { down: "↓", up: "↑", left: "←", right: "→", enter: "↵", escape: "Esc", tab: "Tab", space: "Space" };

// The name a key press is matched by: "j", "J", "?", "enter", "ctrl-d", "cmd-k".
export function keyName(event) {
  const modifiers = event.modifiers ?? {};
  if (modifiers.control) return `ctrl-${event.key}`;
  if (modifiers.platform) return `cmd-${event.key}`;
  if (NAMED.includes(event.key)) return modifiers.shift ? `shift-${event.key}` : event.key;
  return event.key_char || event.key;
}

// How a key is shown to the person: ctrl-d reads ⌃D, enter reads ↵.
export function keyLabel(key) {
  if (key.startsWith("ctrl-")) return `⌃${key.slice(5).toUpperCase()}`;
  if (key.startsWith("cmd-")) return `⌘${key.slice(4).toUpperCase()}`;
  if (key.startsWith("shift-")) return `⇧${SHOWN[key.slice(6)] ?? key.slice(6)}`;
  return SHOWN[key] ?? key;
}

export function keyChip(t, key) {
  return h_flex().flex_none().h(18).min_w(18).px(5).rounded(4).items_center().justify_center()
    .bg(t.raised).border_1().border_color(t.border)
    .text_size(11).text_color(t.foreground).child(keyLabel(key));
}

// The line at the bottom: the keys marked footer, then a note on the right.
// note is { text, level } with level "error", "success" or nothing.
export function footer(t, keys, note) {
  const color = { error: t.ansi1, success: t.ansi2 }[note?.level] ?? t.muted;
  return h_flex().flex_none().gap(12).items_center()
    .children(keys.filter((k) => k.footer).map((k) => h_flex().gap(5).items_center()
      .child(keyChip(t, k.keys[0]))
      .child(div().text_size(11.5).text_color(t.muted).child(k.label))))
    .child(div().flex_1())
    .when(Boolean(note), (el) => el.child(div().max_w(520).truncate().text_size(11.5).text_color(color).child(note.text)));
}

// One line of text typed while the app's own focus handle keeps the
// keyboard: a search or filter box. The app passes it each key press first;
// it takes letters and backspace and leaves the rest (↵, Esc, arrows) to the
// app. It suits short text. A message or a note needs an Input from gpui-base.
export class TextBox {
  constructor(placeholder) {
    this.placeholder = placeholder;
    this.text = "";
  }

  // True when the key was text and changed the box.
  key(name, event) {
    const modifiers = event.modifiers ?? {};
    if (modifiers.control || modifiers.platform) return false;
    if (name === "backspace") this.text = this.text.slice(0, -1);
    else if (name === "space") this.text += " ";
    else if (event.key_char && event.key_char.length === 1) this.text += event.key_char;
    else return false;
    return true;
  }

  words() {
    return this.text.toLowerCase().split(/\s+/).filter(Boolean);
  }

  // Whether every word typed appears somewhere in the text given.
  matches(text) {
    const within = text.toLowerCase();
    return this.words().every((word) => within.includes(word));
  }

  render(t, active) {
    return h_flex().h(32).flex_none().px(10).gap(1).rounded(8).items_center()
      .border_1().border_color(active ? t.accent : t.border).bg(t.background).text_size(12.5)
      .child(div().text_color(this.text ? t.foreground : t.dim).child(this.text || (active ? "" : this.placeholder)))
      .when(active, (el) => el.child(div().w(1.5).h(15).bg(t.accent)))
      .when(active && !this.text, (el) => el.child(div().pl(4).text_color(t.dim).child(this.placeholder)));
  }
}

// The help popup: every key in the table, grouped, with a search box.
// While it shows, / opens the box, ↵ keeps the matches, Esc clears the search
// and then closes the sheet, and any other key closes it.
export class Help {
  constructor() {
    this.open = false;
    this.searching = false;
    this.search = new TextBox("Search keys…");
  }

  show(cx) {
    this.open = true;
    this.searching = false;
    this.search.text = "";
    cx.notify();
  }

  // Call first in the app's key handler. True when the sheet took the key.
  key(name, event, cx) {
    if (!this.open) return false;
    cx.stop_propagation();
    if (this.searching && name === "enter") this.searching = false;
    else if (this.searching && this.search.key(name, event)) {}
    else if (name === "escape" && (this.searching || this.search.text)) {
      this.searching = false;
      this.search.text = "";
    } else if (!this.searching && name === "/") this.searching = true;
    else if (!this.searching) this.open = false;
    cx.notify();
    return true;
  }

  // The groups shared out over up to three columns of about the same height,
  // so a long key table still fits the pane.
  columns(shown) {
    const groups = [...new Set(shown.map((k) => k.group))].map((name) => ({ name, keys: shown.filter((k) => k.group === name) }));
    const rows = (group) => group.keys.length + 2;
    const total = groups.reduce((sum, group) => sum + rows(group), 0);
    const count = Math.min(3, Math.max(1, Math.ceil(total / 16)));
    const columns = [[]];
    let filled = 0;
    for (const group of groups) {
      if (filled >= total / count && columns.length < count) {
        columns.push([]);
        filled = 0;
      }
      columns[columns.length - 1].push(group);
      filled += rows(group);
    }
    return columns;
  }

  // A popup over the whole app: add it as the last child of the root element,
  // which must be .relative(), while the help is open. about is one line
  // under the keys saying what the app changes, if anything.
  render(t, keys, about) {
    const shown = keys.filter((k) => this.search.matches([...k.keys, k.label, k.group].join(" ")));
    const columns = this.columns(shown);
    const card = v_flex().w(columns.length * 280 + 40).max_w_full().max_h_full().overflow_hidden()
      .gap(14).p(20).rounded(12).bg(t.popover).border_1().border_color(t.border).shadow_lg()
      .child(div().text_size(14).font_semibold().text_color(t.foreground).child("Keys"))
      .when(this.searching || Boolean(this.search.text), (el) => el.child(this.search.render(t, this.searching)))
      .when(Boolean(this.search.text) && !shown.length, (el) => el.child(div().text_size(12).text_color(t.dim).child(`No key matches “${this.search.text}”`)))
      .child(h_flex().gap(28).items_start().children(columns.map((groups) => v_flex().flex_1().min_w(0).gap(14)
        .children(groups.map((group) => v_flex().gap(4)
          .child(div().text_size(10.5).font_semibold().text_color(t.dim).child(group.name.toUpperCase()))
          .children(group.keys.map((k) => h_flex().gap(8).items_center()
            .child(h_flex().gap(3).w(60).flex_none().children(k.keys.map((key) => keyChip(t, key))))
            .child(div().truncate().text_size(12).text_color(t.muted).child(k.label)))))))))
      .child(div().text_size(11.5).text_color(t.dim).child(`${about ? `${about} ` : ""}/ searches the keys; any other key closes this.`));
    return v_flex().absolute().inset_0().p(24).items_center().justify_center().bg(t.scrim).child(card);
  }
}
