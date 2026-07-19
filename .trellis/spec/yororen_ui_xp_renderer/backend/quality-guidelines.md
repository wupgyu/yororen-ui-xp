# 质量规范 — xp_renderer

---

## 与 default / brutalism 的差异（必须保持）

| 点 | default (Token*) | brutalism (Brutal*) | xp (Xp*) |
|----|------------------|---------------------|----------|
| 按钮面 | 纯色 | 纯色 | **2-stop 纵向渐变**（白→米黄 / 蓝 / 红） |
| 边框 | 常 `None` | 粗黑边 | 1px 橄榄灰 / 深蓝 |
| 圆角 | 中 | 0 | 3px（弹窗 8px） |
| 阴影 | 常 `None` | 硬偏移阴影 | **仅 overlay 软阴影**（blur 4–12，按钮无阴影） |
| 进度条 | 连续填充 | 连续填充 | **绿色分段块**（固定像素宽 + 2px 间隔） |
| 字体 | 系统默认 | 等宽 | Tahoma（`tokens.typography.family_default`） |
| 主题数 | light + dark | light + dark | 单主题（XP 无暗色） |

---

## 必需

- 实现 **完整** 55 个 required renderer slots（与 default 同样覆盖面）。
- 共享调色板 / 渐变 / 阴影 helper 放 `style.rs`；渲染器文件内**禁止出现 hex 字面量**——XP 专属色经 `xp_color(theme, "xp.<path>", style::<fallback_fn>())` 读取，几何经 `xp_number` / `tokens.control.*` 读取。
- 依赖照 brutalism：`core + default-renderer + gpui`（复用 `AnimatedPresenceElement` 与 trait re-export）。
- disabled 用 `action.*.disabled_bg/fg` 平色，opacity 1.0（XP 不做半透明禁用）。
- hover / 选中约定：菜单、列表项 hover = `xp.selection.hover_bg`（淡蓝 #C1D2EE）；选中 = `xp.selection.bg`（#316AC5）+ 白字。

---

## gpui 绘制约束（换肤必备事实）

- **一个 div 只有单一 `border_color`**（`Style.border_color: Option<Hsla>`，逐边只有宽度）。Win32 斜面（bevel）只能用两种手段：
  1. 嵌套 div（外 div 1px 边色 A + 1px padding，内 div 1px 边色 B）；
  2. 自定义 `Element` 多次 `paint_quad`（参照 `XpSliderTrackElement` 的蚀刻双边：`border_widths: Edges { top, left, ..Default::default() }` 结构体更新语法）。
- **`gpui::linear_gradient(angle, from, to)` 只有 2 个色标**、无径向渐变；angle 180° = 顶→底、90° = 左→右。多段渐变用分层 div 叠加近似（见 `style.rs` 的 `vgrad` / `hgrad`）。
- **固定像素绘制**（进度条分段、滑杆拇指）用自定义 `Element`：`request_layout` 里 `style.size.width = gpui::relative(1.0).into()`，`paint` 里按 `bounds` 逐个 `window.paint_quad(PaintQuad { background: Background, ... })`（`PaintQuad.background` 支持渐变）。参照 `renderers/display.rs` 的 `XpProgressChunksElement`。
- 阴影只能整体加：`shadow_vec(xp_shadow(theme))` / `xp_shadow_overlay(theme)`（`style.rs` 已把 `ShadowSpec` → `Vec<BoxShadow>`）。

---

## 禁用

- 在渲染器文件硬编码颜色（绕开 `xp.*` 主题路径）——主题 JSON 换肤会失效。
- 给按钮、输入框等普通控件加阴影——XP 里只有菜单 / 弹窗 / Tooltip 才有投影。
- 只实现部分组件就宣称可 `install`。

---

## 检查

```bash
cargo test -p yororen_ui_xp_renderer        # 55 注册完整性 + 主题解析
cargo check -p yororen_ui --features xp
cargo clippy --workspace -- -D warnings
cargo run -p xp-showcase-demo               # 视觉目检
```
