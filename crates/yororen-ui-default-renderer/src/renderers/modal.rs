//! `ModalRenderer` — visual side of `Modal`.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::renderers::spec::Edges;
use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct ModalRenderState {}

pub trait ModalRenderer: Any + Send + Sync {
    fn scrim(&self, state: &ModalRenderState, theme: &Theme) -> Hsla;
    fn panel_bg(&self, state: &ModalRenderState, theme: &Theme) -> Hsla;
    fn panel_border(&self, state: &ModalRenderState, theme: &Theme) -> Hsla;
    fn panel_padding(&self, state: &ModalRenderState, theme: &Theme) -> Edges<Pixels>;
    fn panel_border_radius(&self, state: &ModalRenderState, theme: &Theme) -> Pixels;
    fn panel_shadow_alpha(&self, state: &ModalRenderState, theme: &Theme) -> f32;
}

pub struct TokenModalRenderer;

impl ModalRenderer for TokenModalRenderer {
    fn scrim(&self, _state: &ModalRenderState, theme: &Theme) -> Hsla {
        let mut c = theme.get_color("shadow.elevation_2").unwrap_or_default();
        c.a = 0.5;
        c
    }
    fn panel_bg(&self, _state: &ModalRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.raised").unwrap_or_default()
    }
    fn panel_border(&self, _state: &ModalRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn panel_padding(&self, _state: &ModalRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::all(gpui::px(
            theme.get_number("tokens.spacing.inset_lg").unwrap_or(0.0) as f32,
        ))
    }
    fn panel_border_radius(&self, _state: &ModalRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.radii.lg").unwrap_or(0.0) as f32)
    }
    fn panel_shadow_alpha(&self, _state: &ModalRenderState, theme: &Theme) -> f32 {
        theme.get_color("shadow.elevation_2").unwrap_or_default().a
    }
}

pub fn arc_modal<T: ModalRenderer + 'static>(r: T) -> Arc<dyn ModalRenderer> {
    Arc::new(r)
}
