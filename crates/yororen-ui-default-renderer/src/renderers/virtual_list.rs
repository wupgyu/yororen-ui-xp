//! `TokenVirtualListRenderer` — default `VirtualListRenderer` impl.
//!
//! Wraps `gpui::list(state, render_row)` in an outer `Div` that
//! gives it a stable id, theme-derived background / border /
//! radius, and `overflow_hidden` so the rounded corners clip
//! the scrolled content.
//!
//! The inner list is forced to `size_full().flex_grow().min_h_0()`
//! so it fills the parent — without this, `gpui::list` collapses
//! to zero height when nested in a flex column.

use std::sync::Arc;

use gpui::{
    App, Div, Hsla, InteractiveElement, ParentElement, Pixels, Stateful, Styled, div, list, px,
};

use yororen_ui_core::headless::virtual_list::{RenderRowFn, VirtualListProps};
use yororen_ui_core::renderer::virtual_list::{VirtualListRenderState, VirtualListRenderer};
use yororen_ui_core::theme::{ActiveTheme, Theme};

pub struct TokenVirtualListRenderer;

impl TokenVirtualListRenderer {
    pub fn bg(&self, _state: &VirtualListRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.base").unwrap_or_default()
    }
    pub fn border_color(&self, _state: &VirtualListRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    pub fn border_radius(&self, _state: &VirtualListRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.radii.sm").unwrap_or(4.0) as f32)
    }
}

impl VirtualListRenderer for TokenVirtualListRenderer {
    fn compose(
        &self,
        props: VirtualListProps,
        render_row: RenderRowFn,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = VirtualListRenderState {
            item_count: props.item_count,
            alignment: props.alignment,
            overdraw: props.overdraw,
            sizing_behavior: props.sizing_behavior,
        };
        let bg = self.bg(&state, theme);
        let border = self.border_color(&state, theme);
        let radius = self.border_radius(&state, theme);

        // Construct the inner list with the caller's state and
        // sizing behavior, then force it to fill the parent —
        // `gpui::list` alone collapses to zero height in a
        // flex column without an explicit size.
        let list_el = list(props.state, render_row).with_sizing_behavior(props.sizing_behavior);
        let inner = list_el.size_full().flex_grow().min_h_0();

        // `gpui::list` handles scroll internally (its own
        // bubble-phase `on_mouse_event` consumes the delta and
        // calls `list_state.scroll(...)`), but it does **not**
        // call `stop_propagation()`. Without an explicit stop
        // here, the wheel event continues to bubble up to the
        // page / outer scrollable container and scrolls *that*
        // in addition to the list — the v0.3 wrapping div
        // introduced this regression (v0.2.0's `VirtualList`
        // was the styled list element itself, so there was no
        // outer hitbox to bubble past). Register a bubble-phase
        // scroll handler on the outer div that stops the event
        // after the list has handled it.
        div()
            .id(props.id)
            .flex()
            .flex_col()
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .overflow_hidden()
            .child(inner)
            .on_scroll_wheel(|_event, _window, cx| {
                cx.stop_propagation();
            })
    }
}

pub fn arc_virtual_list<T: VirtualListRenderer + 'static>(r: T) -> Arc<dyn VirtualListRenderer> {
    Arc::new(r)
}
