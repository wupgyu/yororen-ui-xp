//! `ToastRenderer` — visual side of `Toast`.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::renderers::spec::Edges;
use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct ToastRenderState {
    /// `true` if the toast has a custom override color.
    pub has_custom_color: bool,
}

pub trait ToastRenderer: Any + Send + Sync {
    fn bg(&self, state: &ToastRenderState, theme: &Theme) -> Hsla;
    fn fg(&self, state: &ToastRenderState, theme: &Theme) -> Hsla;
    fn padding(&self, state: &ToastRenderState, theme: &Theme) -> Edges<Pixels>;
    fn border_radius(&self, state: &ToastRenderState, theme: &Theme) -> Pixels;
    fn border(&self, state: &ToastRenderState, theme: &Theme) -> Hsla;
    fn shadow_alpha(&self, state: &ToastRenderState, theme: &Theme) -> f32;
}

pub struct TokenToastRenderer;

impl ToastRenderer for TokenToastRenderer {
    fn bg(&self, _state: &ToastRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.raised").unwrap_or_default()
    }
    fn fg(&self, _state: &ToastRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.primary").unwrap_or_default()
    }
    fn padding(&self, _state: &ToastRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            gpui::px(theme.get_number("tokens.spacing.inset_md").unwrap_or(0.0) as f32),
            gpui::px(theme.get_number("tokens.spacing.inset_sm").unwrap_or(0.0) as f32),
        )
    }
    fn border_radius(&self, _state: &ToastRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
    fn border(&self, _state: &ToastRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn shadow_alpha(&self, _state: &ToastRenderState, theme: &Theme) -> f32 {
        theme.get_color("shadow.elevation_2").unwrap_or_default().a
    }
}

pub fn arc_toast<T: ToastRenderer + 'static>(r: T) -> Arc<dyn ToastRenderer> {
    Arc::new(r)
}
