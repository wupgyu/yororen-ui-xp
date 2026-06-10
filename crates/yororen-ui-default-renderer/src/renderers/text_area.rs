//! `TextAreaRenderer` — visual side of `TextArea`.
//!
//! The multi-line text input / keymap / IME pipeline (the
//! `TextAreaElement` that splits the value by `'\n'`, shapes
//! each line, paints per-row selection quads, and registers
//! the IME pipeline) lives in
//! `yororen-ui-core/src/headless/text_area_element.rs`. This
//! module only provides the `TokenTextAreaRenderer` default
//! impl that the headless `TextAreaProps::render` looks up via
//! the renderer registry.

use std::sync::Arc;

use gpui::{Hsla, Pixels, px};

use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::renderer::text_area::{TextAreaRenderState, TextAreaRenderer};
use yororen_ui_core::theme::Theme;

pub struct TokenTextAreaRenderer;

impl TextAreaRenderer for TokenTextAreaRenderer {
    fn bg(&self, state: &TextAreaRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("surface.sunken").unwrap_or_default()
        } else {
            state
                .custom_bg
                .unwrap_or_else(|| theme.get_color("surface.base").unwrap_or_default())
        }
    }
    fn border(&self, state: &TextAreaRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("border.muted").unwrap_or_default()
        } else {
            state
                .custom_border
                .unwrap_or_else(|| theme.get_color("border.default").unwrap_or_default())
        }
    }
    fn focus_border(&self, state: &TextAreaRenderState, theme: &Theme) -> Hsla {
        state
            .custom_focus_border
            .unwrap_or_else(|| theme.get_color("border.focus").unwrap_or_default())
    }
    fn hover_border(&self, _state: &TextAreaRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn active_border(&self, _state: &TextAreaRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn text_color(&self, state: &TextAreaRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("content.disabled").unwrap_or_default()
        } else {
            state
                .custom_text_color
                .unwrap_or_else(|| theme.get_color("content.primary").unwrap_or_default())
        }
    }
    fn min_height(&self, _state: &TextAreaRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.input.text_area_min_h")
            .unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &TextAreaRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::all(px(theme
            .get_number("tokens.control.input.vertical_padding")
            .unwrap_or(0.0) as f32))
    }
    fn border_radius(&self, _state: &TextAreaRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
}

pub fn arc_text_area<T: TextAreaRenderer + 'static>(r: T) -> Arc<dyn TextAreaRenderer> {
    Arc::new(r)
}
