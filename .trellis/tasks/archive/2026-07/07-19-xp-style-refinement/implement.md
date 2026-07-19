# 执行计划:精调 Windows XP 渲染器样式

## 开发策略决策

- 开发模式:**inline 直接开发**(编辑前 `trellis-before-dev`,编辑后 `trellis-check`)
- 分支策略:**feature 分支** `feature/xp-style-refinement`
- 开发流程:**默认流程**(视觉改动为主,TDD 收益低;验收靠 check/clippy/test + 逐项清单 + 截图目检)
- 架构指导:不需要(改动模式均有仓库先例,见 design.md D1/D2)
- Review 闸门:编辑后 `trellis-check`(lint + type + test + 一致性),截图由用户目检

## 有序清单

1. 创建并切到 feature 分支 `feature/xp-style-refinement`;`task.py set-branch` 记录。
2. **token 双侧**:style.rs fallback + `xp-luna.json` 同步新增/调整色值与几何(按钮边 #003C74、hover 环 #FFCF31、focus #316AC5、listbox 边 #7F9DB9、item hover #CFE0FA、menu hover #316AC5/白字、progress chunk #68D868→#189418、track 边 #7F9DB9、高 12、toast #FFFFE1/黑边/radius 6、titlebar 5-stop/inactive/caption 系列、modal 边 active/inactive)。
3. **共享 element**:`XpBandedGradientElement`(design D1)。
4. **actions.rs**:按钮 3-stop 面、边框 #003C74、hover 橙环嵌套边框、active 反向、min-height 23 / padding 3-14 / disabled #ACA899。
5. **inputs.rs**:focus 边框 #316AC5。
6. **lists.rs**:容器边 #7F9DB9,item hover #CFE0FA。
7. **overlays.rs(Menu)**:菜单项 hover #316AC5 蓝底白字。
8. **notifications.rs**:Toast/Notification 气泡黄样式。
9. **display.rs**:进度条 css 绿 + track 边/高度;divider 蚀刻双线。
10. **core modal.rs**:ModalProps 增量 caption/active 字段(design D3)。
11. **overlays.rs(Modal)**:标题栏 4-band 纵向渐变 26px、radius 8 8 0 0、inactive 态、窗口边框 active/inactive、caption 按钮(21×21、半透明白边、蓝/红渐变、回调接线)。
12. **demo xp_app.rs**:标题栏圆角 8、caption 按钮渐变色对齐 css。
13. 同步 `xp_luna_theme_parses_with_key_paths` 测试;`cargo test -p yororen-ui-xp-renderer`。
14. `cargo check --workspace`、`cargo clippy --workspace -- -D warnings` 全绿。
15. `cargo run -p xp-showcase-demo` 截图,按 prd.md 逐项清单自查 → 用户目检。
16. 更新 `screenshots/`(按 `screenshots/README.md` 惯例)。
17. `trellis-check` → spec 更新(若产出新约定)→ 收尾。

## 验证命令

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test -p yororen-ui-xp-renderer
cargo run -p xp-showcase-demo   # 截图验收
```

## 风险文件 / 回滚点

- `renderers/actions.rs`(按钮结构改动):hover 嵌套边框可能影响布局尺寸,注意内外边距补偿。
- `renderers/overlays.rs`(Modal 重做):标题栏 band 元素与圆角裁剪(`overflow_hidden`)配合。
- `core/headless/modal.rs`(唯一 core 改动):纯增量,其他 renderer 不消费新字段;单独可 revert。
- 每个大步骤(2-3 / 4-9 / 10-11 / 12-16)完成后即可编译验证,作为中间回滚点。
