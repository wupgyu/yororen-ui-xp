# PRD: XP My Computer demo

## Goal / User Value

在 yororen-ui 中新增一个 **Windows XP 风格的 My Computer 视觉验收 demo**，以 `xp_react` 参考资料为布局与视觉基准。

**核心目的不是做“看起来像”的演示页，而是验收框架（headless + XP renderer + theme tokens）能否还原 XP Explorer 风格。**
若关键视觉字段只能靠 demo 内嵌手写样式硬凑，应视为框架能力缺口，而不是 demo 成功路径。

功能交互不需要真实实现；重点是样式与结构还原度，以及尽量通过框架能力表达。

## Confirmed Facts（仓库可验证）

### 任务与状态
- 任务目录：`.trellis/tasks/07-20-xp-my-computer-demo`
- 状态：`planning`
- 目标交付：**新建独立 demo crate**
  - 建议路径：`crates/yororen-ui-demos/xp_my_computer`
  - package 名：`xp-my-computer-demo`
  - 运行：`cargo run -p xp-my-computer-demo`
- 不改写现有 `xp-showcase` 的组件橱窗职责

### 参考资料（`xp_react/`）
- `xp_react/dom.html`：从浏览器提取的 My Computer DOM 树
- `xp_react/MyComputer/index.js`：React + styled-components 片段（布局 + CSS）
- `xp_react/MyComputer/dropDownData.js`：菜单下拉数据（File/Edit/View/...）
- `xp_react/assets/`：图标与基础 CSS（`windowsIcons/*` 等）

### 参考 UI 结构（来自 MyComputer，已按原版 XP 收敛）
窗口内容自上而下：
1. **窗口铬 / 标题栏**：Luna 标题栏 + 图标 + “My Computer” + min/max/close
2. **菜单工具栏**：File / Edit / View / Favorites / Tools / Help（至少可展开一层）
3. **功能栏**：Back / Forward / Up / Search / Folders / Views（含禁用态展示；静态为主）
4. **地址栏**：Address 标签 + 图标 + “My Computer” + 下拉装饰 + Go（静态为主）
5. **内容区**左右分栏：
   - **左侧任务面板**（蓝紫渐变卡片）：
     - System Tasks
     - Other Places
     - Details（简单系统说明，无外链）
   - **右侧图标区**（白底分组）：
     - Files Stored on This Computer
     - Hard Disk Drives（Local Disk (C:)）
     - Devices with Removable Storage（CD Drive (D:)）

### 现有项目现状
- 已有 `crates/yororen-ui-demos/xp_showcase`：组件橱窗，不是 My Computer 场景
- Workspace 需新增本 demo member
- 图片：`headless::image` + `ImageSource`；`UiAsset` 仅含通用 SVG，无 XP 盘符/电脑位图
- 已有 `CompositeAssetSource`，demo 可叠加自有 assets
- XP theme 已有 `xp.titlebar.*` token；`Modal` + `ModalCaption` 已支持 caption 按钮
- **没有** 独立 `Window` / `TitleBar` / `Explorer` / `AddressBar` / `TaskPane` headless 组件
- `xp_showcase` 顶层标题栏仍有历史性 demo 手写；本任务不以该模式为目标

### 用户明确约束
- 样式展现 + **框架能力验收**
- 功能不需要具体实现
- 参考 `xp_react` 片段，不是完整可运行应用

## Decisions Made

| 决策 | 结论 | 原因 |
|------|------|------|
| 交付形态 | 新建独立 demo crate | 与 `xp_showcase` 职责分离 |
| 内容保真度 | 收敛为更接近原版 XP | 去掉 About Me / 作者外链 |
| 窗口铬 | 优先框架能力 | Modal/caption 或本任务内抽取可复用 chrome |
| 图标资源 | 纳入必要 XP 图标 | 最小集合真实显示；优先放 demo assets |
| 界面语言 | 英文 | 便于与 `xp_react` 对照 |
| 框架边界 | 优先组合现有能力，缺口定点补 | 不做完整 Explorer 套件，也不允许 demo 大规模硬编码 |
| Details 面板 | MVP 保留简单系统说明卡 | 完善左侧三卡经典构图 |
| 菜单/下拉深度 | 至少可展开一层 | 验收 XP 菜单样式；命令无真实业务 |
| 验收附件 | 需要截图基线 | 遵循 `screenshots/` 惯例 |
| 视觉保真阈值 | 尽量贴齐 `xp_react` 数值 | 字号/栏高/色值/间距尽量还原；gpui 限制差异记入产物 |
| 功能栏/地址栏交互 | 静态样式为主 | 禁用态与 sunken 外观是重点；下拉不强制展开 |

## Requirements

1. 新增可运行 demo：`cargo run -p xp-my-computer-demo`。
2. 窗口整体观感接近 Windows XP Explorer「My Computer」。
3. 结构覆盖：窗口铬、菜单栏、功能栏、地址栏、左任务面板（System Tasks / Other Places / Details）、右内容图标区。
4. 视觉尽量贴齐 `xp_react` CSS 数值（颜色、栏高、字号、间距、图标排布）；允许个别 gpui 限制偏差并记录。
5. 不实现真实文件系统导航、系统信息读取、菜单命令业务语义、Search/Go 等业务逻辑。
6. 使用 `xp_renderer::install` 与现有 XP theme tokens。
7. 窗口铬走框架：优先复用/扩展 Modal caption 或抽取可复用 window chrome；demo 不长期内嵌标题栏实现。
8. 菜单栏至少一个顶层菜单可展开并显示菜单项；点击可不执行真实命令。
9. 功能栏与地址栏以静态样式为主：Back/Forward 等禁用态可见；地址栏下拉不强制可展开。
10. 图标：从 `xp_react/assets/windowsIcons` 选取最小必要集合，经 demo asset 管线加载。
11. 文案英文。
12. 框架边界：优先组合现有 headless + XP renderer；仅当现有组件无法表达关键 XP 视觉时，定点补框架并记录 gap。
13. 交付 `screenshots/xp-my-computer.png` 作为视觉基线。

## Acceptance Criteria

- [ ] `cargo run -p xp-my-computer-demo` 可打开 My Computer 风格窗口
- [ ] 可见：框架驱动的窗口铬 + 菜单栏 + 功能栏 + 地址栏 + 左右分栏
- [ ] 左侧含 System Tasks、Other Places、Details 三卡（Details 为静态系统说明）
- [ ] 右侧含 Files Stored / Hard Disk / Removable Storage 分组及代表性图标项
- [ ] 至少一个顶层菜单可展开显示菜单项
- [ ] 功能栏可见禁用态（如 Back/Forward）；地址栏呈 sunken/地址框样式
- [ ] 使用真实 XP 图标资源，而非长期色块/通用 SVG 占位
- [ ] 关键视觉数值尽量贴齐 `xp_react`（菜单/功能/地址栏高度、任务卡渐变与字号等）；无法对齐处记录差异
- [ ] 一眼可识别为 XP Luna Explorer，而非普通组件 showcase
- [ ] 关键视觉主要来自框架能力组合；demo 内无大规模硬编码渐变/边框/caption 实现
- [ ] 若实现中发现框架缺口：关键缺口在本任务定点补齐，并在任务产物记录 gap/决策；非关键微细节可列 follow-up
- [ ] 交付含截图基线 `screenshots/xp-my-computer.png`
- [ ] 不破坏现有 `xp-showcase-demo`

## Out of Scope

- 真实文件浏览 / 盘符枚举 / 系统信息读取
- 菜单命令真实业务语义与完整快捷键系统
- 功能栏按钮真实导航 / Search / Folders 面板联动
- 地址栏下拉展开与路径导航
- Start Menu、桌面、任务栏等其它 XP shell
- 作者个性区块（About Me、Github star iframe、Medium、Minesweeper 等）
- 完整像素级动画复刻
- 主动设计整套通用 Explorer 组件库（仅在“不补就无法验收”时定点出现）
- 中文 locale / 运行时换肤切换
- 替换或删除现有 `xp_showcase`

## Open Questions

产品意图问题已关闭。技术落点与执行策略已写入：

- `design.md`：Modal chrome 扩展、ExplorerTaskCard、token、assets、边界
- `implement.md`：inline + feature 分支 + 默认流程 + 有序清单 + 验证命令

待用户审阅上述产物并同意后，再 `task.py start` 进入实现。

## Notes / Constraints

- 本任务偏复杂：`task.py start` 前需要 `design.md` + `implement.md`。
- 图标优先放新 demo crate assets；多 demo 复用后再提升共享层。
- 范围可能包含「demo + 小范围框架补齐」。
- Codex inline：主会话直接实现与检查。
- 参考源大量 px 级尺寸，实现时在 gpui 约束下尽量贴近。
- 开发策略（branch/worktree、TDD、review 闸门）在 design/implement 阶段记录。

