# PRD: XP Notepad demo

## Goal / User Value

在 yororen-ui 中新增一个 **Windows XP 风格的 Notepad（记事本）视觉/框架验收 demo**，用来验证现有项目框架（headless + XP renderer + `XpAppWindow` + theme tokens）能否还原经典 XP 应用窗口形态。

**核心目的是验收框架能力，而不是做完整记事本产品。**
若关键视觉字段只能靠 demo 内嵌手写样式硬凑，应视为框架能力缺口。

## Confirmed Facts（仓库可验证）

### 任务与状态
- 任务目录：`.trellis/tasks/07-20-xp-notepad-demo`
- 状态：`planning`
- 目标交付：新建独立 demo crate
  - 路径：`crates/yororen-ui-demos/xp_notepad`
  - package：`xp-notepad-demo`
  - 运行：`cargo run -p xp-notepad-demo`
- 不改写现有 `xp_showcase` / `xp_my_computer` 职责

### 可复用参考
- 最近归档任务：`archive/2026-07/07-20-xp-my-computer-demo`
  - 独立 demo crate + workspace member
  - `xp_renderer::install` + `XpAppWindow` 全窗铬
  - 菜单栏用 `Menu` + `DropdownMenu`
  - 截图基线：`screenshots/`
- 框架已有：
  - `XpAppWindow`（`crates/yororen-ui-xp-renderer/src/window.rs`）
  - `headless::text_area` + `XpTextAreaRenderer`（Luna 白色 sunken 多行输入）
  - `TextInputState::set_value` 可用于程序化清空（实现期选择可测路径）
  - `headless::menu` / `dropdown_menu`
  - XP theme tokens（titlebar、input、explorer 等）
- 无独立 StatusBar 组件；本 demo MVP 不做状态栏
- `xp_react/` 仅有 My Computer 参考，**无 Notepad 参考源**
- 图标：`xp_react/assets/windowsIcons/edit.png`、`ie-paper.png` 等可用作标题图标候选

### 经典 XP Notepad UI 结构
窗口内容自上而下：
1. **窗口铬 / 标题栏**：Luna 标题栏 + 文档图标 + `Untitled - Notepad` + min/max/close
2. **菜单栏**：File / Edit / Format / View / Help（五项 + 代表性子集）
3. **编辑区**：占满剩余客户区的可编辑多行文本（白底 sunken），启动带短示例文案
4. **状态栏**：MVP **不做**

### 用户明确约束
- 测试当前项目框架
- 简易 demo
- 使用 XP 样式

## Decisions Made

| 决策 | 结论 | 原因 |
|------|------|------|
| 交付形态 | 新建独立 demo crate | 与 showcase / my-computer 职责分离 |
| 窗口铬 | 使用 `XpAppWindow` | 已验证的框架脚手架 |
| 主题 | `xp_renderer::install` | 与现有 XP demo 一致 |
| 参考深度 | 无 `xp_react` Notepad 源；按经典结构 + 现有组件能力还原 | 仓库无专用参考 |
| 交互深度 | **可编辑文本**；菜单命令多数静态 | 同时验收 chrome、菜单、`text_area` |
| 状态栏 | **MVP 不做** | 非识别关键；无框架组件；保持简易 |
| 界面语言 | **英文** | 与 my-computer / 经典 XP 对照一致 |
| 菜单完整度 | **五项顶栏 + 代表性子集** | 辨识度高、数据量可控 |
| 少量菜单生效 | **Exit 关窗 + New 清空文本**；其余静态/禁用 | 证明 on_select 通路，不做文件 I/O |
| 初始内容 | **短示例文案** | 截图与首屏更易识别；New 可清空 |
| 窗口默认尺寸 | ~720×480（实现期可微调） | 比 Explorer 略小，贴近经典 Notepad |
| 任务复杂度 | **轻量：PRD-only** | 结构简单，优先组合现有能力；发现框架缺口再升格 |
| 开发模式 | **Codex inline（主会话直接实现/检查）** | 当前平台模式 |
| 分支策略 | **feature 分支**（对齐 my-computer） | 与归档 demo 任务一致 |
| 流程 | **默认流程（非 TDD）** | 视觉/集成验收为主，非算法逻辑 |
| 架构审查 | 不强制 | 范围小、无结构重构 |

## Requirements

1. 新增可运行 demo：`cargo run -p xp-notepad-demo`。
2. 窗口整体观感接近 Windows XP Notepad。
3. 结构覆盖：XP 窗口铬、五项菜单栏、主编辑区（**无状态栏**）。
4. 使用 `xp_renderer::install` 与现有 XP theme / renderer 能力。
5. 窗口铬走 `XpAppWindow`，demo 不手写 Luna 标题栏 / caption。
6. 菜单栏为 File / Edit / Format / View / Help；每项含代表性子集，至少可展开一层。
7. 主编辑区使用 `headless::text_area`，可实际输入多行文本；视觉来自 `XpTextAreaRenderer`。
8. 启动时编辑区带短英文示例文案。
9. **File → Exit** 关闭窗口；**File → New** 清空编辑区内容。
10. 界面文案为英文（标题 `Untitled - Notepad` 等）。
11. 不实现真实文件系统读写 / 打印 / 字体对话框完整业务。
12. 不做底部状态栏。
13. 不破坏现有 XP demos。
14. 交付含截图基线 `screenshots/xp-notepad.png`。

### 菜单代表性子集（指导，实现可微调）

- **File**: New, Open…, Save, Save As…, separator, Page Setup…, Print…, separator, Exit  
  （Open/Save/Print 等可 disabled 或 no-op；New/Exit 生效）
- **Edit**: Undo, separator, Cut, Copy, Paste, Delete, separator, Find…, Replace…, separator, Select All  
  （多数 disabled/静态；键盘快捷键可展示文案）
- **Format**: Word Wrap, Font…（静态/禁用即可）
- **View**: Status Bar（静态；与「不做状态栏」一致，可显示未勾选外观若组件支持，否则普通项）
- **Help**: Help Topics, separator, About Notepad（静态）

## Acceptance Criteria

- [x] `cargo run -p xp-notepad-demo` 可启动
- [x] 一眼可识别为 XP Luna Notepad 风格窗口（标题栏 + 菜单 + 大编辑区）
- [x] 具备 `XpAppWindow` 标题栏 + 五项菜单栏 + 主编辑区；**无状态栏**
- [x] 菜单至少一层可展开；文案为英文
- [x] 主编辑区可输入多行文本，视觉为 XP sunken 文本区，主要来自框架 renderer
- [x] 启动带短示例文案；File → New 清空；File → Exit 关窗
- [x] 关键视觉不靠 demo 内大规模硬编码 chrome
- [x] 有截图基线 `screenshots/xp-notepad.png`
- [x] 不破坏 `xp-showcase-demo` / `xp-my-computer-demo`
- [x] `cargo check -p xp-notepad-demo` 通过（实现期）

## Out of Scope

- 真实打开/保存文件、打印、查找替换完整业务
- 字体选择对话框 / 完整 Format 业务
- 底部状态栏（含行列跟踪）与 View → Status Bar 真实切换
- Word Wrap 真实开关（除非实现期零成本可做）
- Start Menu、桌面、任务栏等 shell
- 像素级动画复刻
- 中文 locale / 运行时换肤
- 改写现有 showcase / my-computer
- 主动设计通用 Notepad/文档编辑组件库

## Open Questions

产品意图问题已关闭。

实现期技术细节（不阻塞 PRD）：
- 标题图标选用 `ie-paper.png` / `edit.png` 或其它最小 assets
- New 清空：持有 `TextInputState` entity vs remount key
- 菜单/编辑区具体间距 token 选用

## Development Strategy（轻量任务记录于 PRD）

- **开发模式**: Codex inline（主会话实现 + 检查）
- **分支**: `feature/xp-notepad-demo`（base: `main`）
- **流程**: 默认流程（非 TDD）
- **架构审查**: 不启用
- **复杂度**: 轻量 PRD-only；若必须改框架公共 API/渲染契约，再补 `design.md` / `implement.md` 并升格

## Notes / Constraints

- 优先组合现有能力；demo 内允许少量布局 `div`（菜单条背景、编辑区铺满），不允许手写整套 Luna 标题栏。
- 图标优先放新 demo crate assets（可从 `xp_react` / my-computer 复用最小集合）。
- 验收以「一眼像 XP Notepad + 可编辑 + 菜单通路」为准，不追求像素级 CSS 复刻。
