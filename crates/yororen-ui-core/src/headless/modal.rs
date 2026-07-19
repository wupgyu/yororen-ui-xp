//! Headless `modal` — owns `open` + focus-trap config + close
//! reasons. The renderer draws a scrim, a centered dialog, and a
//! focus trap, and routes `Escape` / scrim-click to `on_close`.

use std::sync::Arc;

use gpui::{
    AnyElement, App, AppContext, Div, ElementId, Entity, FocusHandle, InteractiveElement,
    IntoElement, Stateful,
};

use crate::animation::{AnimatedPresenceState, AnimatedVisibility};

/// Reason a modal was closed. Forwarded to the caller's
/// `on_close` so it can branch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModalCloseReason {
    Escape,
    ScrimClick,
    Programmatic,
}

pub type ModalCloseCallback =
    Arc<dyn Fn(ModalCloseReason, &mut gpui::Window, &mut App) + Send + Sync>;

/// Callback for a modal caption (title-bar) button.
pub type ModalCaptionCallback = Arc<dyn Fn(&mut gpui::Window, &mut App) + Send + Sync>;

/// Optional caption-button configuration for renderers that draw
/// window-style title bars (e.g. the XP renderer). Each `Some`
/// entry paints the corresponding button and wires its callback;
/// `None` hides the button. Renderers without caption support
/// ignore this entirely.
#[derive(Clone, Default)]
pub struct ModalCaption {
    pub on_minimize: Option<ModalCaptionCallback>,
    pub on_maximize: Option<ModalCaptionCallback>,
    pub on_close: Option<ModalCaptionCallback>,
}

impl ModalCaption {
    pub fn new() -> Self {
        Self::default()
    }
    /// Paint a minimize button wired to `f`.
    pub fn on_minimize<F>(mut self, f: F) -> Self
    where
        F: 'static + Send + Sync + Fn(&mut gpui::Window, &mut App),
    {
        self.on_minimize = Some(Arc::new(f));
        self
    }
    /// Paint a maximize / restore button wired to `f`.
    pub fn on_maximize<F>(mut self, f: F) -> Self
    where
        F: 'static + Send + Sync + Fn(&mut gpui::Window, &mut App),
    {
        self.on_maximize = Some(Arc::new(f));
        self
    }
    /// Paint a close button wired to `f`. Independent of
    /// `ModalState::set_on_close`; callers usually mirror the same
    /// close path in both.
    pub fn on_close<F>(mut self, f: F) -> Self
    where
        F: 'static + Send + Sync + Fn(&mut gpui::Window, &mut App),
    {
        self.on_close = Some(Arc::new(f));
        self
    }
}

#[derive(Clone)]
pub struct ModalState {
    pub open: bool,
    pub animation: AnimatedVisibility,
    pub dismiss_on_escape: bool,
    pub dismiss_on_scrim: bool,
    /// Focus handle for the *initial* focus when the modal opens.
    /// The renderer traps focus inside the dialog.
    pub initial_focus: Option<FocusHandle>,
    /// Optional label for the scrim (read by `aria-modal` etc.).
    pub title: Option<String>,
    on_close: Option<ModalCloseCallback>,
}

impl ModalState {
    pub fn new(app: &mut App) -> Entity<Self> {
        app.new(|_| Self {
            open: false,
            animation: AnimatedVisibility::new(),
            dismiss_on_escape: true,
            dismiss_on_scrim: true,
            initial_focus: None,
            title: None,
            on_close: None,
        })
    }

    pub fn open(&mut self) {
        self.open = true;
        self.animation.show();
    }
    pub fn close(&mut self) {
        self.open = false;
        self.animation.hide();
    }
    pub fn is_open(&self) -> bool {
        self.open
    }
    pub fn is_visible(&self) -> bool {
        self.animation.is_visible()
    }
    pub fn set_dismiss_on_escape(&mut self, v: bool) {
        self.dismiss_on_escape = v;
    }
    pub fn set_dismiss_on_scrim(&mut self, v: bool) {
        self.dismiss_on_scrim = v;
    }
    pub fn set_initial_focus(&mut self, h: FocusHandle) {
        self.initial_focus = Some(h);
    }
    pub fn set_title(&mut self, t: impl Into<String>) {
        self.title = Some(t.into());
    }
    pub fn set_on_close<F>(&mut self, f: F)
    where
        F: 'static + Send + Sync + Fn(ModalCloseReason, &mut gpui::Window, &mut App),
    {
        self.on_close = Some(Arc::new(f));
    }
    pub fn invoke_close(&self, reason: ModalCloseReason, window: &mut gpui::Window, cx: &mut App) {
        if let Some(f) = &self.on_close {
            f(reason, window, cx);
        }
    }
}

impl AnimatedPresenceState for ModalState {
    fn visibility(&self) -> &AnimatedVisibility {
        &self.animation
    }
    fn visibility_mut(&mut self) -> &mut AnimatedVisibility {
        &mut self.animation
    }
}

pub struct ModalProps {
    pub id: ElementId,
    pub state: Entity<ModalState>,
    /// Children to render inside the modal panel. The renderer
    /// consumes these when `compose` is called.
    pub children: Vec<AnyElement>,
    /// Optional window-style caption buttons (min / max / close).
    /// Renderers that don't draw a title bar ignore this.
    pub caption: Option<ModalCaption>,
    /// Whether the window paints as active (focused): renderers
    /// with active / inactive window chrome use it to pick the
    /// title-bar gradient and frame color. Defaults to `true`.
    pub window_active: bool,
}

pub fn modal(id: impl Into<ElementId>, state: Entity<ModalState>) -> ModalProps {
    ModalProps {
        id: id.into(),
        state,
        children: Vec::new(),
        caption: None,
        window_active: true,
    }
}

impl ModalProps {
    /// Add a child element inside the modal panel.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_element().into_any_element());
        self
    }

    /// Add multiple children inside the modal panel.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        for child in children {
            self.children.push(child.into_element().into_any_element());
        }
        self
    }

    /// Attach window-style caption buttons (min / max / close) to
    /// the title bar. Renderers without caption support ignore
    /// this.
    pub fn caption(mut self, caption: ModalCaption) -> Self {
        self.caption = Some(caption);
        self
    }

    /// Set whether the window paints as active (focused).
    /// Defaults to `true`.
    pub fn window_active(mut self, active: bool) -> Self {
        self.window_active = active;
        self
    }

    pub fn apply(self, el: Div) -> Stateful<Div> {
        el.id(self.id)
    }

    /// Render the modal using the registered `ModalRenderer`.
    /// Returns a `Stateful<Div>` with the element id. The renderer
    /// decides scrim / panel bg / border / padding based on
    /// the `state` entity.
    pub fn render(mut self, cx: &gpui::App) -> Stateful<Div> {
        use crate::renderer::RendererContext;
        use crate::renderer::markers::Modal as ModalMarker;
        use crate::renderer::modal::ModalRenderer;

        let r: &Arc<dyn ModalRenderer> = cx
            .renderer_arc::<ModalMarker, dyn ModalRenderer>()
            .expect("ModalRenderer registered");
        let div = r.compose(&mut self, cx);
        self.apply(div)
    }
}
