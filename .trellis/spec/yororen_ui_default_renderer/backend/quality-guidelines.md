# 质量规范 — default_renderer

> 在工作区通用质量要求之上，约束 Token 渲染器写法。

---

## 必需模式

1. **路径字符串稳定**：组件文档写明读取哪些 theme path；新增 path 时同步更新 bundled JSON。
2. **缺失 path 必须有默认值**（`unwrap_or` / `or_else` 链），禁止热路径 `unwrap`。
3. **Trait 表面保持最小**：如 `ButtonRenderer` 主要是 `compose`；调色板 helper 做 inherent method 供复用与单测。
4. **不在 renderer 里实现业务状态机**；只消费 `XxxProps` / `XxxRenderState`。
5. **注册完整**：`register_default_renderers` 覆盖全部 required slots。

---

## 禁用模式

| 禁用 | 原因 |
|------|------|
| hardcode 品牌色而不读 theme | 无法 JSON 换肤 |
| 把 layout 语义组件做成 renderer trait | layout 故意无 renderer |
| 依赖 brutalism 或 xml crate | default renderer 必须可独立使用 |
| 跳过 `cargo clippy --workspace -- -D warnings` | workspace 强制 |

---

## 测试

- 对 inherent palette helper 做单元测试（path → 颜色/尺寸）。
- 视觉回归通过 demos + PR 截图。

---

## 工作区检查

```bash
cargo test -p yororen_ui_default_renderer
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```
