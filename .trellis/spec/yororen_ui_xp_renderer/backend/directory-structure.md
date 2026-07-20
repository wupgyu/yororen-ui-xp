# 目录结构 — yororen_ui_xp_renderer

---

## 布局

```
crates/yororen-ui-xp-renderer/
├── Cargo.toml
├── themes/
│   └── xp-luna.json          # 唯一内置主题（XP 无暗色模式）
└── src/
    ├── lib.rs                # install / install_with_default_theme / install_with / register_xp_renderers + 测试
    ├── style.rs              # Luna 调色板、渐变/斜面/阴影 helper
    ├── window.rs             # XpAppWindow 全窗应用铬脚手架（Luna 标题条 + 三边蓝框 + caption 接线 + 拖拽区）
    └── renderers/
        ├── mod.rs
        ├── actions.rs        # 5:  Button, IconButton, ToggleButton, SplitButton, ButtonGroup
        ├── controls.rs       # 5:  Switch, Checkbox, Radio, RadioGroup, Slider
        ├── display.rs        # 14: Label … ProgressBar（分段绿块）… Spacer
        ├── inputs.rs         # 9:  TextInput … Select, ComboBox …
        ├── lists.rs          # 9:  ListItem, Listbox, Tree*, Form*, Table, VirtualList*
        ├── notifications.rs  # 2:  Toast, Notification
        ├── overlays.rs       # 6:  Modal（XP 标题条）, Popover, DropdownMenu, Disclosure, Overlay, Menu
        └── surfaces.rs       # 5:  Tooltip, Avatar, Panel, Card, Image
```

与 default_renderer「一文件一组件」不同，xp 按 **domain 分组**（同 brutalism）。
结构命名镜像 brutalism crate：`Brutal*` → `Xp*`，`BRUTAL_*` → `XP_*`。

---

## 安装

```rust
// 单主题（XP 无暗色；install 不区分系统外观）
xp_renderer::install(cx);
// 或自定义主题
xp_renderer::install_with(cx, theme);
// 仅注册渲染器（主题已由别处安装时，如测试）
xp_renderer::register_xp_renderers(cx);
```

全窗应用铬（Explorer 风格窗口）用脚手架，不要手写：

```rust
let options = XpAppWindow::window_options(cx, "My Computer", size(px(800.0), px(560.0)));
// app 实体里：let chrome = XpAppWindow::new(cx, "My Computer");
// render 里：self.chrome.render(Some(title_icon), body, cx)
```

Meta-crate 需 `features = ["xp"]` 才能 `use yororen_ui::xp_renderer`。

---

## 关键文件

- `crates/yororen-ui-xp-renderer/src/lib.rs`（55 条注册 + `registers_all_55_renderers` 测试）
- `crates/yororen-ui-xp-renderer/src/style.rs`（共享视觉 helper）
- `crates/yororen-ui-xp-renderer/src/window.rs`（`XpAppWindow` 全窗铬脚手架；demo 用法见 `xp_my_computer`）
- `crates/yororen-ui-demos/xp_showcase/src/xp_app.rs`（视觉验收 demo）
