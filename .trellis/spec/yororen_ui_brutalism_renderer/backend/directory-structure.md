# 目录结构 — yororen_ui_brutalism_renderer

---

## 布局

```
crates/yororen-ui-brutalism-renderer/
├── Cargo.toml
├── themes/
│   ├── brutalism-light.json
│   └── brutalism-dark.json
└── src/
    ├── lib.rs                 # install / install_with / register_brutal_renderers
    ├── style.rs               # 共享 brutal 样式 helper
    └── renderers/
        ├── mod.rs
        ├── actions.rs         # button 等
        ├── controls.rs
        ├── display.rs
        ├── inputs.rs
        ├── lists.rs
        ├── notifications.rs
        ├── overlays.rs
        └── surfaces.rs
```

与 default_renderer「一文件一组件」不同，brutalism 按 **domain 分组** 收敛实现。

---

## 安装

```rust
// 跟随系统外观
brutalism_renderer::install(cx);
// 或强制 light / 自定义
brutalism_renderer::install_with_default_theme(cx);
brutalism_renderer::install_with(cx, theme);
```

Meta-crate 需 `features = ["brutalism"]` 才能 `use yororen_ui::brutalism_renderer`。

---

## 示例

- `crates/yororen-ui-brutalism-renderer/src/lib.rs`
- `crates/yororen-ui-brutalism-renderer/src/style.rs`
- gallery 中 `RendererKind` 切换（`gallery_demo` / `gallery_xml`）
