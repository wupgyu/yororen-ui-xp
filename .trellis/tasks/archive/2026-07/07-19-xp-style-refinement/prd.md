# 精调 Windows XP 渲染器样式

## Goal

以 `xp_css/xp.css`(第三方 HTML/CSS 实现的 XP Luna 样式,171 行)为参照基准,精调 `yororen-ui-xp-renderer` 与 `xp_showcase` demo 的视觉细节,把上一任务"一眼即 XP"的近似提升到"逐项对得上参考值"的保真度。

## Confirmed Facts(仓库调研已确认,无需再问)

- **xp.css 参考值已全文读取**,覆盖:窗口/标题栏/标题按钮、按钮、输入框、listbox、菜单栏、工具栏、状态栏、fieldset、tabbar、进度条、滚动条、气泡通知。
- **renderer 结构**:`crates/yororen-ui-xp-renderer/src/style.rs`(全部色值 fallback + helper)+ `renderers/` 8 文件按 domain 分组,55 个 trait 全注册;`themes/xp-luna.json` 承载共享 schema + `xp.*` 扩展 + `tokens.*` 几何。
- **spec 硬约束**(`.trellis/spec/yororen_ui_xp_renderer/backend/quality-guidelines.md` 等):
  - renderer 文件内禁止 hex 字面量 → 色值修改必须 `xp-luna.json` + `style.rs` fallback **双侧同步**。
  - gpui `linear_gradient` 仅 2 色标、无径向渐变、无 text-shadow;多段渐变用叠带法(demo `xp_app.rs:124-151` 已有 4-band 先例)或自定义 Element 多次 `paint_quad`(renderer 内 `XpSliderTrackElement`、`XpProgressChunksElement` 为先例)。
  - 仅 overlay 允许软阴影;普通控件禁阴影;选中=#316AC5 白字是既有约定。
- **现状与 xp.css 的差距**(explore 调研结论):
  - 已接近:输入框(仅 focus 边色 #0058E6 vs css #316AC5)、checkbox/radio、进度条(已分段,色调偏黄绿 #B5E388→#6DAF1B vs css #68D868→#189418)、tooltip 底色、选中色、modal 米黄底。
  - 明显不同:按钮(中性边框 #7F7B6B vs css #003C74;面渐变 2-stop vs css 3-stop;hover 蓝色 vs css 橙色 inset 环;min-height 26/padding 12-4 vs css 23/14-3)、Modal 标题栏(横向 2-stop vs css 纵向 5-stop、28px vs 26px、无 inactive 态)、菜单 hover(#C1D2EE 淡蓝黑字 vs css #316AC5 蓝底白字)、Toast/Notification(白底 bevel vs css 气泡黄)、listbox 容器边 #ACA899 vs #7F9DB9、divider 单线 vs 蚀刻双线。
  - xp.css 中 menubar/toolbar/statusbar/fieldset/tabbar/scrollbar 在 55 个 renderer slot 中**无对应组件**,无落点。
- **demo 标题栏**(`xp_showcase/src/xp_app.rs`)已用 css 原值(4-band 渐变链 #0997FF→#0053EE→#0050EE→#0066FF→#0058EB);caption 按钮色近似(#5CA8F8→#1E6AE1 vs css #3C8CFD→#1565E8;close #F0A080→#D04E20 vs css #F08A6D→#D84A28);圆角 7px vs css 8px。

## Requirements

- 以 xp.css 为基准逐项修正 renderer 色值/几何(落点:`xp-luna.json` + `style.rs` fallback 双侧,遵守禁 hex 字面量规则)。
- 结构性升级(多段渐变、inset 橙环)用叠带法/嵌套边框或自定义 Element 实现,不引入位图资产。
- Modal 新增可选标题按钮 props(min/max/close,带回调),标题栏升级纵向 5-stop 渐变 + inactive 态。
- 保持 55 个 trait 全注册、现有测试通过;`xp_luna_theme_parses_with_key_paths` 测试按需同步。
- demo 标题栏/caption 按钮按 css 值补齐细节(圆角 8px、按钮渐变色)。

## Acceptance Criteria(逐项核对清单)

构建与测试:

- [ ] `cargo check --workspace` 与 `cargo clippy --workspace -- -D warnings` 通过。
- [ ] `cargo test -p yororen-ui-xp-renderer` 通过(token 路径测试同步)。

按钮(对照 xp.css `.xp-btn`):

- [ ] 面渐变 3-stop 效果:#FFF → #ECE9D8(45%)→ #DDD8C8(叠带/分层近似)。
- [ ] 中性按钮边框 #003C74(深蓝),radius 3px。
- [ ] min-height 23px,padding 3px 14px。
- [ ] hover:橙色 inset 环 #FFCF31,底色不变。
- [ ] active:渐变反向(#DDD8C8 → #ECE9D8 55% → #FFF)。
- [ ] disabled:文字/边框 #ACA899。

输入框 / 选择类:

- [ ] focus 边框 #316AC5。
- [ ] listbox/tree/虚拟列表容器边框 #7F9DB9。
- [ ] listbox item hover #CFE0FA,选中 #316AC5 白字。

菜单 / 浮层:

- [ ] 菜单项 hover:#316AC5 蓝底白字。
- [ ] tooltip 保持 #FFFFE1( radius 维持 0,css 无 tooltip 定义)。

Toast/Notification(对照 `.xp-balloon`):

- [ ] #FFFFE1 黄底 + 1px 黑边 + radius 6 + 软阴影。

进度条(对照 `.xp-progress`):

- [ ] 分段填充色调整为 css 绿:#68D868 → #189418。
- [ ] track 边框 #7F9DB9,高度 12px。

窗口/Modal(对照 `.xp-window`/`.xp-titlebar`/`.xp-tb-btn`):

- [ ] Modal 标题栏:纵向 5-stop 渐变(#0997FF→#0053EE→#0050EE→#0066FF→#0058EB,叠带法),高 26px,radius 8px 8px 0 0。
- [ ] 标题栏 inactive 态:渐变 #B8C4DC→#98A8C0。
- [ ] 窗口边框 active #0058E6 / inactive #98A8C0。
- [ ] Modal 可选标题按钮:21×21、radius 3、半透明白边 rgba(255,255,255,.6)、蓝渐变 #3C8CFD→#1565E8、close 红渐变 #F08A6D→#D84A28,带回调 props。

其他:

- [ ] divider 改蚀刻双线(凸起/凹陷两条 1px 线)。
- [ ] demo 标题栏圆角 8px、caption 按钮渐变色对齐 css(#3C8CFD→#1565E8 / #F08A6D→#D84A28)。
- [ ] `cargo run -p xp-showcase-demo` 可运行,用户目检截图最终确认。
- [ ] 新增/更新截图记录(按 `screenshots/README.md` 惯例)。

## Out of Scope

- xp.css 中无 renderer 落点的组件:menubar、toolbar、statusbar、fieldset、tabbar、scrollbar(55 个 slot 无对应 trait)。
- 位图皮肤/九宫格;修改 default/brutalism renderer。
- text-shadow(gpui 无此能力)。
- demo 桌面/任务栏/开始按钮等 OS 级绘制。

## Open Questions

1. ~~精调覆盖范围~~ ✅ 已确认:**全量覆盖**——所有已识别差距项一次修完,含色值修正与结构性升级。
2. ~~标题按钮是否进 renderer~~ ✅ 已确认:**进**。Modal 新增可选 min/max/close props(带回调),样式按 css `.xp-tb-btn`;标题栏同步升级 5-stop 纵向渐变 + inactive 态。
3. ~~按钮 hover 样式~~ ✅ 已确认:**改为橙色 inset 环 #FFCF31**(嵌套边框实现,底色不变),替换现有 #C1D2EE 蓝色 hover。
4. ~~Toast/Notification 样式~~ ✅ 已确认:**改为 css 气泡样式**(#FFFFE1 黄底 + 1px 黑边 + radius 6 + 软阴影),与 tooltip 统一。
5. ~~验收颗粒度~~ ✅ 已确认:**逐项核对清单**——验收标准按组件列目标值,实现后逐项自查,用户目检 demo 截图最终确认。

## 实现期变更(用户目检后确认)

- **窗口外框(demo + Modal)**:补充 css `.xp-window` 三件套——1px #0058E6 外框、顶部 8px 圆角(仅上侧两角,底部直角)、body 3px 内缩 + #A09C8C 内边框(顶边开放);新增 `xp.window.body_border` token。
- **窗口外阴影:取消**。demo 曾按 css `box-shadow` 自绘外阴影(透明边距 + BoxShadow),但 Win10 上 DWM 会在 client 区边缘画 1px #AAAAAA 边线(`WS_EX_NOREDIRECTIONBITMAP` + NCCALCSIZE insets 的已知 artifact):`DWMWA_NCRENDERING_POLICY=disabled`(+SWP_FRAMECHANGED)无效;`DwmExtendFrameIntoClientArea(-1)` 虽能去掉线,但玻璃背景把透明边距变成白霜且 DWM 在右上角补画系统 min/max/close 按钮。用户决策:放弃自绘外阴影,全部回退(边距/BoxShadow/DWM FFI/窗口尺寸),仅保留圆角/蓝框/body 内边框。Modal 的应用内 overlay 软阴影不受影响。
