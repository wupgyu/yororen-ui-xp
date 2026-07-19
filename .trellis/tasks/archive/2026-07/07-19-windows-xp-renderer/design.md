# 技术设计 — Windows XP 主题渲染器

## 架构与边界

新增 crate `crates/yororen-ui-xp-renderer`，目录结构完全镜像 `yororen-ui-brutalism-renderer`：

```
crates/yororen-ui-xp-renderer/
├── Cargo.toml                # deps: yororen-ui-core, yororen-ui-default-renderer, gpui-ce; [lints] workspace
├── themes/xp-luna.json       # 唯一内置主题（Luna 蓝）
└── src/
    ├── lib.rs                # install / install_with_default_theme / install_with / register_xp_renderers
    ├── style.rs              # XP 调色板常量 + 渐变/斜面/分段等共享绘制 helper
    └── renderers/
        ├── mod.rs
        ├── actions.rs        # 5:  Button, IconButton, ToggleButton, SplitButton, ButtonGroup
        ├── display.rs        # 14: Label, Heading, Divider, FocusRing, Badge, Tag, Skeleton,
        │                     #     ProgressBar, EmptyState, KeybindingDisplay, ShortcutHint, Icon, Text, Spacer
        ├── surfaces.rs       # 5:  Tooltip, Avatar, Panel, Card, Image
        ├── inputs.rs         # 9:  TextInput, TextArea, PasswordInput, NumberInput, FilePathInput,
        │                     #     SearchInput, Select, ComboBox, KeybindingInput
        ├── controls.rs       # 5:  Switch, Checkbox, Radio, RadioGroup, Slider
        ├── overlays.rs       # 6:  Modal, Popover, DropdownMenu, Disclosure, Overlay, Menu
        ├── notifications.rs  # 2:  Toast, Notification
        └── lists.rs          # 9:  ListItem, Listbox, TreeItem, Tree, Form, FormField,
                              #     Table, VirtualList, UniformVirtualList
```

边界原则：

- **纯增量**：不修改 core / default / brutalism 的任何 API 与视觉代码；只新增 crate + 接线。
- **renderer 拥有全部视觉决策**：55 个 `XpXxxRenderer` 各自实现 `compose()`，同 brutalism 模式；依赖 `yororen-ui-default-renderer` 复用其共享基础设施（如 `AnimatedPresenceElement`）。
- **渐变/斜面参数写在 `style.rs` 代码里**，主题 JSON 只承载色板与几何令牌（开放 JSON 允许 `xp.*` 扩展路径，读取带 fallback）。

## 视觉配方（Luna 蓝，纯矢量）

调色板（写入 `style.rs` 常量，关键色进 JSON）：

| 用途 | 色值 |
|---|---|
| 对话框/窗口底 | `#ECE9D8`（经典 XP 米黄） |
| 主蓝（primary/选中/标题条） | `#0058E6` → `#3A93FF` 渐变 |
| 按钮面 | `#FFFFFF` → `#E3DFD0` 纵向渐变，1px `#003C74` 边 |
| 进度条 | 白色凹陷轨道 + `#8CC63F` 绿分段块（段间 2px 间隔） |
| 输入框 | 白底 + 1px `#7F9DB9` 边，focus 蓝边 |
| Tooltip | `#FFFFE1` 淡黄 + 1px 黑边，无圆角 |
| 字体 | `Tahoma, 'Segoe UI', sans-serif`（fallback 链兼容非 Windows） |

关键组件配方：

- **Button 族**：3px 圆角、纵向 2-stop 渐变、1px 深蓝边；hover 提亮/蓝描边；active 渐变反向（压下感）；disabled 灰平色；primary 变体用蓝渐变面 + 白字。
- **ProgressBar**：凹陷轨道 + 绿色分段块；分段用 N 个子 div 或自定义 `Element`（参照 `BrutalSliderTrackElement` 模式）。
- **Checkbox/Radio**：白底凹陷框（外暗内亮双边）+ 蓝勾 / 蓝圆点。
- **Slider**：XP 轨道 + 渐变拇指。
- **Modal**：XP 窗口形态——蓝色渐变标题条 + `#ECE9D8` 内容区 + 立体边框。
- **Menu/DropdownMenu/Table/Tree/List\***：白底，hover/选中项蓝色高亮。
- **斜面（bevel）**：嵌套双边模拟 Win32 raised（外亮内暗）/ sunken（外暗内亮）；必要时用 gpui 逐边 border 或自定义 `Element`。

技术手段约束（已调研确认）：

- gpui `linear_gradient` 仅 2 个色标、无径向渐变 → 多段效果用叠加条带 / 分层 div 近似。
- 无九宫格 → 不用位图（纯矢量决策）。
- 保真门槛按 PRD："一眼即 XP"，不做像素级复刻。

## 主题 JSON

`themes/xp-luna.json` 沿用现有主题路径 schema（`surface.* / content.* / border.* / action.{neutral,primary,danger}.* / status.* / shadow.* / tokens.*`），保持 renderer 读取约定一致；`tokens.control.<widget>.*` 按 XP 尺寸调校（button radius 3、input 白底、progress 高度等）。XP 专属扩展放 `xp.*` 前缀路径，缺失时回落 `style.rs` 常量。

## API 与接入

- `lib.rs` 导出：`install(cx)`（单主题，不区分系统外观——XP 无暗色）、`install_with_default_theme(cx)`、`install_with(cx, theme)`、`register_xp_renderers(cx)`（55 条 `cx.register_renderer_arc`，镜像 `register_brutal_renderers`）。
- workspace：根 `Cargo.toml` `members` += `crates/yororen-ui-xp-renderer`、`crates/yororen-ui-demos/xp_showcase`。
- meta-crate：`yororen-ui/Cargo.toml` 加 optional dep + `xp = ["dep:yororen-ui-xp-renderer"]` feature；`src/lib.rs` 照 brutalism 方式条件 re-export。
- `scripts/publish.sh`：`PUBLISH_ORDER` 在 `yororen_ui_brutalism_renderer` 后插入 `yororen_ui_xp_renderer`。
- Demo：`crates/yororen-ui-demos/xp_showcase`（包名 `xp-showcase-demo`，`publish = false`），在 XP 米黄背景上展示核心组件；不动 gallery theme_switcher。

## 测试策略（非视觉点兜底）

1. **注册完整性**：dev-dep `gpui` + `test-support` feature（照 meta-crate 先例），TestApp 上调用 `register_xp_renderers` 后，对 55 个 marker 逐一断言 `cx.renderer_arc::<m::X, dyn XRenderer>()` 可解析不 panic。
2. **主题解析**：`Theme::from_json(XP_LUNA)` 成功且关键路径（`action.primary.bg`、`tokens.control.button.radius` 等）存在。
3. 参照系：default renderer 有 per-renderer 单测 + registry `validate()`（`renderers/registry.rs:698`）；brutalism 无测试，不作为测试模式参照。

## 兼容性与回滚

- 无 API 破坏：所有既有 crate 行为不变；meta-crate 默认 feature 集合不变（`xp` 为 opt-in）。
- 回滚 = 删除新 crate / demo 目录 + git 还原 4 处接线（根 `Cargo.toml`、meta-crate `Cargo.toml` 与 `lib.rs`、`publish.sh`、README）。无数据迁移、无运行时状态。

## 风险与权衡

- **2-stop 渐变上限**：多段蓝/高光只能近似 → 接受，符合"一眼即 XP"门槛。
- **Tahoma 字体缺失**（macOS/Linux）→ fallback 链兜底，字形差异可接受。
- **55 个 impl 的体量**（参照 brutalism ~7k 行）→ 按 8 个分组推进，每组完成后 `cargo check` + demo 目检，避免积压到末尾一次性返工。
