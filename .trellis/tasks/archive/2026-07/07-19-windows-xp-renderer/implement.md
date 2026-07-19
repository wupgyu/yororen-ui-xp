# 执行计划 — Windows XP 主题渲染器

## 开发策略决策（已确认）

- **开发模式**：当前会话 inline（视觉调校需频繁跑 demo、看截图迭代）。
- **分支策略**：当前 `main` 分支直接开发（仓库惯例：近期提交均直接在 main；无 worktree 约定）。
- **流程**：默认流程 + 关键测试（非 TDD；视觉代码断言脆弱，测试只兜注册完整性与主题解析）。
- **架构指导**：不需要（brutalism-renderer 即成熟结构模板）。
- **jsonl 上下文清单（step 1.3）**：跳过——inline 开发模式，spec 由 `trellis-before-dev` 在 Phase 2 直接加载；`implement.jsonl` / `check.jsonl` 保留种子行即可。

## Review 闸门

1. **过程中**：每个 renderer 分组完成后 `cargo check -p yororen_ui_xp_renderer` + demo 目检。
2. **完成后**：`trellis-check` 全量质量验证（spec 合规、clippy、测试、一致性）。
3. **视觉验收**：用户目检 `xp_showcase` 运行截图，按 PRD"一眼即 XP"门槛判定。

> 注：Claude Code 专属的 `trellis-spec-review` / `trellis-code-review` / `trellis-code-architecture-review` 子代理在本平台（Kimi）不存在，以闸门 2 + 3 替代。

## 有序清单

1. crate 骨架：`Cargo.toml`、`src/lib.rs`、`src/style.rs`、`src/renderers/mod.rs`、`themes/xp-luna.json`；根 `Cargo.toml` `members` 登记。
2. `xp-luna.json`：全量色板 + tokens（照 brutalism-light.json 的 schema 填 XP 值）。
3. `style.rs`：调色板常量 + 渐变 / 斜面 / 分段进度等共享 helper。
4. `renderers/actions.rs`（5）：Button 族 —— 视觉标杆，先做，定下渐变/斜面手感。
5. `renderers/display.rs`（14）：含 ProgressBar 绿色分段。
6. `renderers/surfaces.rs`（5）。
7. `renderers/inputs.rs`（9）。
8. `renderers/controls.rs`（5）：Checkbox/Radio/Slider 是 XP 辨识度重点。
9. `renderers/overlays.rs`（6）：Modal 的 XP 窗口标题条。
10. `renderers/notifications.rs`（2）。
11. `renderers/lists.rs`（9）。
12. `lib.rs`：install API + `register_xp_renderers` 55 条全量注册（对照 `markers.rs` 清单逐条核对）。
13. 测试：注册完整性（55 marker 可解析）+ 主题解析。
14. meta-crate 接线：`Cargo.toml` optional dep + `xp` feature + `lib.rs` 条件 re-export。
15. demo：`xp_showcase` 可运行（`cargo run -p xp-showcase-demo`）。
16. 视觉调校迭代：截图目检，按"一眼即 XP"门槛调参（可能多轮）。
17. 文档：`README.md` + `README_zh_CN.md`（crate 表、features、demo 表、截图）；`screenshots/xp-showcase.png`（命名按 `screenshots/README.md` 惯例）。
18. `scripts/publish.sh`：`PUBLISH_ORDER` 插入 `yororen_ui_xp_renderer`（位于 brutalism 之后）。
19. 终验：全量验证命令（见下）+ 用户视觉验收。

## 验证命令

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo run -p xp-showcase-demo   # 视觉目检
```

## 风险文件 / 回滚点

- 既有文件改动仅限接线：根 `Cargo.toml`、`crates/yororen-ui/Cargo.toml`、`crates/yororen-ui/src/lib.rs`、`scripts/publish.sh`、`README.md`、`README_zh_CN.md`。
- 回滚：git 还原上述文件 + 删除 `crates/yororen-ui-xp-renderer/` 与 `crates/yororen-ui-demos/xp_showcase/`。其余全部是新增文件，零既有行为风险。

## `task.py start` 前检查

- [x] `prd.md` / `design.md` / `implement.md` 齐备
- [ ] 用户 review 规划产物并明确批准
- [ ] 批准后执行 `python ./.trellis/scripts/task.py start 07-19-windows-xp-renderer`
