//! Headless `virtual_list` — wraps `gpui::ListState` with a row
//! renderer closure. The caller owns a [`VirtualListController`]
//! (a thin handle over `gpui::ListState`) and threads it into the
//! props via the factory; the renderer produces a `gpui::List`
//! element driven by the closure.
//!
//! ## Compared to v0.2.0
//!
//! v0.2.0 had a `VirtualListHandle { state, controller }` and a
//! `VirtualList` element that wrapped `gpui::list(state, render_row)`.
//! v0.3 keeps the same shape: the caller stores the controller on
//! their view, gets the per-frame state via `controller.state()`,
//! and supplies a `FnMut(usize, &mut Window, &mut App) -> AnyElement`
//! closure that the renderer hands to `gpui::list`.
//!
//! ## Why a controller and not raw `gpui::ListState`?
//!
//! `gpui::ListState::new(count, alignment, overdraw)` is the only
//! constructor and there's no other public API surface to add — so
//! the controller is a 4-line ergonomic wrapper that gives the
//! caller `reset / splice / scroll_to_reveal_item` as `&self`
//! methods (the underlying `ListState` is `Rc<RefCell<…>>`, so
//! `&self` mutates the shared inner state). It also pins the
//! default alignment/overdraw so the caller doesn't have to repeat
//! them at every render call site.
//!
//! ## Render closure ownership
//!
//! The render_row closure is `Box<dyn FnMut + 'static>` and is
//! consumed by `render(cx)` (which delegates to the renderer that
//! hands it to `gpui::list`). The renderer is responsible for
//! wrapping the closure in whatever `RenderOnce` shell it needs
//! (e.g. a `VirtualListElement` in the default renderer) — this
//! module deliberately stops at "data + control" and does not
//! define a `RenderOnce` element, since render primitives are
//! out of scope for the headless layer.

use gpui::{
    App, Div, ElementId, InteractiveElement, ListAlignment, ListSizingBehavior, ListState, Pixels,
    Stateful, Window, px,
};
use std::ops::Range;
/// A `&self` handle over a `gpui::ListState` — the caller stores
/// one of these on their view and uses `reset / splice /
/// scroll_to_reveal_item` to mutate the list across frames.
///
/// Cheap to clone (the inner `ListState` is `Rc<RefCell<…>>`).
#[derive(Clone, Debug)]
pub struct VirtualListController {
    state: ListState,
}

impl VirtualListController {
    /// Mint a controller with the given item count, alignment, and
    /// overdraw (in pixels above and below the visible area).
    pub fn new(item_count: usize, alignment: ListAlignment, overdraw: Pixels) -> Self {
        Self {
            state: ListState::new(item_count, alignment, overdraw),
        }
    }

    /// Mint a controller with `ListAlignment::Top` and a 16-px
    /// overdraw — the typical default for a scrolling list.
    pub fn with_default(item_count: usize) -> Self {
        Self::new(item_count, ListAlignment::Top, px(16.))
    }

    /// Snapshot the inner `gpui::ListState` — pass this to
    /// [`virtual_list`].
    pub fn state(&self) -> ListState {
        self.state.clone()
    }

    /// Inform the list that the item count has changed to
    /// `element_count` (used after adding/removing items in bulk).
    pub fn reset(&self, element_count: usize) {
        self.state.reset(element_count);
    }

    /// Inform the list that the items in `old_range` have been
    /// replaced by `count` new items.
    pub fn splice(&self, old_range: Range<usize>, count: usize) {
        self.state.splice(old_range, count);
    }

    /// Scroll the list so that item `ix` is fully visible.
    pub fn scroll_to_reveal_item(&self, ix: usize) {
        self.state.scroll_to_reveal_item(ix);
    }
}

/// Type of the per-item render closure. Called by `gpui::list` for
/// each visible row, with the item index and the gpui context.
pub type RenderRowFn =
    Box<dyn FnMut(usize, &mut Window, &mut App) -> gpui::AnyElement + 'static>;

/// A snapshot of the data + control layer for a virtual list.
/// Constructed by [`virtual_list`], mutated through builder methods,
/// and consumed by `.render(cx)` (which hands it to the
/// `VirtualListRenderer`).
pub struct VirtualListProps {
    pub id: ElementId,
    pub item_count: usize,
    pub alignment: ListAlignment,
    pub overdraw: Pixels,
    pub sizing_behavior: ListSizingBehavior,
    pub state: ListState,
    pub render_row: Option<RenderRowFn>,
}

impl std::fmt::Debug for VirtualListProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VirtualListProps")
            .field("id", &self.id)
            .field("item_count", &self.item_count)
            .field("alignment", &self.alignment)
            .field("overdraw", &self.overdraw)
            .field("sizing_behavior", &self.sizing_behavior)
            .field("render_row", &"<fn>")
            .finish()
    }
}

/// Build a headless `VirtualListProps` for the given `id`, driven by
/// the caller's [`VirtualListController`].
///
/// The returned props need at least `.row(closure)` before
/// `.render(cx)` is meaningful — without a row closure the list has
/// nothing to draw.
pub fn virtual_list(
    id: impl Into<ElementId>,
    controller: &VirtualListController,
    _cx: &mut App,
) -> VirtualListProps {
    VirtualListProps {
        id: id.into(),
        item_count: controller.state.item_count(),
        alignment: ListAlignment::Top,
        overdraw: px(16.),
        sizing_behavior: ListSizingBehavior::default(),
        state: controller.state(),
        render_row: None,
    }
}

impl VirtualListProps {
    /// Update the item count (without touching the controller's
    /// inner state — the caller should also call
    /// `controller.reset(n)` so the ListState stays in sync).
    pub fn item_count(mut self, n: usize) -> Self {
        self.item_count = n;
        self
    }

    /// Top vs bottom alignment — see `gpui::ListAlignment`.
    pub fn alignment(mut self, a: ListAlignment) -> Self {
        self.alignment = a;
        self
    }

    /// Overdraw in pixels — extra space rendered above and below
    /// the visible area to smooth out scrolling.
    pub fn overdraw(mut self, px: Pixels) -> Self {
        self.overdraw = px;
        self
    }

    /// Sizing behavior for layout. `Infer` makes the list adopt the
    /// height of its tallest item; `Auto` (default) lets the parent
    /// drive the size.
    pub fn sizing(mut self, s: ListSizingBehavior) -> Self {
        self.sizing_behavior = s;
        self
    }

    /// Provide the closure that produces each visible row.
    pub fn row(mut self, f: impl FnMut(usize, &mut Window, &mut App) -> gpui::AnyElement + 'static) -> Self {
        self.render_row = Some(Box::new(f));
        self
    }

    /// `apply` for callers that want a custom render path. The
    /// virtual list is closure-driven, so `apply` is vestigial —
    /// it just sets the id and lets the caller provide their own
    /// visual.
    pub fn apply(self, el: Div) -> Stateful<Div> {
        el.id(self.id)
    }

    /// Render through the registered `VirtualListRenderer`. The
    /// props are consumed so the row closure can be transferred
    /// into the renderer's `gpui::list` element.
    pub fn render(mut self, cx: &gpui::App) -> Stateful<Div> {
        use crate::renderer::RendererContext;
        use crate::renderer::markers::VirtualList as VirtualListMarker;
        use crate::renderer::virtual_list::VirtualListRenderer;
        let r: &std::sync::Arc<dyn VirtualListRenderer> = cx
            .renderer_arc::<VirtualListMarker, dyn VirtualListRenderer>()
            .expect("VirtualListRenderer registered");
        // Pull the closure out so we can hand it (and only it) to
        // the renderer; the renderer decides what `RenderOnce`
        // shell to wrap it in.
        let render_row = self
            .render_row
            .take()
            .expect("VirtualListProps::render requires .row(closure)");
        r.compose(self, render_row, cx)
    }
}
