# 质量规范 — brutalism_renderer

---

## 与 default 的差异（必须保持）

| 点 | default (Token*) | brutalism (Brutal*) |
|----|------------------|---------------------|
| 边框 | 常 `None` | 粗边框 |
| 阴影 | 常 `None` | 硬偏移阴影（如 4px Y） |
| 文件组织 | 每组件一文件 | domain 分组 |
| 启用方式 | 默认可用 | optional feature |

---

## 必需

- 实现 **完整** required renderer slots（与 default 同样覆盖面）。
- 共享几何/阴影 helper 放 `style.rs`，避免 8 个 domain 文件复制。
- 依赖 `core + default-renderer + gpui`：复用 default renderer 的 `AnimatedPresenceElement` 等共享元素与 trait re-export，但**不得**把 `TokenButtonRenderer` 等具体视觉实现当作默认再"微调一点点"——风格应独立。

---

## 禁用

- 在 brutalism 中调用 `TokenButtonRenderer` 作为默认再“微调一点点”——风格应独立，必要时可共享 path 约定但不要纠缠实现。
- 只实现部分组件就宣称可 `install`。

---

## 检查

```bash
cargo test -p yororen_ui_brutalism_renderer
cargo check -p yororen_ui --features brutalism
```
