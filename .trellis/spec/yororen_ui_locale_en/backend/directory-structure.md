# 目录结构 — yororen_ui_locale_en

```
crates/yororen-ui-locale-en/
├── Cargo.toml
├── translations/en.json
└── src/lib.rs          # translation_map() + install(cx)
```

---

## API 形态（各 locale crate 一致）

```rust
pub const LOCALE_TAG: &str = "en";
pub fn translation_map() -> TranslationMap { /* 解析 bundled JSON */ }
pub fn install(cx: &mut gpui::App) {
    // Locale::new(LOCALE_TAG).expect(...)
    // I18n::with_locale + load_translations + cx.set_global
}
```

- LOCALE_TAG = "en"
- 应用侧：`yororen_ui::locale_en::install(cx)` 或 meta `locale` 辅助模块。
