//! `VirtualListRenderer` — visual contract for `VirtualList`.
//!
//! Unlike most XxxRenderer traits, `compose` **consumes** the
//! `VirtualListProps` because the row render closure is owned by
//! the props and must be transferred into the inner `gpui::List`
//! element. The renderer is responsible for:
//!
//! 1. Reading theme tokens (bg, border, radius) from
//!    `cx.theme()`.
//! 2. Wrapping the [`VirtualListElement`](crate::headless::virtual_list::VirtualListElement)
//!    in an outer `Div` that gives it an id, theme-derived visual
//!    style, and a bounded size.
//! 3. Returning a `Stateful<Div>` the caller can further style
//!    (typically `.w(...).h(...)`).

use std::any::Any;

use gpui::{px, App, Div, Stateful};

use crate::headless::virtual_list::{RenderRowFn, VirtualListProps};
use gpui::ListAlignment;
use gpui::ListSizingBehavior;
use gpui::Pixels;

/// Pure data view of a virtual list, derived from
/// [`VirtualListProps`](crate::headless::virtual_list::VirtualListProps)
/// at render time. Renderer helpers consume this to look up theme
/// tokens without having to re-derive everything.
///
/// `ListAlignment` doesn't impl `Default`, so we hand-roll the
/// default — the v0.3 default is `Top`, matching the headless
/// factory.
#[derive(Clone, Copy, Debug)]
pub struct VirtualListRenderState {
    pub item_count: usize,
    pub alignment: ListAlignment,
    pub overdraw: Pixels,
    pub sizing_behavior: ListSizingBehavior,
}

impl Default for VirtualListRenderState {
    fn default() -> Self {
        Self {
            item_count: 0,
            alignment: ListAlignment::Top,
            overdraw: px(16.0),
            sizing_behavior: ListSizingBehavior::default(),
        }
    }
}

pub trait VirtualListRenderer: Any + Send + Sync {
    /// Build the full `Stateful<Div>` scrollable container for the
    /// virtual list. The row closure is taken by value (not by
    /// `&mut`) so the renderer can move it into the inner
    /// `gpui::List` element.
    fn compose(
        &self,
        props: VirtualListProps,
        render_row: RenderRowFn,
        cx: &App,
    ) -> Stateful<Div>;
}
