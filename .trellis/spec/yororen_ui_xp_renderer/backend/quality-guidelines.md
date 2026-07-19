# 质量规范 — xp_renderer

---

## 与 default / brutalism 的差异（必须保持）

| 点 | default (Token*) | brutalism (Brutal*) | xp (Xp*) |
|----|------------------|---------------------|----------|
| 按钮面 | 纯色 | 纯色 | **3-stop 纵向渐变**（双 band 叠带近似） |
| 按钮 hover | 变色 | 变色 | **橙色 inset 环 `#FFCF31`**（面不变,`group_hover` 显色) |
| 边框 | 常 `None` | 粗黑边 | 1px 深蓝 `#003C74` |
| 圆角 | 中 | 0 | 3px（弹窗 8px,仅顶部两角） |
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
- hover / 选中约定：菜单项 hover = `xp.menu.hover_bg`（#316AC5 蓝底白字）;列表/树/下拉项 hover = `xp.selection.hover_bg`（淡蓝 #CFE0FA）；选中 = `xp.selection.bg`（#316AC5）+ 白字。

---

## gpui 绘制约束（换肤必备事实）

- **一个 div 只有单一 `border_color`**（`Style.border_color: Option<Hsla>`，逐边只有宽度）。Win32 斜面（bevel）只能用两种手段：
  1. 嵌套 div（外 div 1px 边色 A + 1px padding，内 div 1px 边色 B）；
  2. 自定义 `Element` 多次 `paint_quad`（参照 `XpSliderTrackElement` 的蚀刻双边：`border_widths: Edges { top, left, ..Default::default() }` 结构体更新语法）。
- **`gpui::linear_gradient(angle, from, to)` 只有 2 个色标**、无径向渐变；angle 180° = 顶→底、90° = 左→右。多段渐变用**叠带 div** 近似：`absolute + inset 0` 的 flex 列容器里放若干 `h(relative(frac))` band,每个 band 一个 2-stop `vgrad`（参照 `actions.rs` 按钮面与 `overlays.rs` Modal 标题栏;`style::titlebar_bands` 返回 `(fraction, from, to)` 数组）。
- **交互态联动 band / 内环** 用 `group` + `group_hover` / `group_active` 样式:父元素 `.group("xp-btn")`,子元素带 id(Stateful)后 `.group_hover("xp-btn", |s| ...)`——gpui 命中测试会收集同点全部 hitbox,子 hitbox 不会抢走父级的 hover/click。按钮 hover 橙环就是内缩 1px 的透明边框 div,`group_hover` 时显色;按钮 active 反向渐变由 band 的 `group_active` 换 bg/高度实现。
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
