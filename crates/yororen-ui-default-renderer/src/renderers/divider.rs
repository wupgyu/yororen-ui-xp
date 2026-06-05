//! `DividerRenderer` — the visual side of `Divider`.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct DividerRenderState {
    pub vertical: bool,
}

pub trait DividerRenderer: Any + Send + Sync {
    fn color(&self, state: &DividerRenderState, theme: &Theme) -> Hsla;
    fn thickness(&self, state: &DividerRenderState, theme: &Theme) -> Pixels;
}

pub struct TokenDividerRenderer;

impl DividerRenderer for TokenDividerRenderer {
    fn color(&self, _state: &DividerRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.divider").unwrap_or_default()
    }

    fn thickness(&self, _state: &DividerRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.divider.thickness").unwrap_or(0.0) as f32)
    }
}

pub fn arc_divider<T: DividerRenderer + 'static>(r: T) -> Arc<dyn DividerRenderer> {
    Arc::new(r)
}
