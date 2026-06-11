//! `TokenOverlayRenderer` — default `OverlayRenderer` impl.
//!
//! Paints a *relative* scrim. The headless `OverlayProps::render(cx)`
//! layers dismissal handlers and visibility on top.
//!
//! `compose` returns a `relative().size_full()` div so the
//! overlay stays inside its parent's box (no `absolute().inset_0()`
//! that would escape to the document root). Visibility is
//! driven by `props.open` — when `false`, the scrim is
//! `invisible()` so the cell still shows the content underneath.

use std::sync::Arc;

use gpui::{App, Div, Hsla, InteractiveElement, Stateful, Styled, div};
use gpui::prelude::FluentBuilder;

use yororen_ui_core::headless::overlay::OverlayProps;
use yororen_ui_core::theme::{ActiveTheme, Theme};

pub use yororen_ui_core::renderer::overlay::{OverlayRenderState, OverlayRenderer};

pub struct TokenOverlayRenderer;

impl TokenOverlayRenderer {
    pub fn scrim_color(&self, _state: &OverlayRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("surface.scrim")
            .unwrap_or_else(|| gpui::hsla(0.0, 0.0, 0.0, 0.5))
    }
}

impl OverlayRenderer for TokenOverlayRenderer {
    fn compose(&self, props: &OverlayProps, cx: &App) -> Stateful<Div> {
        let theme = cx.theme();
        let state = OverlayRenderState { open: props.open };
        let scrim = self.scrim_color(&state, theme);

        div()
            .id(props.id.clone())
            .relative()
            .size_full()
            .bg(scrim)
            .when(!props.open, |el| el.invisible())
    }
}

pub fn arc_overlay<T: OverlayRenderer + 'static>(r: T) -> Arc<dyn OverlayRenderer> {
    Arc::new(r)
}
