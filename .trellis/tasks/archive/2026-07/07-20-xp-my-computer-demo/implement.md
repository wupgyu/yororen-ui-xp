# 执行计划: XP My Computer demo

## 开发策略决策

- 开发模式：**inline 直接开发**（Codex inline；编辑前 `trellis-before-dev`，编辑后 `trellis-check`）
- 分支策略：**feature 分支** `feature/xp-my-computer-demo`（基于 `main`；不用 worktree）
- 开发流程：**默认流程**（视觉验收 demo，TDD 收益低；关键框架增量可补少量单测/theme 路径断言）
- 架构指导：默认 **disabled**；用户若要求再开启并更新本文件闸门顺序
- Review 闸门顺序：
  1. `trellis-check`（lint / type / test / 一致性）
  2. 视觉目检 + `screenshots/xp-my-computer.png`
  3. 对照 `prd.md` Acceptance Criteria 勾选
  4. （可选）架构回顾 — 仅当架构指导 enabled

## 有序清单

1. 创建并切换分支 `feature/xp-my-computer-demo`；`python ./.trellis/scripts/task.py set-branch feature/xp-my-computer-demo`（若命令可用）。
2. **Scaffold demo crate**
   - 新建 `crates/yororen-ui-demos/xp_my_computer/{Cargo.toml,src/main.rs}`
   - workspace `Cargo.toml` 加入 member
   - `yororen-ui` 启用 `xp` feature；`gpui-ce` 与其它 demo 对齐
3. **Assets**
   - 复制最小 `windowsIcons` 到 demo `assets/`
   - `rust-embed` + `CompositeAssetSource::new(DemoAsset, UiAsset)`
4. **框架：Modal 标题 leading（D1）**
   - `ModalProps` 增量 `title_leading`
   - `XpModalRenderer` 标题栏图标+文字
   - default/brutalism 忽略或简单兼容
5. **框架：ExplorerTaskCard + tokens（D4/D5）**
   - headless + renderer marker/registry
   - XP 渐变任务卡；default/brutalism 退化
   - `xp-luna.json` / `style.rs` / theme 路径测试
6. **Demo 根 chrome**
   - transparent OS titlebar
   - Modal 常开全屏内容 + ModalCaption 接线 min/max/close
7. **菜单栏**
   - `menu_data.rs` 移植 File/Edit/View/Favorites/Tools/Help
   - DropdownMenu + Menu；至少一个（优先全部）可展开一层
8. **功能栏 + 地址栏**
   - 静态按钮/分隔/禁用态 Back·Forward
   - Address sunken + Go；数值贴 `xp_react`
9. **内容区**
   - 左：三张 ExplorerTaskCard
   - 右：Files Stored / Hard Disk / Removable 分组与图标项
10. **编译与静态检查**
    - `cargo check -p xp-my-computer-demo`
    - `cargo check --workspace`
    - `cargo test -p yororen-ui-xp-renderer`（及 core 若有新测）
    - `cargo clippy --workspace -- -D warnings`（或项目惯用等价）
11. **视觉验收**
    - `cargo run -p xp-my-computer-demo`
    - 产出 `screenshots/xp-my-computer.png`
    - 记录 gpui 无法对齐的差异到任务 Notes
12. **质量闸门**
    - `trellis-check`
    - 勾选 `prd.md` Acceptance Criteria
    - 不破坏 `xp-showcase-demo`

## 验证命令

```bash
cargo check -p xp-my-computer-demo
cargo check --workspace
cargo test -p yororen-ui-xp-renderer
cargo test -p yororen_ui_core
cargo clippy --workspace -- -D warnings
cargo run -p xp-my-computer-demo
cargo run -p xp-showcase-demo   # 回归：不要求像素不变，要求可运行
```

## 风险文件 / 回滚点

| 文件 | 风险 |
|---|---|
| `yororen-ui-core/src/headless/modal.rs` | 公共 API 增量；保持默认行为 |
| `yororen-ui-xp-renderer/src/renderers/overlays.rs` | Modal 标题布局回归 |
| 新 `explorer_task_card` 注册路径 | 漏注册会导致 expect panic |
| `themes/xp-luna.json` + `style.rs` | token 双侧不同步 |
| demo `app.rs` 布局 | 最易堆手写样式；审查时对照 D3/D4 边界 |

回滚点建议：步骤 2–3（纯 demo）→ 4–5（框架）→ 6–9（场景）→ 10–12（验收）。

## `task.py start` 前检查

- [x] `prd.md` 需求与验收完整，产品 Open Questions 已关闭
- [x] `design.md` 技术边界与 D1–D9 已写
- [x] `implement.md` 策略与有序清单已写
- [ ] 用户审阅规划产物并同意开始实现
- [ ] （可选）架构指导 guidance 已跑并附入 design
- [ ] 切换/记录 feature 分支后执行 `python ./.trellis/scripts/task.py start`
