//! Headless `popover` — owns `open` + placement + a stored trigger
//! and content element. The renderer watches the `Entity<PopoverState>`
//! and lays out the content via `gpui::anchored` when `open` flips.

use std::sync::Arc;

use gpui::{
    App, AppContext, AnyElement, Bounds, Div, ElementId, Entity, InteractiveElement, Pixels, Size,
    Stateful,
};

/// Preferred placement of a popover relative to its trigger.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PopoverPlacement {
    #[default]
    BottomStart,
    BottomEnd,
    TopStart,
    TopEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
}

/// State of a single popover. Mutate `open` to show / hide.
///
/// The `trigger_bounds` and `content_size` fields are written by
/// the renderer during prepaint (the trigger element reports its
/// position; the content element reports its measured size). The
/// renderer then positions the content via `anchored`.
#[derive(Clone)]
pub struct PopoverState {
    pub open: bool,
    pub placement: PopoverPlacement,
    pub width: Option<Pixels>,
    pub dismiss_on_escape: bool,
    pub dismiss_on_outside_click: bool,

    pub trigger_bounds: Option<Bounds<Pixels>>,
    pub content_size: Option<Size<Pixels>>,

    on_close: Option<CloseFn>,
}

pub type CloseFn = Arc<dyn Fn(&mut gpui::Window, &mut App) + Send + Sync>;

impl PopoverState {
    pub fn new(app: &mut App) -> Entity<Self> {
        app.new(|_| Self {
            open: false,
            placement: PopoverPlacement::default(),
            width: None,
            dismiss_on_escape: true,
            dismiss_on_outside_click: true,
            trigger_bounds: None,
            content_size: None,
            on_close: None,
        })
    }

    pub fn open(&mut self) {
        self.open = true;
    }
    pub fn close(&mut self) {
        self.open = false;
    }
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
    pub fn is_open(&self) -> bool {
        self.open
    }
    pub fn set_placement(&mut self, p: PopoverPlacement) {
        self.placement = p;
    }
    pub fn set_width(&mut self, w: Pixels) {
        self.width = Some(w);
    }
    pub fn set_dismiss_on_escape(&mut self, v: bool) {
        self.dismiss_on_escape = v;
    }
    pub fn set_dismiss_on_outside_click(&mut self, v: bool) {
        self.dismiss_on_outside_click = v;
    }
    pub fn set_on_close<F>(&mut self, f: F)
    where
        F: 'static + Send + Sync + Fn(&mut gpui::Window, &mut App),
    {
        self.on_close = Some(Arc::new(f));
    }
    pub fn invoke_close(&self, window: &mut gpui::Window, cx: &mut App) {
        if let Some(f) = &self.on_close {
            f(window, cx);
        }
    }
}

/// The headless popover props handed to `.apply(div)` or the
/// renderer's `DefaultPopover::default_render`.
///
/// `trigger` and `content` are *caller-supplied* elements
/// stored on the props. They are **data** (a description of
/// UI the caller wants shown), not visual decisions — the
/// renderer is what actually lays them out (trigger in
/// normal flow, content floated with `gpui::deferred` +
/// absolute positioning when `state.is_open()`). Holding
/// `AnyElement` here is the v0.3 architectural compromise
/// that lets composite overlays work without changing the
/// `XxxRenderer` signature to accept extra arguments.
pub struct PopoverProps {
    pub id: ElementId,
    pub state: Entity<PopoverState>,
    pub trigger: Option<AnyElement>,
    pub content: Option<AnyElement>,
}

pub fn popover(id: impl Into<ElementId>, state: Entity<PopoverState>) -> PopoverProps {
    PopoverProps {
        id: id.into(),
        state,
        trigger: None,
        content: None,
    }
}

impl PopoverProps {
    /// Set the trigger element. Rendered in the normal layout
    /// flow; the caller's click handler on the trigger is
    /// expected to call `state.toggle()` (the popover's
    /// "open on click" wiring is the caller's responsibility).
    pub fn trigger(mut self, t: AnyElement) -> Self {
        self.trigger = Some(t);
        self
    }

    /// Set the popover content element. Floated next to the
    /// trigger via `gpui::deferred` + absolute positioning
    /// by the registered `PopoverRenderer` when
    /// `state.is_open()` is true.
    pub fn content(mut self, c: AnyElement) -> Self {
        self.content = Some(c);
        self
    }

    pub fn apply(self, el: Div) -> Stateful<Div> {
        el.id(self.id)
    }

    /// Render the popover using the registered `PopoverRenderer`.
    /// Returns a `Stateful<Div>` with the element id. The renderer
    /// decides bg / border / shadow / deferred paint based on the
    /// `state` entity and the supplied `trigger` / `content`.
    pub fn render(mut self, cx: &gpui::App) -> Stateful<Div> {
        use crate::renderer::RendererContext;
        use crate::renderer::popover::PopoverRenderer;
        use crate::renderer::markers::Popover as PopoverMarker;

        let r: &Arc<dyn PopoverRenderer> = cx
            .renderer_arc::<PopoverMarker, dyn PopoverRenderer>()
            .expect("PopoverRenderer registered");
        let div = r.compose(&mut self, cx);
        self.apply(div)
    }
}

/// Re-export `Point` so callers can use it from the headless
/// popover without importing `gpui::Point` directly.
pub use gpui::Point as AnchorPoint;
