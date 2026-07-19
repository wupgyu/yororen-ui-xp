//! XP (Luna) notification renderers: `Toast`, `Notification`.
//!
//! Both follow the XP balloon-help look: `#FFFFE1` pale-yellow
//! background, 1px black border, 6px rounded corners, and the
//! theme's soft overlay shadow at full strength.

use gpui::{Hsla, Pixels, px};
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

use crate::style::{self, toast_bg, toast_border, xp_color};

// =====================================================================
// Toast
// =====================================================================

pub use yororen_ui_core::renderer::toast::{ToastRenderState, ToastRenderer};

pub struct XpToastRenderer;

impl ToastRenderer for XpToastRenderer {
    fn bg(&self, _: &ToastRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.toast.bg", toast_bg())
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
    fn border_radius(&self, _: &ToastRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.toast.radius")
            .unwrap_or(6.0) as f32)
    }
    fn border(&self, _: &ToastRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.toast.border", toast_border())
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
        xp_color(theme, "xp.toast.bg", toast_bg())
    }
    fn border(&self, _: &NotificationRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.toast.border", toast_border())
    }
    fn padding(&self, _: &NotificationRenderState, theme: &Theme) -> Edges<Pixels> {
        let p = theme
            .get_number("tokens.control.notification.padding")
            .unwrap_or(12.0) as f32;
        Edges::all(px(p))
    }
    fn border_radius(&self, _: &NotificationRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.notification.radius")
            .unwrap_or(6.0) as f32)
    }
    fn shadow_alpha(&self, _: &NotificationRenderState, _: &Theme) -> f32 {
        1.0
    }
}
