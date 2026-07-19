# 目录结构 — yororen_ui_core

---

## 目录布局

```
crates/yororen-ui-core/
├── Cargo.toml
├── build.rs
├── assets/icons/*.svg          # rust-embed 图标
└── src/
    ├── lib.rs                  # 模块出口；允许 missing_docs（组件数量多）
    ├── assets.rs
    ├── rtl.rs
    ├── a11y/                   # click_outside, focus_trap, keyboard_nav, scroll_lock
    ├── animation/              # config, easing, orchestrator, visibility...
    ├── headless/               # 每个组件一个文件 + layout/
    │   ├── mod.rs
    │   ├── button.rs           # button() -> ButtonProps
    │   ├── layout/             # center/column/row/... 无 renderer trait
    │   └── ...
    ├── i18n/                   # Locale, I18n global, format, translate, macros
    ├── notification/           # NotificationCenter 状态机
    ├── renderer/               # markers, registry, *Renderer traits, Variant*
    └── theme/                  # Theme(JSON Value), ActiveTheme, install
```

---

## 模块组织规则

1. **新组件** = 同步新增：
   - `headless/<name>.rs`：`xxx()` 工厂 + `XxxProps` +（如需）state
   - `renderer/<name>.rs`：`XxxRenderer` trait + `XxxRenderState`
   - `renderer/markers.rs` 中的 marker 类型
   - default / brutalism renderer 的 impl
   -（若暴露给 XML）更新 schema 生成输入并 `gen-schema`
2. **layout 原语** 只放 `headless/layout/`，**没有** `XxxRenderer`。
3. **通知视觉 host** 不在 core 状态机里画 UI；状态在 core，视觉 toast 在 renderer。

---

## 命名约定

| 概念 | 约定 | 例 |
|------|------|----|
| 工厂函数 | snake_case | `button`, `label`, `column` |
| Props | `XxxProps` | `ButtonProps` |
| Renderer trait | `XxxRenderer` | `ButtonRenderer` |
| Marker | 与组件同名 unit/struct | `markers::Button` |
| 回调 | `Arc<dyn Fn(...) + Send + Sync>` | `ClickCallback` |

文件名：与组件 snake_case 一致（`text_input.rs`, `dropdown_menu.rs`）。

---

## 示例

- Headless 按钮：`crates/yororen-ui-core/src/headless/button.rs`
- Layout：`crates/yororen-ui-core/src/headless/layout/mod.rs`
- Theme：`crates/yororen-ui-core/src/theme/mod.rs`
- Registry：`crates/yororen-ui-core/src/renderer/registry.rs`
