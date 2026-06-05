//! Mini renderer impls. Each one reads the bare minimum from
//! the theme — typically 0 to 2 fields — and bakes the rest
//! of the visual into Rust code.
//!
//! Note: `ButtonRenderer` was migrated to the v0.3 core `Theme`
//! during W2; the other three renderers still use the
//! `default-renderer::Theme` view populated from JSON. We keep
//! the two `Theme` types in scope to satisfy both.

use std::sync::Arc;

use gpui::{Hsla, Pixels};

use yororen_ui_core::theme::Theme as CoreTheme;
use yororen_ui_default_renderer::renderers::spec::{BorderSpec, Edges, ShadowSpec};
use yororen_ui_default_renderer::renderers::{
    ButtonRenderState, ButtonRenderer, IconButtonRenderState, IconButtonRenderer,
    LabelRenderState, LabelRenderer, ToggleButtonRenderState, ToggleButtonRenderer,
};
use yororen_ui_default_renderer::Theme;

/// The mini palette: just one `Hsla` reused everywhere.
#[derive(Copy, Clone, Debug)]
pub struct MiniPalette {
    pub base: Hsla,
}

impl MiniPalette {
    pub fn from_theme(theme: &Theme) -> Self {
        // The mini theme lives in `theme.action.primary.bg` of
        // the default Theme (which is populated from JSON via
        // `from_json`). We just read it like any other renderer
        // would; nothing magic.
        Self { base: theme.action.primary.bg }
    }
}

fn radius() -> Pixels {
    // Mini bakes geometry into code, so the radius never changes
    // with the theme.
    gpui::px(4.0)
}

fn min_h() -> Pixels {
    gpui::px(32.0)
}

fn pad_x() -> Pixels {
    gpui::px(12.0)
}

fn pad_y() -> Pixels {
    gpui::px(6.0)
}

// =====================================================================
// `ButtonRenderer` — only reads `themeColor`. Padding, radius,
// height, and border are all hard-coded.
// =====================================================================

pub struct MiniButtonRenderer {
    pub base: Hsla,
}

impl ButtonRenderer for MiniButtonRenderer {
    fn bg(&self, _state: &ButtonRenderState, _theme: &CoreTheme) -> Hsla {
        if _state.disabled {
            // A subtle grey, completely independent of the theme.
            gpui::hsla(0.0, 0.0, 0.5, 0.6)
        } else {
            self.base
        }
    }
    fn fg(&self, _state: &ButtonRenderState, _theme: &CoreTheme) -> Hsla {
        gpui::hsla(0.0, 0.0, 1.0, 1.0)
    }
    fn padding(&self, _state: &ButtonRenderState, _theme: &CoreTheme) -> Edges<Pixels> {
        Edges::symmetric(pad_x(), pad_y())
    }
    fn border_radius(&self, _state: &ButtonRenderState, _theme: &CoreTheme) -> Pixels {
        radius()
    }
    fn border(&self, _state: &ButtonRenderState, _theme: &CoreTheme) -> Option<BorderSpec> {
        None
    }
    fn shadow(&self, _state: &ButtonRenderState, _theme: &CoreTheme) -> Option<ShadowSpec> {
        None
    }
    fn min_height(&self, _state: &ButtonRenderState, _theme: &CoreTheme) -> Pixels {
        min_h()
    }
    fn disabled_opacity(&self, _state: &ButtonRenderState, _theme: &CoreTheme) -> f32 {
        1.0
    }
}

// =====================================================================
// `IconButtonRenderer` — reuses the MiniButtonRenderer's bg/fg but
// with a square aspect.
// =====================================================================

pub struct MiniIconButtonRenderer {
    pub base: Hsla,
}

impl IconButtonRenderer for MiniIconButtonRenderer {
    fn bg(&self, _state: &IconButtonRenderState, _theme: &Theme) -> Hsla { self.base }
    fn hover_bg(&self, _state: &IconButtonRenderState, _theme: &Theme) -> Hsla {
        // No hover differentiation in the mini.
        self.base
    }
    fn size(&self, _state: &IconButtonRenderState, _theme: &Theme) -> Pixels { min_h() }
    fn border_radius(&self, _state: &IconButtonRenderState, _theme: &Theme) -> Pixels { radius() }
    fn disabled_opacity(&self, _state: &IconButtonRenderState, _theme: &Theme) -> f32 { 1.0 }
}

// =====================================================================
// `ToggleButtonRenderer` — selected state uses the base color;
// unselected state is "no fill".
// =====================================================================

pub struct MiniToggleButtonRenderer {
    pub base: Hsla,
}

impl ToggleButtonRenderer for MiniToggleButtonRenderer {
    fn bg(&self, state: &ToggleButtonRenderState, _theme: &Theme) -> Hsla {
        if state.selected { self.base } else { gpui::hsla(0.0, 0.0, 0.95, 1.0) }
    }
    fn fg(&self, state: &ToggleButtonRenderState, _theme: &Theme) -> Hsla {
        if state.selected { gpui::hsla(0.0, 0.0, 1.0, 1.0) } else { gpui::hsla(0.0, 0.0, 0.1, 1.0) }
    }
    fn min_height(&self, _state: &ToggleButtonRenderState, _theme: &Theme) -> Pixels { min_h() }
    fn border_radius(&self, _state: &ToggleButtonRenderState, _theme: &Theme) -> Pixels { radius() }
    fn disabled_opacity(&self, _state: &ToggleButtonRenderState, _theme: &Theme) -> f32 { 1.0 }
}

// =====================================================================
// `LabelRenderer` — completely theme-agnostic. Always the same
// text color regardless of muted/strong/mono/inherit_color.
// =====================================================================

pub struct MiniLabelRenderer;

impl LabelRenderer for MiniLabelRenderer {
    fn color(&self, _state: &LabelRenderState, _theme: &Theme) -> Hsla {
        gpui::hsla(0.0, 0.0, 0.1, 1.0)
    }
    fn strong_weight(&self, _state: &LabelRenderState, _theme: &Theme) -> gpui::FontWeight {
        gpui::FontWeight(700.0)
    }
    fn family_mono(&self, _state: &LabelRenderState, _theme: &Theme) -> gpui::SharedString {
        "ui-monospace".into()
    }
}

// Suppress unused-import warning when only some impls are referenced
// (e.g. in the install() entry point).
#[allow(dead_code)]
fn _force_imports(_: Arc<()>) {
    let _ = MiniPalette::from_theme;
}
