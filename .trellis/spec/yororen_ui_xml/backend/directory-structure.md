# 目录结构 — yororen_ui_xml

---

## 布局

```
crates/yororen-ui-xml/
├── Cargo.toml
├── overrides.toml
├── benches/
├── fuzz/
└── src/
    ├── lib.rs                 # #![forbid(unsafe_code)]
    ├── ast.rs
    ├── parser.rs              # roxmltree → AST
    ├── error.rs               # XmlError / XmlErrorKind
    ├── schema.rs              # tag/attr 映射 + user schema
    ├── schema_generated.rs    # gen-schema 生成，始终提交
    ├── runtime.rs             # register_xml_component inventory
    ├── codegen/               # AST → TokenStream
    │   ├── mod.rs
    │   ├── attr.rs, leaf.rs, container.rs, control_flow.rs, ...
    │   └── tests.rs
    └── bin/
        ├── gen_schema.rs      # 生成/检查 schema_generated.rs
        └── test_codegen.rs
```

---

## 职责边界

| 模块 | 职责 |
|------|------|
| parser | XML 字符串 → AST |
| schema (+ generated) | 标签/属性合法性与映射 |
| codegen | AST → 等价 headless Rust |
| error | 结构化诊断；不依赖 proc_macro Diagnostic（可单测） |
| runtime | 自定义组件 inventory 注册 |

用户通常不直接依赖本 crate，而是通过 meta feature `xml` 使用宏。

---

## 用户扩展

- 源文件旁 `yororen-ui-xml-components.toml`：编译期注册自定义标签（见 xml-macro `load_user_schema`）。
- 示例：`crates/yororen-ui-demos/gallery_xml/src/yororen-ui-xml-components.toml`
