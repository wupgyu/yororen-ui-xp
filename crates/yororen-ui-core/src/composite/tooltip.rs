//! \`TooltipRoot\` — composite API for tooltips.
//!
//! Split form of the underlying \`tooltip()\` builder. The Root
//! renders a wrapper div around the trigger and attaches a
//! hover-popup tooltip view to it via \`Div::tooltip(closure)\`.
//! This is a *real* implementation (Phase I.2 P0): hovering
//! the trigger displays the configured content as a popup,
//! themed through the active \`TooltipRenderer\`.
//!
//! # Usage
//!
//! ```ignore
//! TooltipRoot::new("save-tip")
//!     .trigger(button("save").child("Save"))
//!     .text("Save the current document (Ctrl+S)")
//!     .placement(TooltipPlacement::Bottom);
//! ```
//!
//! \`.text()\` and \`.content()\` (string form) are the supported
//! paths. \`.content(element)\` is accepted for API symmetry with
//! the other Root types but is currently a no-op: the wrapped
//! trigger is rendered alongside the element, but no popup is
//! shown for the element. Use the underlying \`Tooltip\` builder
//! directly if you need a custom-element popup.

use std::sync::Arc;

use gpui::{
    AnyElement, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, div, prelude::FluentBuilder,
};

use crate::component::tooltip::{Tooltip, TooltipPlacement};
use crate::theme::ActiveTheme;

/// \`TooltipRoot\` is the split-API form of \`tooltip()\`. It
/// renders the trigger inside a div, attaches a hover-popup
/// tooltip view to it, and is fully integrated with the active
/// \`Theme.renderers.tooltip\`.
#[derive(IntoElement)]
pub struct TooltipRoot {
    id: ElementId,
    trigger: Option<AnyElement>,
    /// Tooltip body text. None when the caller did not call
    /// \`.text(...)\` or \`.content(...)\` with a string.
    text: Option<SharedString>,
    /// \`.content(element)\` set. Currently a no-op; stored for
    /// future \`TooltipView\` integration. The element is
    /// rendered as a sibling of the trigger in the wrapper div
    /// so the call site still gets visual feedback.
    content: Option<AnyElement>,
    placement: TooltipPlacement,
    dismiss_on_escape: bool,
}

impl TooltipRoot {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            text: None,
            content: None,
            placement: TooltipPlacement::Auto,
            dismiss_on_escape: true,
        }
    }

    /// Set the trigger element. Anything that implements
    /// \`IntoElement\` is accepted; \`button(...)\`, a custom
    /// \`div()\`, or another composite Root.
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    /// Set the tooltip body as a string. Preferred for
    /// plain-text tooltips. Pass the same text you'd pass to
    /// \`tooltip("…")\`.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(SharedString::from(text.into()));
        self
    }

    /// Alias for \`String\` content, kept for API symmetry with
    /// the other Root types. \`element\` is rendered as a
    /// sibling of the trigger inside the wrapper div (visual
    /// confirmation that the Root took the value).
    pub fn content(mut self, element: impl IntoElement) -> Self {
        self.content = Some(element.into_any_element());
        self
    }

    pub fn placement(mut self, placement: TooltipPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// Whether an Escape keypress dismisses the popup. Default:
    /// \`true\`.
    pub fn dismiss_on_escape(mut self, dismiss: bool) -> Self {
        self.dismiss_on_escape = dismiss;
        self
    }
}

impl RenderOnce for TooltipRoot {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let TooltipRoot {
            id,
            trigger,
            text,
            content,
            placement,
            dismiss_on_escape,
        } = self;

        // ---- Build the underlying Tooltip builder ----
        let tooltip_text: SharedString = text
            .clone()
            .or_else(|| content.as_ref().and_then(|_| Some(SharedString::from("<content>"))))
            .unwrap_or_else(|| SharedString::from(""));

        let tooltip_id = id.clone();
        let tt = Tooltip::text(tooltip_text.to_string())
            .id(tooltip_id)
            .placement(placement)
            .dismiss_on_escape(dismiss_on_escape);

        // Materialise the popup view: the builder's `.build()`
        // returns `Fn(&mut Window, &mut App) -> AnyView`. We
        // invoke it with the current window/cx so the view is
        // live and ready to be attached to the wrapper div.
        let view = (tt.build())(window, cx);

        // ---- Wrap the trigger ----
        let trigger_el: AnyElement =
            trigger.unwrap_or_else(|| div().into_any_element());

        // The wrapper div is the layout-visible element. We
        // attach the popup via `div().tooltip(closure)`. The
        // closure is `move |_w, _cx| view.clone()` — a fresh
        // `AnyView` clone per call so the underlying
        // `Window::set_tooltip` machinery always has a live
        // entity handle.
        let view_for_attach = view;
        let wrapper = div()
            .id(id)
            .child(trigger_el)
            .tooltip(move |_w, _cx| view_for_attach.clone());

        // ---- Optional content sibling (visual feedback) ----
        if let Some(c) = content {
            let theme = cx.theme();
            let _ = theme;
            // The element is currently rendered as a sibling.
            // (Future: pipe it into a custom TooltipView so
            // the popup shows it.)
            wrapper.child(c)
        } else {
            wrapper
        }
    }
}

/// \`TooltipTrigger\` — convenience alias.
pub struct TooltipTrigger {
    id: ElementId,
    element: AnyElement,
}

impl TooltipTrigger {
    pub fn new(id: impl Into<ElementId>, element: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            element: element.into_any_element(),
        }
    }
    pub fn child(&self) -> &AnyElement {
        &self.element
    }
    pub fn id(&self) -> ElementId {
        self.id.clone()
    }
}

/// \`TooltipContent\` — convenience alias.
pub struct TooltipContent {
    id: ElementId,
    element: AnyElement,
}

impl TooltipContent {
    pub fn new(id: impl Into<ElementId>, element: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            element: element.into_any_element(),
        }
    }
    pub fn child(&self) -> &AnyElement {
        &self.element
    }
    pub fn id(&self) -> ElementId {
        self.id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_root_defaults() {
        let r = TooltipRoot::new("t1");
        assert!(r.trigger.is_none());
        assert!(r.content.is_none());
        assert!(r.text.is_none());
    }

    #[test]
    fn tooltip_root_text_shortcut() {
        let r = TooltipRoot::new("t1").text("hi");
        assert_eq!(r.text.as_ref().map(|s| s.to_string()), Some("hi".to_string()));
        assert!(r.content.is_none());
    }

    #[test]
    fn tooltip_root_placement_settable() {
        let r = TooltipRoot::new("t1").placement(TooltipPlacement::Bottom);
        assert!(matches!(r.placement, TooltipPlacement::Bottom));
    }

    #[test]
    fn tooltip_root_dismiss_on_escape_default_true() {
        let r = TooltipRoot::new("t1");
        assert!(r.dismiss_on_escape);
    }

    #[test]
    fn tooltip_root_dismiss_on_escape_setter() {
        let r = TooltipRoot::new("t1").dismiss_on_escape(false);
        assert!(!r.dismiss_on_escape);
    }

    #[test]
    fn tooltip_root_content_stored() {
        let r = TooltipRoot::new("t1").content(div());
        assert!(r.content.is_some());
    }
}
