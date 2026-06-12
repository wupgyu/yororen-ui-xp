//! High-level locale helpers for yororen-ui applications.
//!
//! The [`yororen_ui_core::i18n`] module provides the low-level runtime:
//! `I18n`, `TranslationMap`, `Locale`, etc. This module adds the
//! convenience layer that knows about the three bundled locale crates
//! (`yororen-ui-locale-en`, `zh-CN`, `ar`) so applications don't have to
//! manually match on BCP-47 tags.

use gpui::App;
use yororen_ui_core::i18n::{I18n, I18nContext, TranslationMap, parse_translation_value};

/// Supported BCP-47 locale tags bundled by yororen-ui.
pub const SUPPORTED_LOCALE_TAGS: &[&str] = &["en", "zh-CN", "ar"];

/// Install a bundled yororen-ui locale by BCP-47 tag.
///
/// Supported tags: `en`, `zh-CN`, `ar`.
///
/// Panics if `locale_tag` is not supported. This is intentional: a typo
/// in a hard-coded locale tag is a programmer error that should surface
/// at startup, not silently fall back to English.
pub fn install_locale(cx: &mut App, locale_tag: &str) {
    match locale_tag {
        "en" => crate::locale_en::install(cx),
        "zh-CN" => crate::locale_zh_cn::install(cx),
        "ar" => crate::locale_ar::install(cx),
        other => panic_support_error(other),
    }
}

/// Install a bundled yororen-ui locale and merge application-specific
/// translations on top.
///
/// This is the one-call convenience used by multi-locale applications
/// that ship their own translation catalog alongside the framework's
/// component defaults.
///
/// # Example
///
/// ```ignore
/// use yororen_ui::locale;
///
/// let app_map = locale::parse_bundled_translations(include_str!("../translations/en.json"));
/// locale::install_with_translations(cx, "en", app_map);
/// ```
pub fn install_with_translations(cx: &mut App, locale_tag: &str, app_translations: TranslationMap) {
    install_locale(cx, locale_tag);
    let locale = cx.i18n().locale().clone();
    cx.global_mut::<I18n>()
        .merge_translations(locale, app_translations);
}

/// Return the bundled framework translation map for a locale.
///
/// This is useful when an application wants to layer multiple catalogs
/// itself instead of using [`install_with_translations`].
pub fn framework_translation_map(locale_tag: &str) -> TranslationMap {
    match locale_tag {
        "en" => crate::locale_en::translation_map(),
        "zh-CN" => crate::locale_zh_cn::translation_map(),
        "ar" => crate::locale_ar::translation_map(),
        other => panic_support_error(other),
    }
}

/// Parse a JSON string that was embedded at compile time (e.g. via
/// `include_str!`) into a [`TranslationMap`].
///
/// Panics on malformed JSON or non-object JSON — bundled translation
/// files are checked-in artifacts, so syntax errors are build-time bugs.
pub fn parse_bundled_translations(raw: &str) -> TranslationMap {
    let value: serde_json::Value =
        serde_json::from_str(raw).expect("bundled locale JSON must be valid JSON");
    parse_translation_value(value).expect("bundled locale JSON must be a JSON object")
}

fn panic_support_error(tag: &str) -> ! {
    panic!(
        "unsupported locale tag {:?}; expected one of {:?}",
        tag, SUPPORTED_LOCALE_TAGS
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_tags_match_locale_crates() {
        assert_eq!(crate::locale_en::LOCALE_TAG, "en");
        assert_eq!(crate::locale_zh_cn::LOCALE_TAG, "zh-CN");
        assert_eq!(crate::locale_ar::LOCALE_TAG, "ar");
    }

    #[test]
    fn framework_translation_map_returns_framework_keys() {
        let en = framework_translation_map("en");
        assert_eq!(en.get("common.save"), Some("Save"));
        assert_eq!(en.get("select.placeholder"), Some("Select…"));

        let zh = framework_translation_map("zh-CN");
        assert_eq!(zh.get("common.save"), Some("保存"));

        let ar = framework_translation_map("ar");
        assert_eq!(ar.get("common.save"), Some("حفظ"));
    }

    #[test]
    fn parse_bundled_translations_parses_nested_json() {
        let raw = r#"{"demo": {"title": "Gallery", "actions": {"save": "Save"}}}"#;
        let map = parse_bundled_translations(raw);
        assert_eq!(map.get("demo.title"), Some("Gallery"));
        assert_eq!(map.get("demo.actions.save"), Some("Save"));
    }

    #[test]
    #[should_panic(expected = "unsupported locale tag")]
    fn framework_translation_map_panics_on_unknown_tag() {
        framework_translation_map("fr");
    }
}
