# 目录结构 — xp-showcase-demo

## 目的

XP 渲染器的视觉验收 demo（"一眼即 XP"门槛）：渐变按钮、绿色分段进度条、蓝勾 checkbox / 蓝点 radio、滑杆、白色凹陷输入框、徽章——全部跑在 `#ECE9D8` 米黄背景上。

## 布局

```
crates/yororen-ui-demos/xp_showcase/
├── Cargo.toml          # publish = false；yororen-ui features = ["xp"]
└── src/
    ├── main.rs         # xp_renderer::install(cx) + open_window（appears_transparent 隐藏系统标题栏）
    └── xp_app.rs       # 自绘 XP 标题条 + 单窗口组件展示（纯 headless 调用，零 XP 专属 API）
```

- 演示 `use yororen_ui::xp_renderer; xp_renderer::install(cx);`——装一次主题 + 55 个 renderer。
- **自绘 XP 窗口外框**（应用层窗口装饰，不在 55 个 renderer trait 内）：根节点是 1px `#0058E6` 边框 + 顶部 8px 圆角的外框（圆角像素透明透出桌面），body 距外框 3px 并带自己的 `#A09C8C` 内边框（顶边开放，对齐 css `.xp-window-body`）；不画窗口外阴影。
- **自绘 XP 标题条**：`TitlebarOptions { appears_transparent: true, .. }` 隐藏系统标题栏；标题条 div 用 `.window_control_area(WindowControlArea::Drag)`，最小化 / 最大化 / 关闭按钮分别用 `Min / Max / Close`——Windows 经 WM_NCHITTEST 原生执行拖拽与按钮动作，无需 on_click。标题条配色与 demo 内联常量对齐 css 5-stop 渐变（4 band 叠带）与 caption 按钮渐变。
- 根节点背景读 `surface.base`（米黄），窗口内容全部走 headless 工厂函数。
- 验收截图存 `screenshots/xp-showcase.png`（命名惯例见 `screenshots/README.md`）。

## 真实示例文件

- `crates/yororen-ui-demos/xp_showcase/src/xp_app.rs`

## 运行

```bash
cargo run -p xp-showcase-demo
```
