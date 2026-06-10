//! `TextInputRenderer` — visual side of `TextInput`.
//!
//! The text input / keymap / IME pipeline lives in
//! `yororen-ui-core/src/headless/text_input_element.rs`
//! (`TextInputElement`, `wire_input_keyboard`,
//! `start_cursor_blink`). This module only provides the
//! `TokenTextInputRenderer` default impl that the headless
//! `TextInputProps::render` looks up via the renderer registry.

use std::sync::Arc;

use gpui::{Hsla, Pixels, hsla, px};

use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::renderer::text_input::{TextInputRenderState, TextInputRenderer};
use yororen_ui_core::theme::Theme;

pub struct TokenTextInputRenderer;

impl TextInputRenderer for TokenTextInputRenderer {
    fn bg(&self, state: &TextInputRenderState, theme: &Theme) -> Hsla {
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
    fn border(&self, state: &TextInputRenderState, theme: &Theme) -> Hsla {
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
    fn focus_border(&self, state: &TextInputRenderState, theme: &Theme) -> Hsla {
        if state.has_custom_focus_border {
            state
                .custom_focus_border
                .unwrap_or_else(|| theme.get_color("border.focus").unwrap_or_default())
        } else {
            theme.get_color("border.focus").unwrap_or_default()
        }
    }
    fn hover_border(&self, _state: &TextInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn active_border(&self, _state: &TextInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn text_color(&self, state: &TextInputRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("content.disabled").unwrap_or_default()
        } else if state.custom_text_color.is_some() {
            state.custom_text_color.unwrap()
        } else {
            theme.get_color("content.primary").unwrap_or_default()
        }
    }
    fn hint_color(&self, _state: &TextInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.tertiary").unwrap_or_default()
    }
    fn cursor_color(&self, state: &TextInputRenderState, theme: &Theme) -> Hsla {
        if state.has_custom_focus_border {
            state
                .custom_focus_border
                .unwrap_or_else(|| theme.get_color("border.focus").unwrap_or_default())
        } else {
            theme.get_color("border.focus").unwrap_or_default()
        }
    }
    fn selection_color(&self, _state: &TextInputRenderState, theme: &Theme) -> Hsla {
        let c = theme.get_color("border.focus").unwrap_or_default();
        hsla(c.h, c.s, c.l, 0.25)
    }
    fn min_height(&self, _state: &TextInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.input.min_height")
            .unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &TextInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            px(theme
                .get_number("tokens.control.input.horizontal_padding")
                .unwrap_or(0.0) as f32),
            px(theme
                .get_number("tokens.control.input.vertical_padding")
                .unwrap_or(0.0) as f32),
        )
    }
    fn border_radius(&self, _state: &TextInputRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
    fn disabled_opacity(&self, state: &TextInputRenderState, _theme: &Theme) -> f32 {
        if state.disabled { 0.6 } else { 1.0 }
    }
}

pub fn arc_text_input<T: TextInputRenderer + 'static>(r: T) -> Arc<dyn TextInputRenderer> {
    Arc::new(r)
}
