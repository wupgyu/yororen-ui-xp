//! `NumberInputRenderer` — visual side of `NumberInput`.
//!
//! The text input / keymap / IME pipeline and the +/- stepper
//! button layout live in
//! `yororen-ui-core/src/headless/number_input.rs` (and the
//! shared helpers in `text_input_element.rs`). This module
//! only provides the `TokenNumberInputRenderer` default impl.

use std::sync::Arc;

use gpui::{Hsla, Pixels, px};

use yororen_ui_core::renderer::number_input::{NumberInputRenderState, NumberInputRenderer};
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

pub struct TokenNumberInputRenderer;

impl NumberInputRenderer for TokenNumberInputRenderer {
    fn bg(&self, _state: &NumberInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.base").unwrap_or_default()
    }
    fn border(&self, _state: &NumberInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn focus_border(&self, _state: &NumberInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.focus").unwrap_or_default()
    }
    fn hover_border(&self, _state: &NumberInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn active_border(&self, _state: &NumberInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn min_height(&self, _state: &NumberInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.number_input.min_height")
            .unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &NumberInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            px(theme
                .get_number("tokens.control.number_input.horizontal_padding")
                .unwrap_or(0.0) as f32),
            px(theme
                .get_number("tokens.control.input.vertical_padding")
                .unwrap_or(0.0) as f32),
        )
    }
    fn stepper_button_size(&self, _state: &NumberInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.number_input.stepper_button_size")
            .unwrap_or(0.0) as f32)
    }
    fn border_radius(&self, _state: &NumberInputRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
}

pub fn arc_number_input<T: NumberInputRenderer + 'static>(r: T) -> Arc<dyn NumberInputRenderer> {
    Arc::new(r)
}
