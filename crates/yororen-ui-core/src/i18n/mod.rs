//! Internationalization (i18n) module.
//!
//! This module provides internationalization support including:
//! - Translation key-value system with nested keys
//! - Runtime language switching
//! - Plural forms support (CLDR rules)
//! - Placeholder replacement
//! - Number and date/time formatting
//! - RTL (right-to-left) support预留
//!
//! # Usage
//!
//! ## Basic Translation
//!
//! ```ignore
//! use gpui::App;
//! use yororen_ui::i18n::I18nContext;
//!
//! // Get translated string
//! let text = cx.t("select.placeholder");
//! ```
//!
//! ## With Placeholders
//!
//! Positional (`{}`) substitutions:
//!
//! ```ignore
//! let text = cx.t_with_args("greeting", &["World"]);
//! ```
//!
//! Named placeholders (`{name}`) — preferred for i18n because order
//! can vary across languages:
//!
//! ```ignore
//! use yororen_ui::{t, t_named};
//!
//! let text = t_named!(cx, "greeting", name => "World");
//! ```
//!
//! ## Plural Forms
//!
//! In your translation JSON file:
//! ```json
//! {
//!   "items": {
//!     "one": "{count} item",
//!     "other": "{count} items"
//!   }
//! }
//! ```
//!
//! ```ignore
//! use gpui::App;
//! use yororen_ui::i18n::I18nContext;
//!
//! let text = cx.tn("items", n = 5);
//! ```

pub mod format;
pub mod loader;
pub mod locale;
pub mod placeholder;
pub mod runtime;
pub mod translate;

pub use format::{
    CurrencyDisplay, DateTimeFormatOptions, DateTimeFormatter, DateTimeLength, Formatter,
    I18nFormatter, NumberFormatOptions, NumberFormatter,
};
pub use loader::{
    FallbackLoader, FileLoader, LoadError, TranslationLoader, filename_for_locale,
    parse_translation_value,
};
pub use locale::{Locale, SupportedLocale, TextDirection};
pub use placeholder::{
    GlobalPlaceholderResolver, NoopPlaceholderResolver, PlaceholderContext, PlaceholderKey,
    PlaceholderResolver,
};
pub use runtime::{I18n, I18nContext, Translate, TranslationMap};
// `t!` and `t_named!` are #[macro_export], so they live at crate root;
// pull them into the `i18n` module as well for discoverability.
pub use crate::{t, t_named};
pub use translate::{PluralCategory, TranslatedString, Translator};

// Re-export commonly used types
pub use locale::Locale as I18nLocale;
