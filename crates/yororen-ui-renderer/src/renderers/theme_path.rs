//! `Theme` path helpers — small typed wrappers around
//! `Theme::get_color` / `get_number` that pick a path shape
//! the renderer crate actually consumes.
//!
//! The JSON schema is open; these helpers are the place where
//! the renderer makes a *concrete* choice about which keys to
//! read. If a different renderer wants different paths, it
//! writes its own helpers and ignores this file.
//!
//! Conventions used here:
//!
//! - `action.<variant>.<field>` — color slot for an action
//!   variant. `variant` is `neutral` / `primary` / `danger`;
//!   `field` is e.g. `bg` / `hover_bg` / `fg` / `disabled_bg`.
//! - `surface.<field>` — base canvas colors.
//! - `content.<field>` — text colors.
//! - `border.<field>` — border colors.
//! - `status.<kind>.{bg,fg}` — status pill colors.
//! - `tokens.control.<component>.<field>` — geometry / size
//!   tokens per control.
//! - `tokens.radii.<key>` — radius scale.
//! - `tokens.typography.<field>` — font sizes and weights.

use gpui::Hsla;

use yororen_ui_core::theme::Theme;

use super::button::ActionVariantKind;

/// `action.<variant>.<field>` color. Returns `Hsla::default()` if
/// the path is missing — renderers that want a different
/// fallback should call `theme.get_color` directly.
pub fn action_color(theme: &Theme, variant: ActionVariantKind, field: &str) -> Hsla {
    let key = format!("action.{}.{}", variant.as_str(), field);
    theme.get_color(&key).unwrap_or_default()
}

/// `surface.<field>` color.
pub fn surface(theme: &Theme, field: &str) -> Hsla {
    let key = format!("surface.{}", field);
    theme.get_color(&key).unwrap_or_default()
}

/// `content.<field>` color.
pub fn content(theme: &Theme, field: &str) -> Hsla {
    let key = format!("content.{}", field);
    theme.get_color(&key).unwrap_or_default()
}

/// `border.<field>` color.
pub fn border(theme: &Theme, field: &str) -> Hsla {
    let key = format!("border.{}", field);
    theme.get_color(&key).unwrap_or_default()
}

/// `status.<kind>.{bg|fg}` color.
pub fn status(theme: &Theme, kind: &str, field: &str) -> Hsla {
    let key = format!("status.{}.{}", kind, field);
    theme.get_color(&key).unwrap_or_default()
}

/// `tokens.control.button.<field>` number.
pub fn control_button(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.button.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.input.<field>` number.
pub fn control_input(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.input.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.switch.<field>` number.
pub fn control_switch(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.switch.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.toggle_button.<field>` number.
pub fn control_toggle_button(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.toggle_button.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.checkbox.<field>` number.
pub fn control_checkbox(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.checkbox.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.radio.<field>` number.
pub fn control_radio(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.radio.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.select.<field>` number.
pub fn control_select(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.select.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.combo_box.<field>` number.
pub fn control_combo_box(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.combo_box.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.avatar.<field>` number.
pub fn control_avatar(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.avatar.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.tag.<field>` number.
pub fn control_tag(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.tag.{}", field);
    theme.get_number(&key)
}

/// `tokens.control.badge.<field>` number.
pub fn control_badge(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.control.badge.{}", field);
    theme.get_number(&key)
}

/// `tokens.radii.<key>` number.
pub fn radii(theme: &Theme, key: &str) -> Option<f64> {
    let path = format!("tokens.radii.{}", key);
    theme.get_number(&path)
}

/// `tokens.radii.md` number.
pub fn radii_md(theme: &Theme) -> Option<f64> {
    radii(theme, "md")
}

/// `tokens.typography.<field>` number.
pub fn typography(theme: &Theme, field: &str) -> Option<f64> {
    let key = format!("tokens.typography.{}", field);
    theme.get_number(&key)
}
