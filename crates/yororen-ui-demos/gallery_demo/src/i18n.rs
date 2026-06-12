//! Gallery demo's **own** i18n catalog.
//!
//! The locale crates under `yororen-ui-locale-*` ship the **component
//! defaults** (`common.save`, `button.neutral`, `select.placeholder`,
//! …) — strings the components themselves own. Everything else in this
//! gallery (toolbar titles, demo cell labels, section headings, sample
//! data labels, …) is **caller-specific text** and lives here.
//!
//! ## Why a separate catalog
//!
//! The `yororen-ui-locale-*` crates are framework-level. Mixing
//! demo-specific keys into them would mean every downstream consumer
//! pays the cost of catalog bytes they never use, and editing the
//! demo's strings would require rebuilding the framework crates.
//! Keeping the boundary here makes both layers evolve independently.
//!
//! ## Lookup semantics
//!
//! The gallery's `install_for_locale` flow is:
//! 1. `yororen_ui::locale::install_with_translations(cx, tag, demo_map)`
//!    — installs the framework defaults for the chosen locale and layers
//!    the demo's own keys on top.
//!
//! All demo keys live under the `demo.*` namespace so a future
//! sweep for "strings that should not be in the framework" is a
//! one-line regex.

use crate::state::LocaleChoice;

const RAW_EN: &str = include_str!("../translations/en.json");
const RAW_ZH_CN: &str = include_str!("../translations/zh-CN.json");
const RAW_AR: &str = include_str!("../translations/ar.json");

/// Demo-only translation map for the chosen locale.
fn demo_translation_map(choice: LocaleChoice) -> yororen_ui::i18n::TranslationMap {
    let raw = match choice {
        LocaleChoice::En => RAW_EN,
        LocaleChoice::ZhCn => RAW_ZH_CN,
        LocaleChoice::Ar => RAW_AR,
    };
    yororen_ui::locale::parse_bundled_translations(raw)
}

/// Install the chosen locale on `cx`, layering the gallery demo's own
/// translations on top of the component defaults from
/// `yororen-ui-locale-*`.
///
/// Idempotent: re-calling with the same `choice` is a no-op-ish merge
/// (the same keys are overwritten with the same values). Re-calling
/// with a different `choice` swaps both the active locale and the
/// demo catalog, so a toolbar toggle can hot-swap languages without
/// restarting the app.
pub fn install_for_locale(cx: &mut gpui::App, choice: LocaleChoice) {
    let locale_tag = choice.tag();
    let demo_map = demo_translation_map(choice);
    yororen_ui::locale::install_with_translations(cx, locale_tag, demo_map);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_demo(raw: &str) -> yororen_ui::i18n::TranslationMap {
        yororen_ui::locale::parse_bundled_translations(raw)
    }

    #[test]
    fn demo_keys_resolve() {
        let map = parse_demo(RAW_EN);
        assert_eq!(
            map.get("demo.title").map(String::from),
            Some("yororen-ui gallery".to_string())
        );
        assert_eq!(
            map.get("demo.section_actions").map(String::from),
            Some("1. Actions".to_string())
        );
    }

    #[test]
    fn demo_zh_keys_resolve() {
        let map = parse_demo(RAW_ZH_CN);
        assert_eq!(
            map.get("demo.title").map(String::from),
            Some("yororen-ui 画廊".to_string())
        );
    }

    #[test]
    fn demo_ar_keys_resolve() {
        let map = parse_demo(RAW_AR);
        assert_eq!(
            map.get("demo.title").map(String::from),
            Some("معرض yororen-ui".to_string())
        );
    }

    /// Every locale must have the same set of `demo.*` keys.
    /// Catches the bug where a key was added to `en.json` but
    /// the other locales silently fall back to the key path
    /// (e.g. the "common.top" → "demo.common.top" rename left
    /// `en` correct but `zh` / `ar` rendering as the raw key).
    #[test]
    fn demo_keys_present_in_all_locales() {
        fn collect(raw: &str) -> std::collections::BTreeSet<String> {
            let map = parse_demo(raw);
            map.values()
                .keys()
                .map(|k| format!("demo.{k}"))
                .collect()
        }
        let en = collect(RAW_EN);
        let zh = collect(RAW_ZH_CN);
        let ar = collect(RAW_AR);
        let in_en_only: Vec<_> = en.difference(&zh).chain(en.difference(&ar)).collect();
        let in_zh_only: Vec<_> = zh.difference(&en).collect();
        let in_ar_only: Vec<_> = ar.difference(&en).collect();
        assert!(
            in_en_only.is_empty(),
            "keys present in en but missing in zh/ar: {in_en_only:?}"
        );
        assert!(
            in_zh_only.is_empty(),
            "keys present in zh but missing in en: {in_zh_only:?}"
        );
        assert!(
            in_ar_only.is_empty(),
            "keys present in ar but missing in en: {in_ar_only:?}"
        );
    }
}
