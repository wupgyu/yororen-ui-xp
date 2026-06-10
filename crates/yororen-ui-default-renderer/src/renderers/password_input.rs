//! `PasswordInputRenderer` — visual side of `PasswordInput`.
//!
//! The text input / keymap / IME pipeline lives in
//! `yororen-ui-core/src/headless/text_input_element.rs`. This
//! module only provides the `TokenPasswordInputRenderer` default
//! impl. The headless `PasswordInputProps::render` masks the
//! displayed value with `mask_char` while the underlying
//! `TextInputState.value` keeps the real text.

use std::sync::Arc;

use gpui::{Hsla, Pixels, px};

use yororen_ui_core::renderer::password_input::{
    PasswordInputRenderState, PasswordInputRenderer,
};
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

pub struct TokenPasswordInputRenderer;

impl PasswordInputRenderer for TokenPasswordInputRenderer {
    fn bg(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("surface.sunken").unwrap_or_default()
        } else if state.has_custom_bg {
            state
                .custom_bg
                .unwrap_or_else(|| theme.get_color("surface.base").unwrap_or_default())
        } else {
            theme.get_color("surface.base").unwrap_or_default()
        }
    }
    fn border(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("border.muted").unwrap_or_default()
        } else if state.has_custom_border {
            state
                .custom_border
                .unwrap_or_else(|| theme.get_color("border.default").unwrap_or_default())
        } else {
            theme.get_color("border.default").unwrap_or_default()
        }
    }
    fn focus_border(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla {
        if state.has_custom_focus_border {
            state
                .custom_focus_border
                .unwrap_or_else(|| theme.get_color("border.focus").unwrap_or_default())
        } else {
            theme.get_color("border.focus").unwrap_or_default()
        }
    }
    fn hover_border(&self, _state: &PasswordInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn active_border(&self, _state: &PasswordInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn fg(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("content.disabled").unwrap_or_default()
        } else {
            state
                .custom_fg
                .unwrap_or_else(|| theme.get_color("content.primary").unwrap_or_default())
        }
    }
    fn min_height(&self, _state: &PasswordInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.input.min_height")
            .unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &PasswordInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            px(theme
                .get_number("tokens.control.input.horizontal_padding")
                .unwrap_or(0.0) as f32),
            px(theme
                .get_number("tokens.control.input.vertical_padding")
                .unwrap_or(0.0) as f32),
        )
    }
    fn border_radius(&self, _state: &PasswordInputRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
}

pub fn arc_password_input<T: PasswordInputRenderer + 'static>(
    r: T,
) -> Arc<dyn PasswordInputRenderer> {
    Arc::new(r)
}
