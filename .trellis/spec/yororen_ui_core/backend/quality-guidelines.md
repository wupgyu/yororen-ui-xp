# 质量规范

> 本包代码质量标准与禁用模式（来自仓库真实约定）。

---

## 概览

- 工作区统一 lint：`[lints] workspace = true`，根 `Cargo.toml` 将 `clippy.all = "warn"`。
- PR / 贡献检查清单见 `CONTRIBUTING.md` 与 `.github/PULL_REQUEST_TEMPLATE.md`。
- 公共 API 必须有文档注释；遵循 [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)。
- 提交信息使用 Conventional Commits：`feat(button): ...` / `fix(toast): ...`。

---

## 强制检查

在请求 review 前必须通过：

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

涉及 XML schema 时额外：

```bash
cargo run -p yororen_ui_xml --bin gen-schema -- --check
cargo check --workspace --no-default-features
```

---

## 必需模式

1. **三层架构边界**
   - `headless`：纯行为 / props / 状态机，不做视觉决策。
   - `renderer`：把 props / RenderState 变成 styled `div`，读 theme path。
   - `theme` JSON：开放 palette（`action.primary.bg` 等），无固定 schema。
2. **依赖方向**（不可反向依赖）
   ```
   theme JSON → renderer → core → gpui-ce
   ```
3. **交互组件入口**：`headless/` 是构造交互元素的唯一方式；通过 `.apply(div)` 或 `.render(cx)` 使用。
4. **Builder + clone 友好回调**：`on_click` 等使用 `Arc<dyn Fn(...) + Send + Sync>`。
5. **gpui 依赖**：统一 `gpui = { package = "gpui-ce", version = "0.3" }`（第三方 fork，发布名不同）。
6. **成员 crate 版本**：`version.workspace = true`，`edition.workspace = true`（edition = 2024）。

---

## 禁用模式

| 禁用 | 原因 |
|------|------|
| 在 `yororen-ui-core` 写颜色 / 圆角 / 阴影等视觉决策 | 破坏 headless 契约；视觉属 renderer + theme |
| 在 headless 直接 hardcode palette | 无法切换 renderer / theme |
| 新增组件却不注册 `RendererRegistry` slot | `.render(cx)` 运行时找不到 renderer |
| 修改 headless 公共 API 后不跑 `gen-schema --check` | XML 宏 schema 会过期（CI 会挂） |
| 引入 `unsafe` 到 xml 相关 crate | 已 `#![forbid(unsafe_code)]` |
| 在 demo 里绕过 meta-crate 直接拼深层路径（除非演示分层） | 应用代码应优先 `use yororen_ui::...` |
| 跳过 `cargo fmt` / clippy `-D warnings` | workspace 与 PR 检查硬性要求 |

---

## 测试要求

- 库 crate：关键路径加单元测试；gpui 测试用 `features = ["test-support"]`。
- 视觉变更：PR 描述附截图或短视频。
- 公共 API 变更：更新文档注释；破坏性变更在 PR 的 Breaking Changes 写明。
- demo：`publish = false`，不发布到 crates.io，但仍受 workspace clippy 约束。

---

## Code Review 清单

- [ ] 是否破坏三层边界（core / renderer / theme）？
- [ ] 新组件是否同时有 headless + renderer trait/impl +（如适用）XML schema？
- [ ] theme path 是否有合理 `unwrap_or` / `unwrap_or_default` 回退？
- [ ] 公共 API 文档是否齐全？
- [ ] 是否需要更新 `AGENTS.md` / CONTRIBUTING 相关流程说明？


---

## Core 额外规则

- **禁止视觉决策**：不在 headless 里设置 bg/fg/radius/shadow（文档与 `button` 模块注释明确约定）。
- `apply` 只接线 a11y / 交互；`render` 才走 renderer。
- `Context<T>` 通过 `DerefMut<Target = App>` 拿到 `&mut App`（gallery state 注释中的 v0.3 模式）。
- 状态复合组件用 `Entity<XxxState>`，简单 props 用纯值字段。
- 图标资源走 `assets` + `UiAsset` / `rust-embed`，locale **不再** 嵌在 core（独立 locale crate）。
