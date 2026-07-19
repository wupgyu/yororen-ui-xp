# 质量规范 — yororen_ui_xml

---

## 硬性约束

- `#![forbid(unsafe_code)]`
- schema 与 CI 同步（`.github/workflows/schema.yml`）
- 修改 codegen 时补充 `codegen/tests.rs` 或 crate 测试
- fuzz：`fuzz/fuzz_targets/`（如 `normalise_bool_attrs`）

---

## 必需工作流

```bash
# API 变更后
cargo run -p yororen_ui_xml --bin gen-schema
cargo test -p yororen_ui_xml
cargo check --workspace --no-default-features
```

---

## 禁用

| 禁用 | 原因 |
|------|------|
| 手改 `schema_generated.rs` | 下次 gen 覆盖；应用 overrides / 生成器 |
| 在 codegen 静默吞掉未知标签 | 应 UnknownTag 编译失败 |
| 把业务逻辑塞进 XML 运行时 | 设计目标是零运行时 |
