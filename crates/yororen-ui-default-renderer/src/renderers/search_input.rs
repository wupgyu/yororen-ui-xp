//! `SearchInputRenderer` — visual side of `SearchInput`.
//!
//! The text input / keymap / IME pipeline plus the leading
//! search icon and trailing clear button layout live in
//! `yororen-ui-core/src/headless/search_input.rs` (and the
//! shared helpers in `text_input_element.rs`). This module
//! only provides the `TokenSearchInputRenderer` default impl.

use std::sync::Arc;

use gpui::{Hsla, Pixels, px};

use yororen_ui_core::renderer::search_input::{SearchInputRenderState, SearchInputRenderer};
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

pub struct TokenSearchInputRenderer;

impl SearchInputRenderer for TokenSearchInputRenderer {
    fn bg(&self, _state: &SearchInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.base").unwrap_or_default()
    }
    fn border(&self, _state: &SearchInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn focus_border(&self, _state: &SearchInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.focus").unwrap_or_default()
    }
    fn hover_border(&self, _state: &SearchInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn active_border(&self, _state: &SearchInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn icon_color(&self, _state: &SearchInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.tertiary").unwrap_or_default()
    }
    fn fg(&self, _state: &SearchInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.primary").unwrap_or_default()
    }
    fn min_height(&self, _state: &SearchInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.search_input.min_height")
            .unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &SearchInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            px(theme
                .get_number("tokens.control.search_input.horizontal_padding")
                .unwrap_or(0.0) as f32),
            px(theme
                .get_number("tokens.control.input.vertical_padding")
                .unwrap_or(0.0) as f32),
        )
    }
    fn border_radius(&self, _state: &SearchInputRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
    fn input_gap(&self, _state: &SearchInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.search_input.input_gap")
            .unwrap_or(0.0) as f32)
    }
    fn icon_size(&self, _state: &SearchInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.search_input.icon_size")
            .unwrap_or(0.0) as f32)
    }
}

pub fn arc_search_input<T: SearchInputRenderer + 'static>(r: T) -> Arc<dyn SearchInputRenderer> {
    Arc::new(r)
}
