# 目录结构 — gallery-xml-demo

## 目的

组件画廊的 XML 版本：与 gallery_demo 对等能力，验证 xml! 覆盖面。

## 布局

```
crates/yororen-ui-demos/gallery_xml/
├── Cargo.toml
├── translations/
└── src/
    ├── main.rs
    ├── controller.rs
    ├── state.rs
    ├── view.rs
    ├── i18n.rs / theme_switcher.rs / notifications_host.rs
    ├── yororen-ui-xml-components.toml
    └── ui/
```


## 与 gallery_demo 对齐

- 状态模型、主题/语言切换、notification host 行为应对等。
- 自定义 XML 标签用旁路 `yororen-ui-xml-components.toml`。
- 变更组件 API 后同时修 Rust gallery 与 XML gallery，避免双轨漂移。


## 真实示例文件

- `crates/yororen-ui-demos/gallery_xml/src/main.rs`
- `crates/yororen-ui-demos/gallery_xml/src/yororen-ui-xml-components.toml`
