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

pub fn titlebar_from() -> Hsla {
    hsl(0x0058E6)
}
pub fn titlebar_to() -> Hsla {
    hsl(0x3A93FF)
}

/// Button face gradient: near-white top, cream bottom.
pub fn button_face_from() -> Hsla {
    hsl(0xFFFFFF)
}
pub fn button_face_to() -> Hsla {
    hsl(0xE3DFD0)
}
/// Neutral button edge — XP uses a soft olive-gray, not the dark
/// blue reserved for the default (focused) button.
pub fn button_border() -> Hsla {
    hsl(0x7F7B6B)
}
pub fn button_default_border() -> Hsla {
    hsl(0x003C74)
}
pub fn button_hover_ring() -> Hsla {
    hsl(0xC1D2EE)
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
    hsl(0xACA899)
}
pub fn progress_chunk_from() -> Hsla {
    hsl(0xB5E388)
}
pub fn progress_chunk_to() -> Hsla {
    hsl(0x6DAF1B)
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
    hsl(0x0058E6)
}

/// Menu / list selection & hover highlight.
pub fn selection_bg() -> Hsla {
    hsl(0x316AC5)
}
pub fn selection_fg() -> Hsla {
    hsl(0xFFFFFF)
}
pub fn selection_hover_bg() -> Hsla {
    hsl(0xC1D2EE)
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

/// Horizontal (left → right) two-stop gradient — the XP title-bar
/// direction.
pub fn hgrad(from: Hsla, to: Hsla) -> Background {
    linear_gradient(
        90.0,
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

/// XP title-bar blue, horizontal left → right.
pub fn titlebar_gradient(theme: &Theme) -> Background {
    hgrad(
        xp_color(theme, "xp.titlebar.from", titlebar_from()),
        xp_color(theme, "xp.titlebar.to", titlebar_to()),
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
