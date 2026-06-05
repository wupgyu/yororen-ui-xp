//! `HeadingRenderer` — the visual side of `Heading`.

use std::any::Any;
use std::sync::Arc;

use gpui::{FontWeight, Hsla, Pixels};

use yororen_ui_core::theme::Theme;
use yororen_ui_core::headless::heading::HeadingLevel;

#[derive(Clone, Copy, Debug)]
pub struct HeadingRenderState {
    pub level: HeadingLevel,
}

pub trait HeadingRenderer: Any + Send + Sync {
    fn size(&self, state: &HeadingRenderState, theme: &Theme) -> Pixels;
    fn weight(&self, state: &HeadingRenderState, theme: &Theme) -> FontWeight;
    fn color(&self, state: &HeadingRenderState, theme: &Theme) -> Hsla;
}

pub struct TokenHeadingRenderer;

impl HeadingRenderer for TokenHeadingRenderer {
    fn size(&self, state: &HeadingRenderState, theme: &Theme) -> Pixels {
        let path = match state.level {
            HeadingLevel::H1 => "tokens.typography.font_size_2xl",
            HeadingLevel::H2 => "tokens.typography.font_size_xl",
            HeadingLevel::H3 => "tokens.typography.font_size_lg",
            HeadingLevel::H4 => "tokens.typography.font_size_md",
            HeadingLevel::H5 => "tokens.typography.font_size_sm",
            HeadingLevel::H6 => "tokens.typography.font_size_xs",
        };
        gpui::px(theme.get_number(path).unwrap_or(0.0) as f32)
    }

    fn weight(&self, state: &HeadingRenderState, theme: &Theme) -> FontWeight {
        let (path, default) = match state.level {
            HeadingLevel::H1 => ("tokens.typography.weight_bold", 700.0),
            _ => ("tokens.typography.weight_semibold", 600.0),
        };
        FontWeight(theme.get_number(path).unwrap_or(default) as f32)
    }

    fn color(&self, _state: &HeadingRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.primary").unwrap_or_default()
    }
}

pub fn arc_heading<T: HeadingRenderer + 'static>(r: T) -> Arc<dyn HeadingRenderer> {
    Arc::new(r)
}
