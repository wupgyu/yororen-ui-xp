//! XP (Luna) notification renderers: `Toast`, `Notification`.
//!
//! Both follow the XP balloon-help look: raised white background,
//! 1px bevel border, 3px rounded corners, and the theme's soft
//! overlay shadow at full strength.

use gpui::{Hsla, Pixels, px};
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

use crate::style::{self, XP_RADIUS, bevel_inner_dark, xp_color};

// =====================================================================
// Toast
// =====================================================================

pub use yororen_ui_core::renderer::toast::{ToastRenderState, ToastRenderer};

pub struct XpToastRenderer;

impl ToastRenderer for XpToastRenderer {
    fn bg(&self, _: &ToastRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("surface.raised")
            .unwrap_or(style::hsl_fallback(0xFFFFFF))
    }
    fn fg(&self, _: &ToastRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.primary")
            .unwrap_or(style::hsl_fallback(0x000000))
    }
    fn padding(&self, _: &ToastRenderState, theme: &Theme) -> Edges<Pixels> {
        let p = theme
            .get_number("tokens.control.toast.padding")
            .unwrap_or(10.0) as f32;
        Edges::all(px(p))
    }
    fn border_radius(&self, _: &ToastRenderState, _: &Theme) -> Pixels {
        px(XP_RADIUS)
    }
    fn border(&self, _: &ToastRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
    }
    fn shadow_alpha(&self, _: &ToastRenderState, _: &Theme) -> f32 {
        1.0
    }
}

// =====================================================================
// Notification
// =====================================================================

pub use yororen_ui_core::renderer::notification::{NotificationRenderState, NotificationRenderer};

pub struct XpNotificationRenderer;

impl NotificationRenderer for XpNotificationRenderer {
    fn bg(&self, _: &NotificationRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("surface.raised")
            .unwrap_or(style::hsl_fallback(0xFFFFFF))
    }
    fn border(&self, _: &NotificationRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
    }
    fn padding(&self, _: &NotificationRenderState, theme: &Theme) -> Edges<Pixels> {
        let p = theme
            .get_number("tokens.control.notification.padding")
            .unwrap_or(12.0) as f32;
        Edges::all(px(p))
    }
    fn border_radius(&self, _: &NotificationRenderState, _: &Theme) -> Pixels {
        px(XP_RADIUS)
    }
    fn shadow_alpha(&self, _: &NotificationRenderState, _: &Theme) -> f32 {
        1.0
    }
}
