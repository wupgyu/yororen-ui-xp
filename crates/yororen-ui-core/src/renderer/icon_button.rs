//! `IconButtonRenderer` — visual side of `IconButton`.

use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct IconButtonRenderState {
    pub disabled: bool,
    pub has_custom_bg: bool,
    pub has_custom_hover_bg: bool,
}

pub trait IconButtonRenderer: Send + Sync {
    fn bg(&self, state: &IconButtonRenderState, theme: &Theme) -> Hsla;
    fn hover_bg(&self, state: &IconButtonRenderState, theme: &Theme) -> Hsla;
    fn size(&self, state: &IconButtonRenderState, theme: &Theme) -> Pixels;
    fn border_radius(&self, state: &IconButtonRenderState, theme: &Theme) -> Pixels;
    fn disabled_opacity(&self, state: &IconButtonRenderState, theme: &Theme) -> f32;
}

pub struct TokenIconButtonRenderer;

impl IconButtonRenderer for TokenIconButtonRenderer {
    fn bg(&self, state: &IconButtonRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.action.neutral.disabled_bg
        } else {
            theme.action.neutral.bg
        }
    }
    fn hover_bg(&self, _state: &IconButtonRenderState, theme: &Theme) -> Hsla {
        theme.action.neutral.hover_bg
    }
    fn size(&self, _state: &IconButtonRenderState, theme: &Theme) -> Pixels {
        theme.tokens.control.button.icon_button_min_size
    }
    fn border_radius(&self, _state: &IconButtonRenderState, theme: &Theme) -> Pixels {
        theme.tokens.radii.md
    }
    fn disabled_opacity(&self, _state: &IconButtonRenderState, _theme: &Theme) -> f32 {
        1.0
    }
}

pub fn arc_icon_button<T: IconButtonRenderer + 'static>(r: T) -> Arc<dyn IconButtonRenderer> {
    Arc::new(r)
}
