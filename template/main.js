import { View, div } from "gpui-kit";
import { v_flex } from "gpui-base";
import { theme } from "bmo";

export default class App extends View {
  render() {
    const t = theme();
    return v_flex()
      .size_full()
      .bg(t.background)
      .text_color(t.foreground)
      .child(div().p(16).child("{{title}}"));
  }
}
