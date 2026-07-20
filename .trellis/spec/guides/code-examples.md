# 代码示例索引

> 从仓库真实文件摘录的“推荐写法”。AI 与贡献者应优先模仿这些路径。

---

## 应用启动

**文件**：`crates/yororen-ui-demos/counter/src/main.rs`

```rust
use yororen_ui::assets::UiAsset;
use yororen_ui::locale_en;
use yororen_ui::renderer;

fn main() {
    let app = Application::new().with_assets(UiAsset);
    app.run(|cx: &mut App| {
        renderer::install(cx, cx.window_appearance());
        locale_en::install(cx);
        // ... state + open_window
    });
}
```

---

## XP 全窗应用铬（Luna 标题条 + 三边蓝框）

**文件**：`crates/yororen-ui-demos/xp_my_computer/src/main.rs`、`crates/yororen-ui-xp-renderer/src/window.rs`

```rust
app.run(|cx: &mut App| {
    xp_renderer::install(cx);
    // 透明 OS 标题栏 + 不透明客户区；标题/尺寸在此指定
    let options = XpAppWindow::window_options(cx, "My Computer", size(px(800.0), px(560.0)));
    let app_entity = cx.new(app::MyComputerApp::new);
    let _ = cx.open_window(options, |_, _cx| app_entity);
});

// app 实体：new() 里建脚手架，render() 里只给标题图标和 body
let chrome = XpAppWindow::new(cx, "My Computer");          // 常开 ModalState
self.chrome.render(Some(title_icon), body, cx)             // 蓝框 + 标题条 + caption + 拖拽区
```

窗口铬全部来自框架（`XpModalRenderer` 全窗路径），应用不要手写标题栏 / caption 按钮 / `WindowControlArea`。

---

## Headless + layout + render

**文件**：`crates/yororen-ui-demos/counter/src/counter_app.rs`

```rust
column("card", cx)
    .gap(Spacing::Lg)
    .p(Inset::Lg)
    .child(
        button("increase", cx)
            .on_click(move |_, _, cx| {
                entity.update(cx, |c, cx| {
                    c.value += 1;
                    cx.notify();
                });
            })
            .render(cx)
            .child("+"),
    )
    .render(cx);
```

---

## 纯自定义视觉（保留 headless 行为）

**文件**：`crates/yororen-ui-demos/layers_demo/src/material_button.rs`

- `button(id, cx).apply(div)` 只接线 focus/click。
- 动画状态：`window.use_keyed_state`。
- 自绘：`Element::prepaint` / `paint`。

---

## Token renderer 读 theme

**文件**：`crates/yororen-ui-default-renderer/src/renderers/button.rs`

```rust
let key = format!("action.{}.{}", state.variant.as_str(), field);
theme.get_color(&key).unwrap_or_default()
```

几何 path 示例：`tokens.control.button.radius`，回退 `tokens.radii.md`。

---

## Theme 开放模型

**文件**：`crates/yororen-ui-core/src/theme/mod.rs`

```rust
let theme = Theme::from_json(json)?;
theme.get_color("action.primary.bg");
theme.get_number("tokens.spacing.md");
```

---

## XML Controller 绑定

**文件**：`crates/yororen-ui-demos/showcase_xml/src/controller.rs`

```rust
#[derive(Clone)]
pub struct Controller {
    state: Entity<ShowcaseState>,
}

impl Controller {
    pub fn increment(&self, _ev: &ClickEvent, _w: &mut Window, cx: &mut App) {
        self.state.update(cx, |s, cx| { /* ... */ cx.notify(); });
    }
}
```

---

## Locale install

**文件**：`crates/yororen-ui-locale-en/src/lib.rs`

```rust
pub fn install(cx: &mut gpui::App) {
    let locale = Locale::new(LOCALE_TAG).expect("LOCALE_TAG must be a valid locale");
    let mut i18n = I18n::with_locale(locale.clone());
    i18n.load_translations(locale, translation_map());
    cx.set_global(i18n);
}
```

---

## 新组件检查清单（跨包）

1. `yororen-ui-core`：headless props + renderer trait + marker  
2. `yororen-ui-default-renderer`：`Token*` impl + 注册  
3. （可选）`yororen-ui-brutalism-renderer`：Brutal* impl  
4. （若 XML 暴露）更新 schema → `gen-schema`  
5. gallery_demo / gallery_xml 各加一节演示  
6. 需要文案时同步三个 locale JSON  

---

## 质量命令速查

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
cargo run -p yororen_ui_xml --bin gen-schema -- --check
```
