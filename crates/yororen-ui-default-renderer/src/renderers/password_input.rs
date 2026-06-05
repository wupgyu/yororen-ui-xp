//! `PasswordInputRenderer` — visual side of `PasswordInput`.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::renderers::spec::Edges;
use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct PasswordInputRenderState {
    pub disabled: bool,
    pub focused: bool,
    pub custom_bg: Option<Hsla>,
    pub custom_border: Option<Hsla>,
    pub custom_focus_border: Option<Hsla>,
    pub custom_fg: Option<Hsla>,
}

pub trait PasswordInputRenderer: Any + Send + Sync {
    fn bg(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla;
    fn border(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla;
    fn focus_border(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla;
    fn fg(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla;
    fn min_height(&self, state: &PasswordInputRenderState, theme: &Theme) -> Pixels;
    fn padding(&self, state: &PasswordInputRenderState, theme: &Theme) -> Edges<Pixels>;
    fn border_radius(&self, state: &PasswordInputRenderState, theme: &Theme) -> Pixels;
    fn toggle_icon_size(&self, state: &PasswordInputRenderState, theme: &Theme) -> Pixels;
}

pub struct TokenPasswordInputRenderer;

impl PasswordInputRenderer for TokenPasswordInputRenderer {
    fn bg(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("surface.sunken").unwrap_or_default()
        } else {
            state
                .custom_bg
                .unwrap_or_else(|| theme.get_color("surface.base").unwrap_or_default())
        }
    }
    fn border(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("border.muted").unwrap_or_default()
        } else {
            state
                .custom_border
                .unwrap_or_else(|| theme.get_color("border.default").unwrap_or_default())
        }
    }
    fn focus_border(&self, state: &PasswordInputRenderState, theme: &Theme) -> Hsla {
        state
            .custom_focus_border
            .unwrap_or_else(|| theme.get_color("border.focus").unwrap_or_default())
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
        gpui::px(theme.get_number("tokens.control.input.min_height").unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &PasswordInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            gpui::px(theme.get_number("tokens.control.input.horizontal_padding").unwrap_or(0.0) as f32),
            gpui::px(theme.get_number("tokens.control.input.vertical_padding").unwrap_or(0.0) as f32),
        )
    }
    fn border_radius(&self, _state: &PasswordInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
    fn toggle_icon_size(&self, _state: &PasswordInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.sizes.icon_sm").unwrap_or(0.0) as f32)
    }
}

pub fn arc_password_input<T: PasswordInputRenderer + 'static>(
    r: T,
) -> Arc<dyn PasswordInputRenderer> {
    Arc::new(r)
}
