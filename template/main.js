import { View, div } from "gpui-kit";
import { v_flex } from "gpui-base";
import { TitleBar } from "gpui-component";

export default class App extends View {
  render(cx) {
    const colors = cx.theme().colors;
    return v_flex()
      .size_full()
      .bg(colors.background)
      .child(TitleBar.new().child(div().text_size(13).text_color(colors.foreground).child("{{title}}")));
  }
}
