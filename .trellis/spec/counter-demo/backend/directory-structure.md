# 目录结构 — counter-demo

## 目的

最小应用：Entity 状态、按钮 on_click、layout 原语、renderer/locale install。

## 布局

```
crates/yororen-ui-demos/counter/
├── Cargo.toml          # publish = false
└── src/
    ├── main.rs         # Application + install + open_window
    ├── state.rs        # CounterState global + Entity
    └── counter_app.rs  # Render impl
```


## 标准启动顺序（所有 demo 共用骨架）

```rust
let app = Application::new().with_assets(UiAsset);
app.run(|cx: &mut App| {
    renderer::install(cx, cx.window_appearance());
    locale_en::install(cx);
    // set_global state...
    cx.open_window(options, |_, cx| cx.new(|_cx| AppView));
});
```

## UI 写法

- 使用 `headless::layout::{center, column, row}` + `Spacing` / `Inset`。
- 交互：`button(id, cx).on_click(...).render(cx).child(...)`。
- 状态更新：`entity.update(cx, |s, cx| { ...; cx.notify(); })`。


## 真实示例文件

- `crates/yororen-ui-demos/counter/src/main.rs`
- `crates/yororen-ui-demos/counter/src/counter_app.rs`
- `crates/yororen-ui-demos/counter/src/state.rs`
