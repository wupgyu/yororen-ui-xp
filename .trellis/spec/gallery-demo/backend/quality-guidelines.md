# 质量规范 — demo

> Demo 是可运行文档，不是玩具脚本。

---

## 硬性

- `publish = false`
- 仍继承 workspace lints：`cargo clippy --workspace -- -D warnings` **包含 demos**
- `cargo fmt` 同样适用
- 依赖优先 `yororen_ui` meta-crate（layers 教学可深挖 core API）

---

## 必需模式

1. `main`：`UiAsset` + `renderer::install` + locale install + `open_window`
2. 状态：`Entity` / `Global` + `cx.notify()`
3. UI：优先 headless + `.render(cx)` 或 layout 原语
4. 文件头模块文档说明「演示什么」和 `cargo run -p <pkg>`

---

## 禁用

| 禁用 | 原因 |
|------|------|
| 在 demo 合并进库 crate 源码 | 污染可发布 API |
| 提交过大的二进制资源 | 仓库体积 |
| 依赖未启用的 feature 却不在 Cargo.toml 打开 | 无法编译 |
| 忽略 workspace clippy | CONTRIBUTING 明确 demos 不豁免 |

---

## 运行

```bash
cargo run -p <package-name>
```


---

## 本 demo 特别注意

- 包名：`gallery-demo`
- 路径：`crates/yororen-ui-demos/gallery_demo`
