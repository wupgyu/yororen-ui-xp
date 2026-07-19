//! Shared constants and helpers for the Windows XP (Luna) renderers.
//!
//! Every `XxxRenderer` implementation in this crate reads from here
//! for the recurring XP values (Luna palette, gradients, bevel
//! colors, progress segments, soft shadows) so the 55 renderers
//! stay in stylistic lockstep.
//!
//! Colors are read from the open theme (`xp.*` extension paths)
//! with these constants as fallbacks; geometry lives here and in
//! `tokens.control.*` so the whole look can be re-tuned from JSON.

use gpui::{Background, BoxShadow, Hsla, linear_color_stop, linear_gradient, point, px, rgb};
use yororen_ui_core::renderer::spec::ShadowSpec;
use yororen_ui_core::theme::Theme;

pub const XP_FONT_FAMILY: &str = "Tahoma, 'Segoe UI', sans-serif";
pub const XP_DISABLED_OPACITY: f32 = 0.6;
pub const XP_RADIUS: f32 = 3.0;
pub const XP_BORDER_WIDTH: f32 = 1.0;

/// Progress bar chunk geometry (Luna green segments).
pub const XP_PROGRESS_SEGMENT_W: f32 = 10.0;
pub const XP_PROGRESS_SEGMENT_GAP: f32 = 2.0;

fn hsl(hex: u32) -> Hsla {
    rgb(hex).into()
}

/// Hex (`0xRRGGBB`) → `Hsla`, for renderer-local fallbacks.
pub fn hsl_fallback(hex: u32) -> Hsla {
    hsl(hex)
}

/// Lighten a color toward white by `amount` (lightness bump).
pub fn lighten(c: Hsla, amount: f32) -> Hsla {
    Hsla {
        l: (c.l + amount).clamp(0.0, 1.0),
        ..c
    }
}

/// Darken a color away from white by `amount`.
pub fn darken(c: Hsla, amount: f32) -> Hsla {
    Hsla {
        l: (c.l - amount).clamp(0.0, 1.0),
        ..c
    }
}

/// `xp.*` color lookup with fallback.
pub fn xp_color(theme: &Theme, path: &str, fallback: Hsla) -> Hsla {
    theme.get_color(path).unwrap_or(fallback)
}

/// `xp.*` number lookup with fallback.
pub fn xp_number(theme: &Theme, path: &str, fallback: f64) -> f32 {
    theme.get_number(path).unwrap_or(fallback) as f32
}

// =====================================================================
// Luna palette (fallbacks — overridden by `xp.*` theme paths)
// =====================================================================

/// Classic XP dialog beige (`#ECE9D8`).
pub fn dialog_bg() -> Hsla {
    hsl(0xECE9D8)
}

/// Luna title-bar gradient stops (CSS `#0997FF → #0053EE →
/// #0050EE → #0066FF → #0058EB`).
pub fn titlebar_from() -> Hsla {
    hsl(0x0997FF)
}
pub fn titlebar_mid_1() -> Hsla {
    hsl(0x0053EE)
}
pub fn titlebar_mid_2() -> Hsla {
    hsl(0x0050EE)
}
pub fn titlebar_mid_3() -> Hsla {
    hsl(0x0066FF)
}
pub fn titlebar_to() -> Hsla {
    hsl(0x0058EB)
}
/// Inactive window title bar (`#B8C4DC → #98A8C0`).
pub fn titlebar_inactive_from() -> Hsla {
    hsl(0xB8C4DC)
}
pub fn titlebar_inactive_to() -> Hsla {
    hsl(0x98A8C0)
}

/// Button face gradient stops: white top, beige middle
/// (`#ECE9D8` at 45%), cream bottom (`#DDD8C8`).
pub fn button_face_from() -> Hsla {
    hsl(0xFFFFFF)
}
pub fn button_face_mid() -> Hsla {
    hsl(0xECE9D8)
}
pub fn button_face_to() -> Hsla {
    hsl(0xDDD8C8)
}
/// Neutral button edge — the same dark blue as the default
/// (focused) button, matching the CSS reference (`#003C74`).
pub fn button_border() -> Hsla {
    hsl(0x003C74)
}
pub fn button_default_border() -> Hsla {
    hsl(0x003C74)
}
/// Hover inset ring (`#FFCF31` orange).
pub fn button_hover_ring() -> Hsla {
    hsl(0xFFCF31)
}
pub fn primary_from() -> Hsla {
    hsl(0x3A93FF)
}
pub fn primary_to() -> Hsla {
    hsl(0x0058E6)
}

pub fn progress_track_bg() -> Hsla {
    hsl(0xFFFFFF)
}
pub fn progress_track_border() -> Hsla {
    hsl(0x7F9DB9)
}
pub fn progress_chunk_from() -> Hsla {
    hsl(0x68D868)
}
pub fn progress_chunk_to() -> Hsla {
    hsl(0x189418)
}
pub fn progress_chunk_border() -> Hsla {
    hsl(0x5C9918)
}

/// Win32 bevel edges (raised: light outer / dark inner; sunken:
/// dark outer / light inner).
pub fn bevel_outer_light() -> Hsla {
    hsl(0xFFFFFF)
}
pub fn bevel_inner_light() -> Hsla {
    hsl(0xF1EFE2)
}
pub fn bevel_inner_dark() -> Hsla {
    hsl(0xACA899)
}
pub fn bevel_outer_dark() -> Hsla {
    hsl(0x716F64)
}

pub fn input_bg() -> Hsla {
    hsl(0xFFFFFF)
}
pub fn input_border() -> Hsla {
    hsl(0x7F9DB9)
}
pub fn input_focus_border() -> Hsla {
    hsl(0x316AC5)
}

/// Menu / list selection & hover highlight.
pub fn selection_bg() -> Hsla {
    hsl(0x316AC5)
}
pub fn selection_fg() -> Hsla {
    hsl(0xFFFFFF)
}
/// List item hover (`#CFE0FA`, paler than selection).
pub fn selection_hover_bg() -> Hsla {
    hsl(0xCFE0FA)
}

/// Menu item hover: solid selection blue with white text.
pub fn menu_hover_bg() -> Hsla {
    hsl(0x316AC5)
}
pub fn menu_hover_fg() -> Hsla {
    hsl(0xFFFFFF)
}

/// Balloon toast / notification (`#FFFFE1` + 1px black edge).
pub fn toast_bg() -> Hsla {
    hsl(0xFFFFE1)
}
pub fn toast_border() -> Hsla {
    hsl(0x000000)
}

/// XP window frame, active (`#0058E6`) / inactive (`#98A8C0`).
pub fn window_border_active() -> Hsla {
    hsl(0x0058E6)
}
pub fn window_border_inactive() -> Hsla {
    hsl(0x98A8C0)
}
/// Inner border of the window body (`#A09C8C`), drawn around
/// the content area inside the frame.
pub fn window_body_border() -> Hsla {
    hsl(0xA09C8C)
}

/// Caption (title-bar) button gradients + translucent white edge.
pub fn caption_from() -> Hsla {
    hsl(0x3C8CFD)
}
pub fn caption_to() -> Hsla {
    hsl(0x1565E8)
}
pub fn caption_close_from() -> Hsla {
    hsl(0xF08A6D)
}
pub fn caption_close_to() -> Hsla {
    hsl(0xD84A28)
}
pub fn caption_border() -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.0,
        l: 1.0,
        a: 0.6,
    }
}

/// Checkbox / radio glyph blue.
pub fn check_glyph() -> Hsla {
    hsl(0x0058E6)
}

/// XP tooltip pale yellow (`#FFFFE1`).
pub fn tooltip_bg() -> Hsla {
    hsl(0xFFFFE1)
}

// =====================================================================
// Gradients
// =====================================================================

/// Vertical (top → bottom) two-stop gradient.
pub fn vgrad(from: Hsla, to: Hsla) -> Background {
    linear_gradient(
        180.0,
        linear_color_stop(from, 0.0),
        linear_color_stop(to, 1.0),
    )
}

/// Button face: white → cream vertical gradient.
pub fn button_face(theme: &Theme) -> Background {
    vgrad(
        xp_color(theme, "xp.button.face_from", button_face_from()),
        xp_color(theme, "xp.button.face_to", button_face_to()),
    )
}

/// 3-stop button face colors (top / mid / bottom) for the banded
/// button face; pressed state plays them in reverse.
pub fn button_face_stops(theme: &Theme) -> (Hsla, Hsla, Hsla) {
    (
        xp_color(theme, "xp.button.face_from", button_face_from()),
        xp_color(theme, "xp.button.face_mid", button_face_mid()),
        xp_color(theme, "xp.button.face_to", button_face_to()),
    )
}

/// Pressed button face: gradient reversed for the sunken look.
pub fn button_face_pressed(theme: &Theme) -> Background {
    vgrad(
        xp_color(theme, "xp.button.face_to", button_face_to()),
        xp_color(theme, "xp.button.face_from", button_face_from()),
    )
}

/// Primary (default) button face: Luna blue vertical gradient.
pub fn primary_face(theme: &Theme) -> Background {
    vgrad(
        xp_color(theme, "xp.button.primary_from", primary_from()),
        xp_color(theme, "xp.button.primary_to", primary_to()),
    )
}

/// Pressed primary face: blue gradient reversed.
pub fn primary_face_pressed(theme: &Theme) -> Background {
    vgrad(
        xp_color(theme, "xp.button.primary_to", primary_to()),
        xp_color(theme, "xp.button.primary_from", primary_from()),
    )
}

/// Luna title-bar bands: `(height fraction, from, to)` vertical
/// 2-stop slices approximating the 5-stop CSS gradient
/// (`0% / 8% / 40% / 88% / 100%`). Painted as stacked divs.
pub fn titlebar_bands(theme: &Theme) -> [(f32, Hsla, Hsla); 4] {
    [
        (
            0.08,
            xp_color(theme, "xp.titlebar.from", titlebar_from()),
            xp_color(theme, "xp.titlebar.mid_1", titlebar_mid_1()),
        ),
        (
            0.32,
            xp_color(theme, "xp.titlebar.mid_1", titlebar_mid_1()),
            xp_color(theme, "xp.titlebar.mid_2", titlebar_mid_2()),
        ),
        (
            0.48,
            xp_color(theme, "xp.titlebar.mid_2", titlebar_mid_2()),
            xp_color(theme, "xp.titlebar.mid_3", titlebar_mid_3()),
        ),
        (
            0.12,
            xp_color(theme, "xp.titlebar.mid_3", titlebar_mid_3()),
            xp_color(theme, "xp.titlebar.to", titlebar_to()),
        ),
    ]
}

/// Inactive title bar: plain vertical 2-stop gradient.
pub fn titlebar_inactive_gradient(theme: &Theme) -> Background {
    vgrad(
        xp_color(theme, "xp.titlebar.inactive_from", titlebar_inactive_from()),
        xp_color(theme, "xp.titlebar.inactive_to", titlebar_inactive_to()),
    )
}

/// Progress chunk green, vertical.
pub fn progress_chunk_gradient(theme: &Theme) -> Background {
    vgrad(
        xp_color(theme, "xp.progress.chunk_from", progress_chunk_from()),
        xp_color(theme, "xp.progress.chunk_to", progress_chunk_to()),
    )
}

// =====================================================================
// Shadows — XP uses soft blurred shadows, not hard offsets
// =====================================================================

/// Small soft shadow for buttons / raised controls.
pub fn xp_shadow(theme: &Theme) -> ShadowSpec {
    let offset_y = theme.get_number("shadow.default.offset_y").unwrap_or(2.0) as f32;
    let blur = theme.get_number("shadow.default.blur").unwrap_or(4.0) as f32;
    let color = theme
        .get_color("shadow.default.color")
        .unwrap_or(hsl(0x000000).opacity(0.25));
    ShadowSpec {
        blur: px(blur),
        offset_y: px(offset_y),
        color,
    }
}

/// Larger soft shadow for overlays (modals, popovers, menus).
pub fn xp_shadow_overlay(theme: &Theme) -> ShadowSpec {
    let offset_y = theme.get_number("shadow.overlay.offset_y").unwrap_or(4.0) as f32;
    let blur = theme.get_number("shadow.overlay.blur").unwrap_or(12.0) as f32;
    let color = theme
        .get_color("shadow.overlay.color")
        .unwrap_or(hsl(0x000000).opacity(0.4));
    ShadowSpec {
        blur: px(blur),
        offset_y: px(offset_y),
        color,
    }
}

/// `ShadowSpec` → gpui `BoxShadow` list ready for `.shadow(...)`.
pub fn shadow_vec(spec: ShadowSpec) -> Vec<BoxShadow> {
    vec![BoxShadow {
        color: spec.color,
        offset: point(px(0.0), spec.offset_y),
        blur_radius: spec.blur,
        spread_radius: px(0.0),
    }]
}
