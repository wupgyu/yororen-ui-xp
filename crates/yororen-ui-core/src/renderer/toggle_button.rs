//! `ToggleButtonRenderer` — visual side of `ToggleButton`.

use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct ToggleButtonRenderState {
    pub selected: bool,
    pub disabled: bool,
}

pub trait ToggleButtonRenderer: Send + Sync {
    fn bg(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Hsla;
    fn fg(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Hsla;
    fn min_height(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Pixels;
    fn border_radius(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Pixels;
    fn disabled_opacity(&self, state: &ToggleButtonRenderState, theme: &Theme) -> f32;
}

pub struct TokenToggleButtonRenderer;

impl ToggleButtonRenderer for TokenToggleButtonRenderer {
    fn bg(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.action.neutral.disabled_bg
        } else if state.selected {
            theme.action.primary.bg
        } else {
            theme.action.neutral.bg
        }
    }
    fn fg(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Hsla {
        if state.selected {
            theme.action.primary.fg
        } else {
            theme.action.neutral.fg
        }
    }
    fn min_height(&self, _state: &ToggleButtonRenderState, theme: &Theme) -> Pixels {
        theme.tokens.control.toggle_button.min_height
    }
    fn border_radius(&self, _state: &ToggleButtonRenderState, theme: &Theme) -> Pixels {
        theme.tokens.radii.md
    }
    fn disabled_opacity(&self, _state: &ToggleButtonRenderState, _theme: &Theme) -> f32 {
        1.0
    }
}

pub fn arc_toggle_button<T: ToggleButtonRenderer + 'static>(r: T) -> Arc<dyn ToggleButtonRenderer> {
    Arc::new(r)
}
