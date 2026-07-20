# 技术设计: XP My Computer demo

## 架构与边界

| 位置 | 角色 |
|---|---|
| `crates/yororen-ui-demos/xp_my_computer`（新建） | 独立 demo：场景编排、菜单数据、图标资源、截图基线 |
| `crates/yororen-ui-core`（定点增量） | 仅补“不扩展就无法验收”的 headless API |
| `crates/yororen-ui-xp-renderer`（定点增量） | 仅补 XP 侧 compose / token；不动 default/brutalism 视觉 |
| `Cargo.toml` workspace members | 注册 `xp-my-computer-demo` |
| `xp_react/` | 只读参考（DOM/CSS/图标）；不作为运行时依赖 |
| `crates/yororen-ui-demos/xp_showcase` | **不改写**职责；可对照 Modal/caption 现状 |

**交付边界**

- 做：可运行 My Computer 视觉验收窗口 + 必要框架缺口定点补齐 + 截图基线。
- 不做：真实导航/系统信息/完整 Explorer 组件库/Start Menu/桌面壳。

**框架验收原则**

- 关键 XP 视觉字段优先走 headless + XP renderer + `xp-luna.json` token。
- demo 只做场景数据与布局编排；禁止长期内嵌标题栏/caption 渐变（`xp_showcase` 历史手写模式不作为目标）。
- 若某关键视觉只能靠 demo 硬编码才能过关 → 视为 gap，本任务定点补框架并记录。

## 组件映射（参考结构 → 实现落点）

```
[ OS window, transparent titlebar ]
└─ Modal (always open, no scrim) ── XpModalRenderer 窗口铬
   ├─ caption: min / max / close
   ├─ title: icon + "My Computer"
   └─ body (beige)
      ├─ Menu bar: DropdownMenu × 6 (File…Help) + Menu 面板
      ├─ Function bar: Button/Icon/Image 静态编排（含 disabled）
      ├─ Address bar: Label + Image + sunken well + Go
      └─ Content split
         ├─ Left: ExplorerTaskCard × 3（System / Other Places / Details）
         └─ Right: 分组标题 + 大图标项（Image + Label）
```

## 关键技术决策

### D1. 窗口铬：复用/扩展 Modal，不新造 Window 组件

**结论**：根 UI 使用 headless `modal` + `ModalCaption`，由 `XpModalRenderer` 画 Luna 标题栏/边框/圆角。

**接线**

- `ModalState`：`open()` 后常开；`dismiss_on_escape=false`；`dismiss_on_scrim=false`；`set_title("My Computer")`。
- `ModalCaption`：`on_minimize` / `on_maximize` / `on_close` 调 gpui `Window` API（最小化 / 最大化或 zoom / 关闭窗口）。
- OS titlebar：`TitlebarOptions { appears_transparent: true, ... }`，与 `xp_showcase` 一致隐藏系统栏，但**绘制**走 Modal renderer，不手写 band 渐变。
- 无 scrim/居中：Modal 作为窗口根内容 `w_full().h_full()` 铺满；不走 gallery 的 deferred scrim 包装。

**必要 core 扩展（标题图标）**

现状：`ModalState.title: Option<String>`，XP 标题栏只渲染文字，无 leading 图标。原版 My Computer 需要 16×16 电脑图标。

- 在 `ModalProps`（或 `ModalState`）增加可选 `title_leading: Option<AnyElement>` **或** 更窄的 `title_icon: Option<ImageSource>`。
- 推荐 **`title_leading: Option<AnyElement>`**（更通用，零业务语义）。
- `XpModalRenderer` 在标题文字前插入 leading；default/brutalism 可忽略或简单前置。
- 纯增量，无 breaking change。

**不在本任务做**

- 独立 `Window` / `TitleBar` headless 套件。
- 把 `WindowControlArea` 硬编进 Modal（除非回调路径在实现中证明不可用，再最小补 hitbox）。

### D2. 菜单栏：组合 DropdownMenu + Menu

**结论**：每个顶层项一个 `DropdownMenu`；trigger 为文本样式；content 为 `Menu`（或等价 item 列表），数据来自 `xp_react/MyComputer/dropDownData.js` 的精简移植。

- 至少 **File** 可展开并显示项（含 Close）；其余顶层可同样接线或静态 trigger，优先全部可展开一层。
- 命令：`on_select` 无业务；Close 可映射关闭窗口。
- 样式：依赖 `XpDropdownMenuRenderer` / `XpMenuRenderer`；trigger 尽量贴齐 XP 菜单栏（11px 字、扁平 hover），必要时仅在 XP renderer 微调 menubar trigger 外观，不在 demo 画菜单面板。

### D3. 功能栏 / 地址栏：现有原子组合 + token，不新增 ExplorerToolbar 组件

**结论**：MVP 用 `button` / `label` / `image` / layout 编排静态工具栏与地址栏。

- Back/Forward：`button().disabled(true)` + 图标，验收禁用态。
- Up/Search/Folders/Views：静态可点或无操作。
- 地址栏：左侧 “Address” 标签 + 白底 sunken 内容框（图标 + “My Computer” + 下拉装饰）+ Go。
- 若 sunken 边框/工具栏底色无法用现有 input/button token 表达，**只加 token**（见 D5），demo 通过 theme 取值上色，不写死魔法色常量优先。

**不在本任务做**：真实导航、地址下拉展开、Folders 侧栏联动。

### D4. 左侧任务面板：定点新增薄组件（关键视觉）

**结论**：现有 `Card`/`Panel` 的 XP 渲染是米色对话框表面，**无法**表达 Explorer 左栏蓝紫渐变任务卡。该卡是“一眼 XP”的关键特征 → 本任务定点补齐。

建议 API（最小）：

- headless：`explorer_task_card(id) -> ExplorerTaskCardProps`
  - 字段：`title: String`，`children` 由调用方 chain；可选 `collapsed` 预留但不做动画。
- XP renderer：header 横向浅蓝渐变 + 标题色 `#0C327D`；body 蓝系横向渐变；圆角顶边；行内 16px 图标 + 链接色文字。
- default/brutalism：退化为普通分组卡片（标题 + 内容），保证 registry 完整。

右侧图标区：白底分组 + 48px 图标 + 标签，优先 `image` + `label` + layout；分组标题可用 `heading`/`label.strong`。不必新增 `ExplorerIconView`。

Details 卡：静态文案（如 “System:” / “Microsoft Windows XP” 风格说明），无外链。

### D5. Token 扩展

在 `xp-luna.json` + `style.rs` fallback 双侧同步新增（命名可微调，语义固定）：

```text
xp.explorer.toolbar_bg          // 功能栏/菜单栏底 #ECE9D8 系
xp.explorer.address_border      // 地址框蓝系边
xp.explorer.task_pane_bg_from/to
xp.explorer.task_card_header_from/to
xp.explorer.task_card_body_from/mid/to
xp.explorer.task_card_title     // #0C327D
xp.explorer.content_bg          // #F1F1F1
xp.explorer.link                // 左栏链接蓝
```

同步 `xp_luna_theme_parses_with_key_paths`（或等价）路径断言。

### D6. 图标资源与 Asset 管线

- 从 `xp_react/assets/windowsIcons/` **复制**最小集合到  
  `crates/yororen-ui-demos/xp_my_computer/assets/windowsIcons/`。
- 最小集合（实现期可按缺失微调文件名）：
  - 电脑/窗口：`676(16x16).png`, `windows.png`
  - 工具栏：`back.png`, `forward.png`, `up.png`, `290.png`(Go), `299(32x32).png`(Search), `337(32x32).png`(Folders), `358(32x32).png`(Views), `dropdown.png`, `pullup.png`
  - 任务/内容：`300(16x16).png`, `302(16x16).png`, `693(16x16).png`, `308(16x16).png`, `318(16x16).png`, `318(48x48).png`, `334(48x48).png`, `111(48x48).png`, `view-info.ico` 等
- demo 内 `rust-embed` 定义 `DemoAsset`，启动：

```rust
Application::new().with_assets(CompositeAssetSource::new(DemoAsset, UiAsset))
```

- 加载：`image(id, ImageSource::Resource("windowsIcons/...".into()), cx)`。
- 图标留在 demo assets；多 demo 复用后再提升共享层。

### D7. Demo crate 结构

```text
crates/yororen-ui-demos/xp_my_computer/
  Cargo.toml                 # name = xp-my-computer-demo, features 依赖 yororen-ui/xp
  assets/windowsIcons/...
  src/main.rs                # install XP renderer + window + window
  src/app.rs                 # 根状态与 layout 编排
  src/menu_data.rs           # 菜单静态数据
  src/assets_embed.rs        # Embed + 可选 re-export
```

运行：`cargo run -p xp-my-computer-demo`  
建议窗口约 `800×600`（可按 `xp_react` 观感微调）。

### D8. 视觉保真策略

- 尺寸优先对齐 `xp_react/MyComputer/index.js` CSS：菜单栏 ~20–30px 段、功能栏高约 36、地址栏约 20、左栏宽 180、任务卡 header 23 等。
- gpui 无法 1:1 的滤镜（如 `grayscale` 禁用图标）允许近似（opacity + disabled 样式），并在任务 Notes 记录差异。
- 验收附件：`screenshots/xp-my-computer.png`（遵循 `screenshots/` 惯例）。

### D9. 兼容性与回归

- core：仅增量字段/新组件模块；既有 Modal 调用方默认行为不变。
- XP theme：新 token 缺省走 fallback。
- default/brutalism：为新 marker 注册可运行退化实现（若引入新 headless）。
- `xp-showcase-demo` 不改行为；workspace check/clippy 全绿。

## 数据流

```text
main
  → xp_renderer::install(cx)
  → CompositeAssetSource(DemoAsset, UiAsset)
  → open_window(AppEntity)
AppEntity
  → ModalState(open) + 6× DropdownMenuState + MenuState items
  → render:
       Modal.compose
         children = [menu_bar, function_bar, address_bar, content_split]
```

交互：菜单展开/关闭由 DropdownMenuState；caption 回调操作 OS 窗口；其余点击可 no-op。

## 风险与权衡

| 风险 | 缓解 |
|---|---|
| Modal 作为根窗口与 “overlay modal” 语义不完全一致 | 常开 + 无 scrim + 全尺寸；文档标明本 demo 的 window-chrome 用法 |
| Caption 回调 vs 原生 hitbox | 先回调；若拖拽/系统按钮体验不足再补 `WindowControlArea` |
| 任务卡新组件 scope creep | API 保持极薄；不做折叠动画/通用 Explorer 套件 |
| 图标版权/体积 | 仅复制 demo 所需最小 PNG/ICO |
| 禁用态灰度难还原 | opacity/disabled token 近似并记录 |

## 回滚

- demo crate + workspace member：删除 member 与目录即可。
- core/XP 增量：按文件独立 revert；优先保证 Modal leading 与 task card 可拆。

## 架构审查

默认 **不强制** 会前深度架构指导：落点均有仓库先例（Modal caption 增量、CompositeAssetSource、DropdownMenu/Menu、theme 双侧 token）。若用户要求，则在 `task.py start` 前跑 `trellis-improve-codebase-architecture`（guidance）并把结论附入本文。
