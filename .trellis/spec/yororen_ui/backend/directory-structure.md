# 目录结构 — yororen_ui

> Meta-crate：尽量薄，只做再导出与 feature 门面。

---

## 目录布局

```
crates/yororen-ui/
├── Cargo.toml          # features: default=["xml"], brutalism, xml
└── src/
    ├── lib.rs          # 再导出 core / renderer / locales / 可选 xml
    └── locale.rs       # 安装内置 locale + 叠加应用翻译的便利函数
```

工作区成员（相关）：

```
crates/
├── yororen-ui-core
├── yororen-ui-default-renderer
├── yororen-ui-brutalism-renderer   # optional feature "brutalism"
├── yororen-ui-xml                  # optional feature "xml"
├── yororen-ui-xml-macro
├── yororen-ui-locale-en
├── yororen-ui-locale-zh-CN
├── yororen-ui-locale-ar
└── yororen-ui-demos/...
```

---

## 模块职责

| 路径 | 职责 |
|------|------|
| `lib.rs` | `pub use` core 的 headless/a11y/animation/assets/i18n/notification/rtl/theme；再导出 default renderer 为 `renderer`；feature 门控 brutalism / xml |
| `locale` | 安装 bundled locales、允许 app 叠自己的翻译 |

---

## 命名约定

- **目录名**：kebab-case（`yororen-ui`）
- **Cargo package name**：snake_case（`yororen_ui`）
- 用户代码优先：`use yororen_ui::headless::button::button;` 而不是深依赖 `yororen_ui_core::...`（分层 demo 除外）

---

## Feature 矩阵

| Feature | 效果 |
|---------|------|
| `default` | 启用 `xml` |
| `xml` | 导出 `xml!` / `xml_file!` / `register_xml_component` / `yororen_ui_xml` |
| `brutalism` | 导出 `brutalism_renderer` |

未启用 `xml` 时，`xml!` 等宏仍存在但展开为 `compile_error!("enable features = [\"xml\"] ...")`，避免“找不到宏”的含糊错误。

---

## 示例

- 应用入口安装：`crates/yororen-ui-demos/counter/src/main.rs`
  - `renderer::install(cx, cx.window_appearance())`
  - `locale_en::install(cx)`
