# 目录结构 — showcase-xml-demo

## 目的

XML DSL 最小业务演示：Controller 方法绑定、声明式 UI 与状态分离。

## 布局

```
crates/yororen-ui-demos/showcase_xml/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── controller.rs   # 业务方法，Clone + Entity
    ├── state.rs
    ├── view.rs
    └── ui/             # XML 视图
```


## Controller 模式

- XML 只声明 UI；`on_click={...}` 指向 `Controller` 方法。
- 方法签名固定为 `(arg0, &mut Window, &mut App)`，codegen 可自动包装。
- `Controller` 必须 `Clone`（宏会捕获到多个闭包）；内部只持 `Entity<State>`。

## 禁止

- 在 XML 里写大段 inline closure / `update` 样板。
- 把网络/业务 IO 直接塞进 `Render::render`。


## 真实示例文件

- `crates/yororen-ui-demos/showcase_xml/src/controller.rs`
- `crates/yororen-ui-demos/showcase_xml/src/view.rs`
