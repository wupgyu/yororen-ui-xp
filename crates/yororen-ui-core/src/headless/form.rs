//! Headless `form` — owns field values + validation + submit.

use std::collections::HashMap;
use std::sync::Arc;

use gpui::{AnyElement, App, Div, ElementId, InteractiveElement, IntoElement, SharedString, Stateful};

pub type FormSubmitCallback =
    Arc<dyn Fn(HashMap<SharedString, String>, &mut gpui::Window, &mut App) + Send + Sync>;

#[derive(Clone)]
pub struct FormProps {
    pub id: ElementId,
    pub values: HashMap<SharedString, String>,
    pub errors: HashMap<SharedString, String>,
    pub on_submit: Option<FormSubmitCallback>,
    /// Optional submit-button label. When set, callers can obtain
    /// a ready-wired button element via [`FormProps::submit_button`]
    /// and place it as a child of the form (typically at the
    /// bottom). The button fires `on_submit` with a snapshot of
    /// `values`.
    pub submit_label: Option<SharedString>,
}

pub fn form(id: impl Into<ElementId>, _cx: &mut App) -> FormProps {
    FormProps {
        id: id.into(),
        values: HashMap::new(),
        errors: HashMap::new(),
        on_submit: None,
        submit_label: None,
    }
}

impl FormProps {
    pub fn value(mut self, field: impl Into<SharedString>, v: impl Into<String>) -> Self {
        self.values.insert(field.into(), v.into());
        self
    }
    pub fn error(mut self, field: impl Into<SharedString>, e: impl Into<String>) -> Self {
        self.errors.insert(field.into(), e.into());
        self
    }
    pub fn on_submit<F>(mut self, f: F) -> Self
    where
        F: 'static + Send + Sync + Fn(HashMap<SharedString, String>, &mut gpui::Window, &mut App),
    {
        self.on_submit = Some(Arc::new(f));
        self
    }
    /// Configure a submit-button label. Without this, no submit
    /// button is generated and [`FormProps::submit_button`]
    /// returns `None`.
    pub fn submit(mut self, label: impl Into<SharedString>) -> Self {
        self.submit_label = Some(label.into());
        self
    }
    pub fn apply(self, el: Div) -> Stateful<Div> {
        el.id(self.id)
    }

    /// Build a ready-wired submit button. Returns `None` if no
    /// label was set via [`FormProps::submit`].
    ///
    /// The button fires `on_submit` with a clone of the current
    /// `values` map. The caller decides where to place the
    /// returned element — typically as the last child of the
    /// form, after every `form_field`.
    ///
    /// The button itself goes through the registered
    /// `ButtonRenderer` (Primary variant), so its appearance
    /// matches the active theme just like every other button.
    pub fn submit_button(&self, cx: &mut App) -> Option<AnyElement> {
        let label = self.submit_label.clone()?;
        let on_submit = self.on_submit.clone();
        let values = self.values.clone();
        let id: ElementId = format!("{:?}-submit", self.id).into();

        let btn = super::button::button(id, cx)
            .variant(crate::renderer::ActionVariantKind::Primary)
            .caption(label)
            .on_click(move |_, window, cx| {
                if let Some(cb) = on_submit.as_ref() {
                    cb(values.clone(), window, cx);
                }
            });
        Some(btn.render(cx).into_any_element())
    }

    /// Trigger `on_submit` with the current `values` snapshot.
    /// Useful if the caller wants to bind submit to a custom
    /// trigger (Enter key, an external button) instead of the
    /// built-in `submit_button` helper.
    pub fn trigger_submit(&self, window: &mut gpui::Window, cx: &mut App) {
        if let Some(cb) = self.on_submit.as_ref() {
            cb(self.values.clone(), window, cx);
        }
    }

    /// Render the form using the registered `FormRenderer`.
    /// Returns a `Stateful<Div>` with the element id and the
    /// renderer-built gap / column.
    pub fn render(self, cx: &gpui::App) -> Stateful<Div> {
        use crate::renderer::RendererContext;
        use crate::renderer::form::FormRenderer;
        use crate::renderer::markers::Form as FormMarker;

        let r: &Arc<dyn FormRenderer> = cx
            .renderer_arc::<FormMarker, dyn FormRenderer>()
            .expect("FormRenderer registered");
        let div = r.compose(&self, cx);
        self.apply(div)
    }
}
