//! `SearchInputRenderer` — visual side of `SearchInput`.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::renderers::spec::Edges;
use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct SearchInputRenderState {
    pub disabled: bool,
    pub focused: bool,
    pub custom_bg: Option<Hsla>,
    pub custom_border: Option<Hsla>,
    pub custom_focus_border: Option<Hsla>,
    pub custom_fg: Option<Hsla>,
}

pub trait SearchInputRenderer: Any + Send + Sync {
    fn bg(&self, state: &SearchInputRenderState, theme: &Theme) -> Hsla;
    fn border(&self, state: &SearchInputRenderState, theme: &Theme) -> Hsla;
    fn focus_border(&self, state: &SearchInputRenderState, theme: &Theme) -> Hsla;
    fn icon_color(&self, state: &SearchInputRenderState, theme: &Theme) -> Hsla;
    fn fg(&self, state: &SearchInputRenderState, theme: &Theme) -> Hsla;
    fn min_height(&self, state: &SearchInputRenderState, theme: &Theme) -> Pixels;
    fn padding(&self, state: &SearchInputRenderState, theme: &Theme) -> Edges<Pixels>;
    fn border_radius(&self, state: &SearchInputRenderState, theme: &Theme) -> Pixels;
    fn input_gap(&self, state: &SearchInputRenderState, theme: &Theme) -> Pixels;
    fn icon_size(&self, state: &SearchInputRenderState, theme: &Theme) -> Pixels;
}

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
    fn icon_color(&self, _state: &SearchInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.tertiary").unwrap_or_default()
    }
    fn fg(&self, _state: &SearchInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.primary").unwrap_or_default()
    }
    fn min_height(&self, _state: &SearchInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.search_input.min_height").unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &SearchInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            gpui::px(theme.get_number("tokens.control.search_input.horizontal_padding").unwrap_or(0.0) as f32),
            gpui::px(theme.get_number("tokens.control.input.vertical_padding").unwrap_or(0.0) as f32),
        )
    }
    fn border_radius(&self, _state: &SearchInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
    fn input_gap(&self, _state: &SearchInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.search_input.input_gap").unwrap_or(0.0) as f32)
    }
    fn icon_size(&self, _state: &SearchInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.search_input.icon_size").unwrap_or(0.0) as f32)
    }
}

pub fn arc_search_input<T: SearchInputRenderer + 'static>(r: T) -> Arc<dyn SearchInputRenderer> {
    Arc::new(r)
}
