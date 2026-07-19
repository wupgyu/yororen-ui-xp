# 技术设计:精调 Windows XP 渲染器样式

## 架构与边界

改动集中在三处,不动 default/brutalism renderer,不动 core 的渲染派发机制:

| 位置 | 改动性质 |
|---|---|
| `crates/yororen-ui-xp-renderer`(style.rs、renderers/*.rs、themes/xp-luna.json) | 主体:色值/几何精调 + 结构性升级 |
| `crates/yororen-ui-core/src/headless/modal.rs` | **唯一一处 core 改动**:ModalProps 附加可选字段(纯增量,见下文) |
| `crates/yororen-ui-demos/xp_showcase/src/xp_app.rs` | 标题栏圆角/caption 按钮色对齐 |

## 关键技术决策

### D1. 多段渐变:叠带 div + group 样式(实现期修订)

gpui `linear_gradient` 仅 2 色标。css 的 3-stop 按钮面与 5-stop 标题栏需要多段近似。

- ~~原方案:新增共享 `XpBandedGradientElement` 自定义 Element,逐 band `paint_quad`。~~
- **修订(实施时发现)**:gpui-ce 0.3.3 提供 `group` / `group_hover` / `group_active` 样式(div.rs),叠带 band div 以 `absolute + inset 0` 悬挂时不参与布局(不干扰内容),且 band/ring 子元素能通过 group 样式响应按钮自身的 hover/active 状态——原方案担心的"嵌套 div 干扰布局"与"自定义 Element 无法感知交互态"两个问题同时消失。因此改为:
  - 按钮面:两个 band div(`relative(0.45)` / `flex_grow`),`group_active` 反向;hover 橙环 = 内缩 1px 的透明边框 div,`group_hover` 显色 #FFCF31。
  - Modal 标题栏:4 个 band div 近似 5-stop(`style::titlebar_bands` 返回 `(fraction, from, to)`);inactive 态为普通 2-stop vgrad。
  - group 样式要求元素为 Stateful(带 id),band/ring 以 `format!("{:?}-face-top", props.id)` 等派生 id;已核实 hit_test 会收集同点全部 hitbox,子 hitbox 不抢走按钮的 hover/click。
- 无新增自定义 Element 类型;`XpProgressChunksElement` / `XpSliderTrackElement` 先例保持原样。

### D2. 按钮 hover 橙环:嵌套边框

gpui 无 inset box-shadow。hover 态在按钮边框内侧放一个 1px 边框的绝对定位 div,静止时边框色透明,经 `group_hover("xp-btn", …)` 在 hover 时显色 #FFCF31,底色渐变不变。改动限于 `renderers/actions.rs` 的按钮组装处(中性/primary/danger 三态统一,disabled 与 custom_style 不加环)。

### D3. Modal 标题按钮:core `ModalProps` 纯增量扩展

标题按钮需要回调,回调无法走 theme JSON;`ModalProps` 在 core,因此 core 附加可选字段:

- 新增 `ModalCaption` 配置结构(min/max/close 三个 `Option<回调>`,回调签名参照现有 `ModalCloseCallback` 的 `Arc<dyn Fn(&mut Window, &mut App) + Send + Sync>`)。
- `ModalProps` 增加 builder(如 `.caption(...)`);不设置时一切行为与现状一致 → 对 default/brutalism/既有调用方**零影响**(它们忽略该字段)。
- 另加 `window_active: bool`(默认 true)驱动标题栏 inactive 渐变与窗口边框色。
- 仅 XP renderer 消费这些字段;default/brutalism 是否跟进不在本任务范围。

### D4. 色值落点:双侧同步

新增/修改的 `xp.*` token(如 `xp.button.hover_ring` 值改 #FFCF31、新增 `xp.titlebar.inactive_from/to`、`xp.caption.*`、`xp.toast.*`、`xp.menu.hover_bg/fg`、`xp.progress.chunk_from/to` 改 css 绿)必须 `xp-luna.json` 与 `style.rs` fallback 双侧同值,并同步 `xp_luna_theme_parses_with_key_paths` 测试断言的路径清单。

### D5. 兼容性

- 主题:旧主题缺新 token 时走 style.rs fallback,不 break。
- API:ModalProps 增量字段,无 breaking change。
- 视觉:default/brutalism renderer 不受影响。

## 回滚

全部改动在 feature 分支;回滚 = 合入前 `git reset` / 合入后 `git revert`。core 改动为纯增量,单独 revert 也安全。
