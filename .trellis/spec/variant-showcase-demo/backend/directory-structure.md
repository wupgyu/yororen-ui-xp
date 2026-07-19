# 目录结构 — variant-showcase-demo

## 目的

ActionVariant / 自定义 VariantStyle 注册与展示。

## 布局

```
crates/yororen-ui-demos/variant_showcase/
├── Cargo.toml
└── src/
    ├── main.rs
    └── variant_app.rs
```


- 使用 core 的 `VariantRegistry` / `TokenVariantStyle` / `ActionVariantKind`。
- 自定义 variant 时颜色走 registry，几何仍可读 theme tokens。


## 真实示例文件

- `crates/yororen-ui-demos/variant_showcase/src/variant_app.rs`
