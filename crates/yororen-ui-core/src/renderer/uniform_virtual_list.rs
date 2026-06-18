//! `UniformVirtualListRenderer` — visual contract for
//! `UniformVirtualList`.
//!
//! Mirrors [`VirtualListRenderer`](crate::renderer::virtual_list::VirtualListRenderer)
//! but binds to `gpui::uniform_list` instead of `gpui::list`. The
//! uniform variant is significantly faster for large lists where
//! every row has the same height — gpui measures only the first
//! row and then lays out the rest in a straight line, skipping the
//! taffy layout pass entirely.
//!
//! Like the heterogeneous variant, `compose` **consumes** the
//! `UniformVirtualListProps` because the row render closure is
//! owned by the props and must be transferred into the inner
//! `gpui::uniform_list` element. The renderer is responsible for:
//!
//! 1. Reading theme tokens (bg, border, radius) from `cx.theme()`.
//! 2. Wrapping the inner `gpui::uniform_list` element in an outer
//!    `Div` that gives it a stable id, theme-derived visual style,
//!    and a bounded size.
//! 3. Returning a `Stateful<Div>` the caller can further style
//!    (typically `.w(...).h(...)`).
//!
//! ## Why no `on_visible_range_change`
//!
//! `gpui::UniformList` does not expose a `set_scroll_handler`
//! equivalent — its scroll position lives on
//! `UniformListScrollHandle::base_handle` (a `ScrollHandle`), and
//! visible-range derivation would require the renderer to do
//! offset/item-size arithmetic on every frame. That logic belongs
//! in the data layer, not the render layer, so the uniform variant
//! deliberately omits the callback. Use the heterogeneous
//! [`virtual_list`](crate::headless::virtual_list::virtual_list)
//! variant for that use case.

use std::any::Any;

use gpui::{App, Div, ListSizingBehavior, Stateful};

use crate::headless::virtual_list::{UniformRenderRowFn, UniformVirtualListProps};

/// Pure data view of a uniform virtual list, derived from
/// [`UniformVirtualListProps`](crate::headless::virtual_list::UniformVirtualListProps)
/// at render time. Renderer helpers consume this to look up theme
/// tokens without having to re-derive everything.
#[derive(Clone, Copy, Debug, Default)]
pub struct UniformVirtualListRenderState {
    pub item_count: usize,
    pub sizing_behavior: ListSizingBehavior,
}

pub trait UniformVirtualListRenderer: Any + Send + Sync {
    /// Build the full `Stateful<Div>` scrollable container for the
    /// uniform virtual list. The row closure is taken by value
    /// (not by `&mut`) so the renderer can move it into the inner
    /// `gpui::uniform_list` element — typically wrapped in a
    /// `RefCell` since `uniform_list` takes a `Fn` while our
    /// closure is `FnMut`.
    fn compose(
        &self,
        props: UniformVirtualListProps,
        render_row: UniformRenderRowFn,
        cx: &App,
    ) -> Stateful<Div>;
}
