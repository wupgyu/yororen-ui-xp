# 目录结构 — gallery-demo

## 目的

组件画廊（纯 Rust headless API）：全组件展示、主题/渲染器/语言切换、notification host。

## 布局

```
crates/yororen-ui-demos/gallery_demo/
├── Cargo.toml
├── translations/{en,zh-CN,ar}.json
└── src/
    ├── main.rs
    ├── gallery_app.rs
    ├── state.rs              # 纯值字段 + Entity 复合状态
    ├── i18n.rs
    ├── theme_switcher.rs     # DarkMode / RendererKind
    ├── notifications_host.rs
    └── sections/             # 按组件分区
```


## 状态拆分

- 简单控件：纯值字段 + on_change。
- 复合控件（modal/popover/select/...）：`Entity<XxxState>`，在 `GalleryApp::new` 铸造。
- 从 `Context<Self>` 取 `App`：`&mut **cx`（DerefMut 到 App）。

## 必须演示的横切能力

- `notification_host` 挂在窗口根。
- locale 切换 en / zh-CN / ar。
- default vs brutalism renderer 切换（若启用）。


## 真实示例文件

- `crates/yororen-ui-demos/gallery_demo/src/state.rs`
- `crates/yororen-ui-demos/gallery_demo/src/main.rs`
