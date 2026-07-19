# 错误处理

> Renderer / theme 安装路径的错误策略。

---

## 概览

- 内置 theme JSON 解析失败视为**编程错误**：`Theme::from_json(...).expect("... is valid")`。
- 自定义 theme：调用方使用 `Theme::from_json` 的 `Result`。
- theme path 缺失：读 `Option`，几何/颜色用 `unwrap_or` / `unwrap_or_default`（见 `TokenButtonRenderer`）。
- Registry 不完整：`validate() -> Result<(), Vec<&'static str>>`；token_based 构建路径会列出缺失 renderer。

---

## 模式

```rust
// 安装（default renderer）
pub fn install(cx: &mut App, appearance: WindowAppearance) {
    install_with(cx, system_for(appearance));
}

pub fn install_with(cx: &mut App, theme: Theme) {
    install_theme(cx, theme);
    register_default_renderers(cx);
}
```

```rust
// 颜色路径回退
let key = format!("action.{}.{}", state.variant.as_str(), field);
theme.get_color(&key).unwrap_or_default()
```

---

## 常见错误

- 只 `install_theme` 却忘记 `register_*_renderers`。
- 自定义 registry 覆盖部分 slot 后不 `validate()`。
- 在 renderer 里 `unwrap` 缺失 path 导致主题缺字段就崩溃。
