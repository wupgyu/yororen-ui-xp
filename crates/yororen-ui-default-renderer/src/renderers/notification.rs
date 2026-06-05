//! `NotificationRenderer` — visual side of the notification host.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::renderers::spec::Edges;
use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct NotificationRenderState {}

pub trait NotificationRenderer: Any + Send + Sync {
    fn bg(&self, state: &NotificationRenderState, theme: &Theme) -> Hsla;
    fn border(&self, state: &NotificationRenderState, theme: &Theme) -> Hsla;
    fn padding(&self, state: &NotificationRenderState, theme: &Theme) -> Edges<Pixels>;
    fn border_radius(&self, state: &NotificationRenderState, theme: &Theme) -> Pixels;
    fn shadow_alpha(&self, state: &NotificationRenderState, theme: &Theme) -> f32;
}

pub struct TokenNotificationRenderer;

impl NotificationRenderer for TokenNotificationRenderer {
    fn bg(&self, _state: &NotificationRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.raised").unwrap_or_default()
    }
    fn border(&self, _state: &NotificationRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn padding(&self, _state: &NotificationRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::all(gpui::px(
            theme.get_number("tokens.spacing.inset_md").unwrap_or(0.0) as f32,
        ))
    }
    fn border_radius(&self, _state: &NotificationRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.radii.lg").unwrap_or(0.0) as f32)
    }
    fn shadow_alpha(&self, _state: &NotificationRenderState, theme: &Theme) -> f32 {
        theme.get_color("shadow.elevation_2").unwrap_or_default().a
    }
}

pub fn arc_notification<T: NotificationRenderer + 'static>(r: T) -> Arc<dyn NotificationRenderer> {
    Arc::new(r)
}
