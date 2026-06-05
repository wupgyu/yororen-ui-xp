//! `NumberInputRenderer` — visual side of `NumberInput`.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::renderers::spec::Edges;
use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct NumberInputRenderState {
    pub disabled: bool,
    pub focused: bool,
    pub custom_bg: Option<Hsla>,
    pub custom_border: Option<Hsla>,
    pub custom_focus_border: Option<Hsla>,
    pub custom_fg: Option<Hsla>,
}

pub trait NumberInputRenderer: Any + Send + Sync {
    fn bg(&self, state: &NumberInputRenderState, theme: &Theme) -> Hsla;
    fn border(&self, state: &NumberInputRenderState, theme: &Theme) -> Hsla;
    fn focus_border(&self, state: &NumberInputRenderState, theme: &Theme) -> Hsla;
    fn stepper_bg(&self, state: &NumberInputRenderState, theme: &Theme) -> Hsla;
    fn stepper_fg(&self, state: &NumberInputRenderState, theme: &Theme) -> Hsla;
    fn min_height(&self, state: &NumberInputRenderState, theme: &Theme) -> Pixels;
    fn padding(&self, state: &NumberInputRenderState, theme: &Theme) -> Edges<Pixels>;
    fn stepper_button_size(&self, state: &NumberInputRenderState, theme: &Theme) -> Pixels;
    fn stepper_icon_size(&self, state: &NumberInputRenderState, theme: &Theme) -> Pixels;
    fn border_radius(&self, state: &NumberInputRenderState, theme: &Theme) -> Pixels;
}

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
    fn stepper_bg(&self, _state: &NumberInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("action.neutral.bg").unwrap_or_default()
    }
    fn stepper_fg(&self, _state: &NumberInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("action.neutral.fg").unwrap_or_default()
    }
    fn min_height(&self, _state: &NumberInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.number_input.min_height").unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &NumberInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            gpui::px(theme.get_number("tokens.control.number_input.horizontal_padding").unwrap_or(0.0) as f32),
            gpui::px(theme.get_number("tokens.control.input.vertical_padding").unwrap_or(0.0) as f32),
        )
    }
    fn stepper_button_size(&self, _state: &NumberInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.number_input.stepper_button_size").unwrap_or(0.0) as f32)
    }
    fn stepper_icon_size(&self, _state: &NumberInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.number_input.stepper_icon_size").unwrap_or(0.0) as f32)
    }
    fn border_radius(&self, _state: &NumberInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
}

pub fn arc_number_input<T: NumberInputRenderer + 'static>(r: T) -> Arc<dyn NumberInputRenderer> {
    Arc::new(r)
}
