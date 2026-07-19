# 目录结构 — layers-demo

## 目的

分层架构教学：自定义视觉（如 Material ripple）如何组合 headless button。

## 布局

```
crates/yororen-ui-demos/layers_demo/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── layers_app.rs
    └── material_button.rs   # 自定义 Element + headless apply
```


## 关键教学点

- headless `button(...).apply(div)` 只接线交互；视觉完全自定义。
- 可用 `window.use_keyed_state` 保存动画状态（ripple）。
- 这是“第三种用法”：不是 Token renderer，也不是纯 XML，而是 app 自绘。

参考：`material_button.rs` 顶部模块文档（ripple 三阶段：mousedown → prepaint → paint）。


## 真实示例文件

- `crates/yororen-ui-demos/layers_demo/src/material_button.rs`
- `crates/yororen-ui-demos/layers_demo/src/layers_app.rs`
