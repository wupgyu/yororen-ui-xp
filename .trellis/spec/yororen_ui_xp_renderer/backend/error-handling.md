# 错误处理 — xp_renderer

> Renderer / theme 安装路径的错误策略（与 brutalism 一致）。

---

## 概览

- 内置 theme JSON 解析失败视为**编程错误**：`Theme::from_json(XP_LUNA).expect("xp-luna.json is valid")`。
- 自定义 theme：调用方使用 `Theme::from_json` 的 `Result`。
- theme path 缺失：读 `Option`，几何/颜色用 `unwrap_or` / fallback fn（`xp_color` / `xp_number` 已内置该约定）。
- 注册缺失会在 `XxxProps::render` 时 panic——由 `registers_all_55_renderers` 测试在 CI 兜住。

---

## 模式

```rust
// 安装（XP 单主题，不区分系统外观）
pub fn install(cx: &mut App) {
    install_with_default_theme(cx);
}

pub fn install_with(cx: &mut App, theme: Theme) {
    install_theme(cx, theme);
    register_xp_renderers(cx);
}
```

```rust
// xp.* 路径回退到 style.rs 常量
xp_color(theme, "xp.button.border", button_border())
xp_number(theme, "xp.progress.segment_width", XP_PROGRESS_SEGMENT_W as f64)
```

---

## 常见错误

- 只 `install_theme` 却忘记 `register_xp_renderers`。
- 在 renderer 里 `unwrap` 缺失 path 导致主题缺字段就崩溃。
- 新增 renderer 实现后忘记在 `register_xp_renderers` 与 `registers_all_55_renderers` 测试里同步登记。
