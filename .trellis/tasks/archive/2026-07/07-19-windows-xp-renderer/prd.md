# 新增 Windows XP 主题渲染器

## Goal

为 yororen-ui 新增一个 Windows XP（Luna）视觉风格的 renderer crate，作为继 default / brutalism 之后的第三个可插拔 renderer。用户价值：应用方只需一次 `install()` 调用即可把整套 UI 切换为 XP 复古风格，无需 fork 库、无需改动 headless 层。

## Confirmed Facts（仓库调研已确认，无需再问）

- **架构支持**：core 定义 55 个 `XxxRenderer` trait（`crates/yororen-ui-core/src/renderer/`），经 `RendererRegistry` 按 marker 类型派发，注册 last-wins。新增 renderer 不需要改 core。
- **现成模板**：`crates/yororen-ui-brutalism-renderer` 即为全量换肤先例——`src/lib.rs`（install/register，234 行）+ `style.rs` + `renderers/` 8 个文件 + `themes/` 2 个 JSON，共约 7.1k 行；缺失注册会在 `XxxProps::render` 时 panic，允许部分注册（可委托 default renderer 的实现）。
- **绘制能力**（gpui-ce 0.3.3）：线性渐变（限 2 个色标，`gpui::linear_gradient`）、圆角、逐边 border、`BoxShadow`、图片、SVG、自定义 `Element` 逃生舱均可用；**无径向渐变、无九宫格**。当前两个 renderer 都未使用渐变。
- **主题机制**：`Theme` 是开放 JSON（`serde_json::Value`），按点路径读取；支持 `hover_bg/active_bg/disabled_*` 等状态色约定与 `tokens.control.<widget>.*` 几何令牌。
- **窗口装饰**：库内没有标题栏组件；gpui 支持 `WindowDecorations::Client`，XP 标题栏属于应用层工作，不在 55 个 trait 内。
- **workspace 接入点**：根 `Cargo.toml` `members` 列表；meta-crate `yororen-ui/Cargo.toml` 有 `brutalism` feature 的可选依赖接线先例；demo 侧 `gallery_demo/src/theme_switcher.rs` 手动枚举 renderer。

## Requirements

- 新增 crate `crates/yororen-ui-xp-renderer`（命名待确认），实现 XP Luna 视觉风格。
- 视觉特征（相对现有 renderer 的增量）：渐变按钮/标题质感、XP 配色（Luna 蓝为主）、3px 左右圆角、分段式绿色进度条、斜面/立体边框、XP 风格控件（checkbox、radio、slider、滚动区域等）。
- 附带 XP 主题 JSON（Luna 蓝一套，`themes/xp-luna.json`）。
- 提供与 brutalism 一致的接入 API：`install(cx)` / `install_with(cx, theme)` / `register_*_renderers(cx)`。
- 接入 workspace 与 meta-crate 可选 feature。
- demo 可切换到 XP renderer 展示效果：新增独立 demo `crates/yororen-ui-demos/xp_showcase`（仿 theme_showcase 模式），不使用 gallery theme_switcher 混排。

## Acceptance Criteria

- [ ] `cargo check --workspace` 与 `cargo clippy --workspace -- -D warnings` 通过。
- [ ] **全量 55 个 `XxxRenderer` trait 均有 XP 实现并注册**（不允许运行时 `expect("XxxRenderer registered")` panic；参照 brutalism 的注册清单逐一核对）。
- [ ] demo 中切换到 XP renderer 后所有组件正常渲染、无 Token 风格混排。
- [ ] 新增独立 demo `xp_showcase` 可运行（`cargo run -p xp-showcase-demo`），展示 XP 风格核心组件。
- [ ] 视觉验收（"一眼即 XP"门槛）：button、progress、checkbox/radio、slider、窗口背景等核心组件第一眼可辨认是 XP Luna 风格（蓝渐变、绿分段进度条、斜面边框）；其余组件合理近似即可。由用户目检截图验收，不逐像素对比；精调留后续任务。
- [ ] 新增截图记录（按 `screenshots/README.md` 惯例）。

## Out of Scope（待用户确认）

- XP 窗口标题栏 / 窗口装饰（应用层，非 55 trait 范围）— 已确认出界。
- 位图皮肤 / 九宫格切图 — 已确认不做（纯矢量路线）。
- 修改 default / brutalism 现有 renderer 的视觉。

## Open Questions

1. ~~组件覆盖范围~~ ✅ 已确认：**全量 55 个 trait 均实现 XP 风格**，不做 Token 委托混排。
2. ~~保真策略~~ ✅ 已确认：**纯矢量近似**，不引入位图资产；多段渐变用分层叠加 / 自定义 Element 实现。
3. ~~XP 标题栏 / 窗口装饰~~ ✅ 已确认：**不纳入本任务**（应用层职责，后续可单独立任务）。
4. ~~配色变体~~ ✅ 已确认：**只出 Luna 蓝一套** `xp-luna.json`；渐变/斜面参数在 renderer 代码内，JSON 承载色板与几何令牌。
5. ~~crate 命名~~ ✅ 已确认：crate `yororen-ui-xp-renderer`（目录 `crates/yororen-ui-xp-renderer`），meta-crate feature 名 `xp`。
6. ~~demo 集成~~ ✅ 已确认：新增独立 demo `crates/yororen-ui-demos/xp_showcase`（包名 `xp-showcase-demo`），不改 gallery theme_switcher。
7. ~~视觉保真门槛~~ ✅ 已确认："一眼即 XP"——核心组件第一眼可辨认，其余合理近似；用户目检截图验收，精调另立后续任务。

## Notes

- 本任务为复杂任务：规划产物已齐备——`prd.md`（本文件）+ `design.md`（技术设计）+ `implement.md`（执行计划与开发策略决策）。
- 规划闸门：brainstorm + grill-me 已完成，10 个决策全部闭环；仓库可回答的问题已通过调研确认（见 Confirmed Facts）。
- 下一步：用户 review 规划产物 → 批准后 `task.py start`（step 1.4）进入实现。
