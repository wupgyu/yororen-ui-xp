# 错误处理 — yororen_ui_locale_en

- `LOCALE_TAG` 非法 → `Locale::new(...).expect("LOCALE_TAG must be a valid locale")`（编译期常量，失败即 bug）。
- JSON 必须可解析；单测 `parses_bundled_json` 校验关键 key（如 `common.save`）。
- 运行时缺 key：由 **core i18n** 一次性 `eprintln!`，本 crate 不重复处理。
