# 数据/资源 — yororen_ui_xml

> 无数据库。唯一“生成数据”是 schema 产物。

---

## schema_generated.rs

- 由 `cargo run -p yororen_ui_xml --bin gen-schema` 生成。
- **必须提交到 git**；CI 用 `--check` 确保与 headless API 同步：
  ```bash
  cargo run -p yororen_ui_xml --bin gen-schema -- --check
  ```
- 修改 headless 公共 builder API 后务必重跑生成。

## overrides.toml

- 用于 schema 生成覆盖/特例，不要手改 generated 文件来“修一下”。
