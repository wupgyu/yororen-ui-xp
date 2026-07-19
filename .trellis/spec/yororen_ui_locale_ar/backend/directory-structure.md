# 目录结构 — yororen_ui_locale_ar

```
crates/yororen-ui-locale-ar/
├── Cargo.toml
├── translations/ar.json
└── src/lib.rs          # translation_map() + install(cx)
```

---

## API 形态（各 locale crate 一致）

```rust
pub const LOCALE_TAG: &str = "ar";
pub fn translation_map() -> TranslationMap { /* 解析 bundled JSON */ }
pub fn install(cx: &mut gpui::App) {
    // Locale::new(LOCALE_TAG).expect(...)
    // I18n::with_locale + load_translations + cx.set_global
}
```

- LOCALE_TAG = "ar"（RTL）
- 应用侧：`yororen_ui::locale_ar::install(cx)` 或 meta `locale` 辅助模块。
