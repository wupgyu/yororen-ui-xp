//! Headless `list_item` — a single row in a list. Pure data
//! carrier; visual lives in the renderer. Optional `on_click`
//! wires the a11y responsibility to the headless layer; the
//! renderer doesn't need to know about click semantics.

use std::sync::Arc;

use gpui::{
    ClickEvent, Div, ElementId, InteractiveElement, SharedString, Stateful,
    StatefulInteractiveElement,
};

/// Click handler shared with the other headless primitives
/// (button, tree_item, etc.). Lives in the headless `button`
/// module — re-exported here for ergonomics.
pub use super::button::ClickCallback;

#[derive(Clone)]
pub struct ListItemProps {
    pub id: ElementId,
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub leading_icon: Option<SharedString>,
    pub trailing_icon: Option<SharedString>,
    pub selected: bool,
    pub disabled: bool,
    /// Optional click handler. Wired by both `apply` and `render`
    /// (whichever the caller uses). Skipped when `disabled` is
    /// `true`.
    ///
    /// `Debug` is hand-rolled because the closure type is `!Debug`
    /// — we print `"<fn>"` to keep struct-debug output stable.
    pub on_click: Option<ClickCallback>,
}

impl std::fmt::Debug for ListItemProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListItemProps")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("description", &self.description)
            .field("leading_icon", &self.leading_icon)
            .field("trailing_icon", &self.trailing_icon)
            .field("selected", &self.selected)
            .field("disabled", &self.disabled)
            .field("on_click", &self.on_click.as_ref().map(|_| "<fn>"))
            .finish()
    }
}

pub fn list_item(
    id: impl Into<ElementId>,
    title: impl Into<SharedString>,
    _cx: &mut gpui::App,
) -> ListItemProps {
    ListItemProps {
        id: id.into(),
        title: title.into(),
        description: None,
        leading_icon: None,
        trailing_icon: None,
        selected: false,
        disabled: false,
        on_click: None,
    }
}

impl ListItemProps {
    pub fn description(mut self, d: impl Into<SharedString>) -> Self {
        self.description = Some(d.into());
        self
    }
    pub fn leading_icon(mut self, i: impl Into<SharedString>) -> Self {
        self.leading_icon = Some(i.into());
        self
    }
    pub fn trailing_icon(mut self, i: impl Into<SharedString>) -> Self {
        self.trailing_icon = Some(i.into());
        self
    }
    pub fn selected(mut self, v: bool) -> Self {
        self.selected = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
    /// Wire a click handler. Skipped when the item is `disabled`.
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: 'static + Send + Sync + Fn(&ClickEvent, &mut gpui::Window, &mut gpui::App),
    {
        self.on_click = Some(Arc::new(f));
        self
    }
    /// `apply` is the a11y path: it sets the id and (if
    /// `on_click` was set and the item isn't `disabled`) wires
    /// the click handler. The caller's `el` (the renderer's
    /// composed `Div`) provides the visual.
    ///
    /// The `Div` -> `Stateful<Div>` conversion happens here so
    /// we can use the `StatefulInteractiveElement::on_click`
    /// builder (the inherent `Div::on_click` is `&mut self`
    /// only and isn't chainable).
    pub fn apply(self, el: Div) -> Stateful<Div> {
        let on_click = self.on_click.clone();
        let disabled = self.disabled;
        let mut s: Stateful<Div> = el.id(self.id);
        if !disabled && let Some(f) = on_click {
            s = s.on_click(move |ev, window, cx| {
                if disabled {
                    return;
                }
                f(ev, window, cx);
            });
        }
        s
    }

    /// Render the list item using the registered `ListItemRenderer`.
    /// Returns a `Stateful<Div>` with the element id, the
    /// renderer-built bg / padding / min_h / radius, and (if set)
    /// the click handler.
    pub fn render(self, cx: &gpui::App) -> Stateful<Div> {
        use crate::renderer::RendererContext;
        use crate::renderer::list_item::ListItemRenderer;
        use crate::renderer::markers::ListItem as ListItemMarker;

        let r: &Arc<dyn ListItemRenderer> = cx
            .renderer_arc::<ListItemMarker, dyn ListItemRenderer>()
            .expect("ListItemRenderer registered");
        let div: Div = r.compose(&self, cx);
        // Compose already produced the visual; we just need to
        // hand it through `apply` (which sets the id and may
        // wire `on_click`).
        self.apply(div)
    }
}
