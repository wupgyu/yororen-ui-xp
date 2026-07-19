# 目录结构 — theme-showcase-demo

## 目的

主题 JSON 展示 / 切换：如何加载本地 themes 并 install_with。

## 布局

```
crates/yororen-ui-demos/theme_showcase/
├── Cargo.toml
├── themes/*.json
└── src/
    ├── main.rs
    └── theme_app.rs
```


- 演示 `Theme::from_json` + `renderer::install_with`。
- 主题文件可放在 demo 自己的 `themes/`，不必改库内 bundled 文件。


## 真实示例文件

- `crates/yororen-ui-demos/theme_showcase/src/theme_app.rs`
