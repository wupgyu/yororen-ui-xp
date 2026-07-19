# 目录结构 — yororen_ui_xml_macro

```
crates/yororen-ui-xml-macro/
├── Cargo.toml
└── src/lib.rs    # xml! / xml_file! / bind 等；forbid(unsafe_code)
```

---

## 职责

- 解析可选 preamble：`cx = expr,` / `window = expr,`
- 读取调用点旁 `yororen-ui-xml-components.toml`
- 将路径 **绝对化** 后再 `include_str!`（避免相对路径双重拼接）
- 调用 `yororen_ui_xml` 完成真实工作

**不要**在本 crate 复制 parser/codegen 逻辑。
