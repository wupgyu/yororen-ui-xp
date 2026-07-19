# 目录结构 — inputs-demo

## 目的

输入类控件专题：text/password/number/search/keybinding 等。

## 布局

```
crates/yororen-ui-demos/inputs_demo/
├── Cargo.toml
└── src/
    ├── main.rs
    └── inputs_app.rs
```


- 聚焦输入状态、disabled、校验展示应可交互演示。
- 使用官方 headless 输入 props，不要复制私有 text_input_element 实现。


## 真实示例文件

- `crates/yororen-ui-demos/inputs_demo/src/inputs_app.rs`
