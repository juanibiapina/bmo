// {{title}}. A bmo app: what it draws is in render, what every key does is in
// KEYS. Replace ITEMS and the detail with the app's own content; keep the
// keyboard conventions (bmo guide lists them).

import process from "process";
import { View, div } from "gpui-kit";
import { h_flex, v_flex, v_virtual_list, VirtualListScrollHandle, Scrollbar } from "gpui-base";
import { theme } from "bmo";
import { keyName, footer, Help } from "./keys.js";

const ROW_HEIGHT = 30;

// Every key, once: the handler, the footer and the help popup all read this.
const KEYS = [
  { keys: ["j", "down"], label: "next", group: "Move", act: "next", footer: true },
  { keys: ["k", "up"], label: "previous", group: "Move", act: "previous" },
  { keys: ["g"], label: "first", group: "Move", act: "first" },
  { keys: ["G"], label: "last", group: "Move", act: "last" },
  { keys: ["ctrl-r"], label: "reload", group: "Other", act: "reload" },
  { keys: ["?"], label: "all keys", group: "Other", act: "help", footer: true },
  { keys: ["q"], label: "quit", group: "Other", act: "quit", footer: true },
];

const ITEMS = [
  { title: "Welcome", body: "This is a new bmo app. Its list is on the left and the selected item is here." },
  { title: "Keys", body: "j and k move. ? shows every key. q quits. Add a key by adding a row to KEYS." },
  { title: "Colors", body: "Every color comes from theme(), so the app follows light and dark." },
];

export default class App extends View {
  init(_props, cx) {
    this.handle = cx.focus_handle();
    this.handle.focus();
    this.help = new Help();
    this.scroll = VirtualListScrollHandle.new();
    this.items = ITEMS;
    this.at = 0;
    this.note = null;
  }

  move(cx, to) {
    this.at = Math.max(0, Math.min(this.items.length - 1, to));
    this.scroll.scroll_to_item(this.at, "center");
    cx.notify();
  }

  key(event, cx) {
    const name = keyName(event);
    if (this.help.key(name, event, cx)) return;
    const entry = KEYS.find((k) => k.keys.includes(name));
    if (!entry) return;
    cx.stop_propagation();
    this.note = null;
    switch (entry.act) {
      case "next": return this.move(cx, this.at + 1);
      case "previous": return this.move(cx, this.at - 1);
      case "first": return this.move(cx, 0);
      case "last": return this.move(cx, this.items.length - 1);
      case "reload": this.note = { text: "Nothing to reload yet" }; return cx.notify();
      case "help": return this.help.show(cx);
      case "quit": return process.exit(0);
    }
  }

  renderList(t) {
    const row = (item, index) => h_flex().id(`item-${index}`).h(ROW_HEIGHT).px(10).rounded(6).items_center()
      .when(index === this.at, (el) => el.bg(t.selected))
      .hover((style) => style.bg(t.hover))
      .on_click((_event, cx) => this.move(cx, index))
      .child(div().truncate().text_color(index === this.at ? t.foreground : t.muted).child(item.title));
    return v_flex().relative().w(260).flex_none().h_full().p(6).rounded(10).bg(t.chrome)
      .child(v_virtual_list("items", this.items.length, ROW_HEIGHT,
        (index) => String(index),
        (range) => this.items.slice(range.start, range.end).map((item, offset) => row(item, range.start + offset)))
        .track_scroll(this.scroll).size_full())
      .child(Scrollbar.vertical("items").absolute().inset_0());
  }

  renderDetail(t) {
    const detail = v_flex().flex_1().min_w(0).h_full().rounded(10).bg(t.background).border_1().border_color(t.border);
    const item = this.items[this.at];
    if (!item) return detail.items_center().justify_center().text_color(t.dim).child("Nothing here");
    return detail.id("detail").p(24).gap(10).overflow_y_scrollbar()
      .child(div().text_size(18).font_semibold().text_color(t.foreground).child(item.title))
      .child(div().text_color(t.muted).child(item.body));
  }

  render() {
    const t = theme();
    return v_flex()
      .id("app")
      .track_focus(this.handle)
      .on_key_down((event, cx) => this.key(event, cx))
      .relative().size_full().p(12).gap(10)
      .bg(t.canvas).text_size(13).text_color(t.foreground)
      .child(h_flex().flex_1().min_h(0).gap(10).child(this.renderList(t)).child(this.renderDetail(t)))
      .child(footer(t, KEYS, this.note))
      .when(this.help.open, (el) => el.child(this.help.render(t, KEYS)));
  }
}
