# 目录结构 — yororen_ui_default_renderer

---

## 布局

```
crates/yororen-ui-default-renderer/
├── Cargo.toml
├── themes/
│   ├── system-light.json
│   └── system-dark.json
└── src/
    ├── lib.rs              # 再导出 renderers + themes helpers
    ├── themes.rs           # system_for / install / install_with
    └── renderers/
        ├── mod.rs
        ├── button.rs       # TokenButtonRenderer（参考实现）
        ├── registry.rs     # register_default_renderers / token_based
        └── ...             # 每个组件一个 Token* 实现
```

---

## 规则

1. **一个组件一个文件**，命名与 core headless 对齐。
2. Trait 定义在 **core**（`yororen_ui_core::renderer::*`）；本 crate 提供 `TokenXxxRenderer` 默认实现并 re-export。
3. 颜色读 `action.<variant>.*`；几何读 `tokens.control.<component>.*` 或共享 `tokens.radii.*` / `tokens.spacing.*`。
4. 安装入口：`install(cx, appearance)` 或 `install_with(cx, theme)`。
5. 依赖：仅 `yororen-ui-core` + `gpui-ce` + `serde_json`。

---

## 示例

- `crates/yororen-ui-default-renderer/src/renderers/button.rs`
- `crates/yororen-ui-default-renderer/src/themes.rs`
- 主题文件：`crates/yororen-ui-default-renderer/themes/system-light.json`
