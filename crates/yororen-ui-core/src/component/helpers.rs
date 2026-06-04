//! Helper functions for UI components.
//!
//! This module provides common utility functions used across multiple components
//! to reduce code duplication.
//!
//! # Renderer-model relationship
//!
//! v0.4 introduces the per-component `XxxRenderer` trait model
//! ([`crate::renderer::ButtonRenderer`], [`crate::renderer::TextInputRenderer`],
//! etc.). Most input components (`text_input`, `text_area`,
//! `password_input`, `number_input`, `search_input`, `file_path_input`,
//! `keybinding_input`, `select`, `combo_box`) read their visuals
//! exclusively through that trait. The v0.4 `compute_input_style` helper
//! was removed in v0.4 (see V5 release notes); the same
//! disabled + custom override contract is now implemented as
//! conditional logic inside each `TokenXxxRenderer::bg`/`border`/etc.
//! method.
//!
//! Action-style and toggle-style components still go through the
//! `compute_action_style` / `compute_toggle_style` helpers in this
//! module, because the `disabled` × `variant` × `VariantStyle`
//! composition is shared across `Button` / `IconButton` /
//! `ToggleButton` / `SplitButton` (and the toggle family
//! `Checkbox` / `Switch` / `Radio`) — not just a per-component
//! lookup. These helpers are an **internal composition step** that
//! feeds into the renderer path: the component reads the
//! `ActionStyle` / `ToggleStyle` once per render and passes the
//! resolved colors down to its `TokenXxxRenderer` call. They are
//! not part of the component's public API surface.
//!
//! If you are adding a new component that needs a one-shot
//! disabled × variant × override composition, prefer
//! `compute_action_style` / `compute_toggle_style` over rolling
//! a new ad-hoc `compute_*_style` helper. The naming convention
//! is intentional.

use std::sync::Arc;

use gpui::{App, Bounds, ElementId, Entity, Pixels, Window, px};

use crate::i18n::TextDirection;
use crate::renderer::{VariantState, VariantStyle};
use crate::theme::{ActionVariantKind, Theme};

/// Computes the desired left position for a dropdown/popover menu relative to its trigger,
/// taking into account text direction, alignment preference, and window boundaries.
///
/// # Parameters
/// - `trigger_bounds` - The bounds of the trigger element
/// - `menu_width` - The width of the menu
/// - `direction` - The current text direction (LTR or RTL)
/// - `align_end` - If `true`, align the menu's end edge to the trigger's end edge
///   (right edge in LTR, left edge in RTL). If `false`, align start to start.
/// - `window` - The window for boundary clamping
///
/// # Returns
/// The absolute left position of the menu in window coordinates.
pub fn desired_menu_left(
    trigger_bounds: Bounds<Pixels>,
    menu_width: Pixels,
    direction: TextDirection,
    align_end: bool,
    window: &gpui::Window,
) -> Pixels {
    let desired_left = if align_end {
        match direction {
            TextDirection::Ltr => trigger_bounds.right() - menu_width,
            TextDirection::Rtl => trigger_bounds.left(),
        }
    } else {
        match direction {
            TextDirection::Ltr => trigger_bounds.left(),
            TextDirection::Rtl => trigger_bounds.right() - menu_width,
        }
    };

    // Use viewport size to compute content-area bounds so that clamping
    // is consistent with trigger_bounds, which is expressed in window-content
    // coordinates rather than screen coordinates.
    let viewport_size = window.viewport_size();
    let min_left = px(0.);
    let max_left = (viewport_size.width - menu_width).max(min_left);
    desired_left.clamp(min_left, max_left)
}

/// Resolves the controlled/uncontrolled state for a component.
///
/// In "controlled" mode, the component's value is managed externally via the
/// `value` parameter and changes are communicated via `on_change`. In "uncontrolled"
/// mode, the component manages its own internal state.
///
/// # Parameters
/// - `external` - The externally provided value (controlled mode)
/// - `internal` - The internal state entity (uncontrolled mode)
/// - `cx` - The app context
/// - `default_value` - The default value to use if neither external nor internal is set
///
/// # Returns
/// The resolved value based on whether the component is controlled or uncontrolled.
pub fn resolve_controlled_state<T: Clone + Default + 'static>(
    external: Option<&T>,
    internal: Option<&Entity<T>>,
    cx: &App,
    default_value: T,
) -> T {
    if let Some(value) = external {
        return value.clone();
    }

    if let Some(internal) = internal {
        return internal.read(cx).clone();
    }

    default_value
}

/// Determines whether a component should manage its own internal
/// state ("uncontrolled" in React parlance).
///
/// Despite the previous name (`use_internal_state`), this is a
/// **pure predicate** — it does not allocate any state, register
/// any hook, or interact with the gpui context. Components that
/// get back `true` from this call should then go through
/// [`create_internal_state`] to actually allocate the keyed state
/// entity.
///
/// A component is "uncontrolled" (uses internal state) when:
/// - No external value is provided (`value` is None)
/// - No external change handler is provided (`on_change` is None)
///
/// # Parameters
/// - `has_value` - Whether an external value is provided
/// - `has_on_change` - Whether an on_change callback is provided
///
/// # Returns
/// `true` if the component should manage its own internal state.
pub fn is_uncontrolled(has_value: bool, has_on_change: bool) -> bool {
    !has_value && !has_on_change
}

/// Deprecated alias for [`is_uncontrolled`]. Kept so the rename
/// doesn't break external callers during the v0.4 → v0.5 cycle.
#[deprecated(
    since = "0.4.1",
    note = "Renamed to `is_uncontrolled` — the `use_` prefix wrongly suggested a hook"
)]
pub fn use_internal_state(has_value: bool, has_on_change: bool) -> bool {
    is_uncontrolled(has_value, has_on_change)
}

/// Simplified predicate for components that only need to check
/// the callback (e.g. checkbox, radio, switch, toggle_button) —
/// they never expose an external `value` prop.
///
/// Returns `true` when the component is uncontrolled.
pub fn is_uncontrolled_simple(has_on_change: bool) -> bool {
    !has_on_change
}

/// Deprecated alias for [`is_uncontrolled_simple`]. Kept so the
/// rename doesn't break external callers during the v0.4 → v0.5
/// cycle.
#[deprecated(
    since = "0.4.1",
    note = "Renamed to `is_uncontrolled_simple` — the `use_` prefix wrongly suggested a hook"
)]
pub fn use_internal_state_simple(has_on_change: bool) -> bool {
    is_uncontrolled_simple(has_on_change)
}

/// Creates a keyed state for internal value management.
///
/// This is a convenience function that creates a use_keyed_state call
/// with a consistent prefix for input components.
///
/// # Parameters
/// - `window` - The window context
/// - `cx` - The app context
/// - `id` - The element ID for keying
/// - `key` - The state key string
/// - `default_value` - The default value for the state
///
/// # Returns
/// An optional Entity containing the internal state
pub fn create_internal_state<T: Clone + Default + 'static>(
    window: &mut Window,
    cx: &mut App,
    id: &ElementId,
    key: String,
    default_value: T,
    should_use: bool,
) -> Option<Entity<T>> {
    if should_use {
        Some(window.use_keyed_state((id.clone(), key), cx, |_, _| default_value))
    } else {
        None
    }
}

/// Updates the internal state value if it exists.
///
/// # Parameters
/// - `internal` - The internal state entity to update
/// - `cx` - The app context
/// - `new_value` - The new value to set
pub fn update_internal_state<T: Clone + 'static>(
    internal: &Option<Entity<T>>,
    cx: &mut App,
    new_value: T,
) {
    if let Some(internal) = internal {
        internal.update(cx, |state, _cx| {
            *state = new_value;
            _cx.notify();
        });
    }
}

/// Reads the value from internal state or returns the external value.
///
/// # Parameters
/// - `external` - The external value (if provided)
/// - `internal` - The internal state entity
/// - `cx` - The app context
///
/// # Returns
/// The resolved value
pub fn resolve_state_value<T: Clone + Default + 'static>(
    external: Option<&T>,
    internal: &Option<Entity<T>>,
    cx: &App,
) -> T {
    if let Some(external) = external {
        return external.clone();
    }

    if let Some(internal) = internal {
        return internal.read(cx).clone();
    }

    T::default()
}

/// Reads the value from internal state or returns the provided external value.
///
/// This is a version for components where the external value is always present
/// (not Option), like checkbox with `checked: bool`.
///
/// # Parameters
/// - `external` - The external value
/// - `internal` - The internal state entity
/// - `cx` - The app context
///
/// # Returns
/// The resolved value (internal if use_internal is true, otherwise external)
pub fn resolve_state_value_simple<T: Clone + 'static>(
    external: T,
    internal: &Option<Entity<T>>,
    cx: &App,
    use_internal: bool,
) -> T {
    if use_internal && let Some(internal) = internal {
        return internal.read(cx).clone();
    }
    external
}

/// Action component style configuration.
///
/// This struct holds the computed style values for action components
/// like Button, IconButton, ToggleButton, etc.
#[derive(Clone, Debug)]
pub struct ActionStyle {
    /// The background color.
    pub bg: gpui::Hsla,
    /// The hover background color.
    pub hover_bg: gpui::Hsla,
    /// The foreground/text color.
    pub fg: gpui::Hsla,
    /// The disabled background color.
    pub disabled_bg: gpui::Hsla,
    /// The disabled foreground/text color.
    pub disabled_fg: gpui::Hsla,
}

/// Computes the action style based on theme and component properties.
///
/// This function consolidates the common style resolution logic found in
/// Button, IconButton, and ToggleButton components.
///
/// # Parameters
/// - `theme` - The application theme
/// - `variant` - The action variant kind (Neutral, Primary, Danger)
/// - `disabled` - Whether the component is disabled
/// - `custom_bg` - Optional custom background color override
/// - `custom_hover_bg` - Optional custom hover background color override
///
/// # Returns
/// An `ActionStyle` struct containing the computed colors.
pub fn compute_action_style(
    theme: &Theme,
    variant: ActionVariantKind,
    disabled: bool,
    custom_bg: Option<gpui::Hsla>,
    custom_hover_bg: Option<gpui::Hsla>,
) -> ActionStyle {
    compute_action_style_with_custom(theme, variant, None, disabled, custom_bg, custom_hover_bg)
}

/// Like [`compute_action_style`], but accepts an optional pre-resolved
/// `VariantStyle` from the global `VariantRegistry`. When provided, the
/// custom style takes precedence over the built-in `variant` for the
/// bg / fg colors (and for `disabled_opacity` is taken from the
/// variant). The user's `custom_bg` / `custom_hover_bg` overrides still
/// win, matching the legacy behavior.
pub fn compute_action_style_with_custom(
    theme: &Theme,
    variant: ActionVariantKind,
    custom_style: Option<Arc<dyn VariantStyle>>,
    disabled: bool,
    custom_bg: Option<gpui::Hsla>,
    custom_hover_bg: Option<gpui::Hsla>,
) -> ActionStyle {
    let state = VariantState { disabled };
    let (
        variant_bg,
        variant_fg,
        variant_disabled_bg,
        variant_disabled_fg,
        variant_disabled_opacity,
    ) = match &custom_style {
        Some(s) => (
            s.bg(&state),
            s.fg(&state),
            s.bg(&VariantState { disabled: true }),
            s.fg(&VariantState { disabled: true }),
            s.disabled_opacity(),
        ),
        None => {
            let av = theme.action_variant(variant);
            (av.bg, av.fg, av.disabled_bg, av.disabled_fg, 1.0f32)
        }
    };

    // `custom_bg` / `custom_hover_bg` (from `.bg(...)` / `.hover_bg(...)`
    // on the builder) still take priority over the variant.
    let bg = custom_bg.unwrap_or(variant_bg);
    let hover_bg = custom_hover_bg.unwrap_or(bg);

    if disabled {
        return ActionStyle {
            bg: variant_disabled_bg,
            hover_bg: variant_disabled_bg,
            fg: variant_disabled_fg,
            disabled_bg: variant_disabled_bg,
            disabled_fg: variant_disabled_fg,
        };
    }

    let _ = variant_disabled_opacity; // exposed via state, not via ActionStyle
    ActionStyle {
        bg,
        hover_bg,
        fg: variant_fg,
        disabled_bg: variant_disabled_bg,
        disabled_fg: variant_disabled_fg,
    }
}

/// Toggle component style configuration.
///
/// This struct holds the computed style values for toggle components
/// like Checkbox, Switch, Radio, etc.
#[derive(Clone, Debug)]
pub struct ToggleStyle {
    /// The background color when checked/selected.
    pub bg: gpui::Hsla,
    /// The border color when checked/selected.
    pub border: gpui::Hsla,
    /// The foreground/text/icon color when checked/selected.
    pub fg: gpui::Hsla,
    /// The background color on hover when checked/selected.
    pub hover_bg: gpui::Hsla,
    /// The opacity value when disabled.
    pub disabled_opacity: f32,
}

/// Computes the toggle style based on theme and component properties.
///
/// This function consolidates the common style resolution logic found in
/// Checkbox, Switch, Radio, and other toggle components.
///
/// # Parameters
/// - `theme` - The application theme
/// - `checked` - Whether the toggle is checked/selected
/// - `disabled` - Whether the component is disabled
/// - `custom_accent` - Optional custom accent color override
///
/// # Returns
/// A `ToggleStyle` struct containing the computed colors and disabled opacity.
pub fn compute_toggle_style(
    theme: &Theme,
    checked: bool,
    disabled: bool,
    custom_accent: Option<gpui::Hsla>,
) -> ToggleStyle {
    let accent = custom_accent.unwrap_or(theme.action.primary.bg);

    if disabled {
        return ToggleStyle {
            bg: theme.surface.sunken,
            border: theme.border.muted,
            fg: theme.content.disabled,
            hover_bg: theme.surface.sunken,
            disabled_opacity: 0.5,
        };
    }

    if checked {
        ToggleStyle {
            bg: accent,
            border: accent,
            fg: theme.action.primary.fg,
            hover_bg: theme.action.primary.hover_bg,
            disabled_opacity: 1.0,
        }
    } else {
        ToggleStyle {
            bg: theme.surface.base,
            border: theme.border.default,
            fg: theme.content.primary,
            hover_bg: theme.surface.hover,
            disabled_opacity: 1.0,
        }
    }
}
