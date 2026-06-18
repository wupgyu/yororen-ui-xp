//! Headless `form_field` — a labelled field slot. The caller
//! supplies the input element via `.input(...)`; the headless
//! layer only owns the label, required flag, help and error
//! strings.

use crate::renderer::RendererContext;
use gpui::{
    AnyElement, App, Div, ElementId, InteractiveElement, IntoElement, SharedString, Stateful,
};

/// A labelled wrapper around a form input. The visual rendering
/// (`[label]`, `[input]`, `[help]`, `[error]` vertical stack)
/// lives in the registered `FormFieldRenderer`; this struct only
/// carries the data.
///
/// `input` holds the caller-supplied input element. It is an
/// `Option<AnyElement>` because (a) it is set lazily via
/// `.input(...)`, and (b) the renderer takes it out of the props
/// during `compose`. The same v0.3 "props own AnyElement" pattern
/// is used by `PopoverProps::{trigger, content}` and
/// `ModalProps::body`.
pub struct FormFieldProps {
    pub id: ElementId,
    pub name: SharedString,
    pub label: Option<SharedString>,
    pub required: bool,
    pub error: Option<SharedString>,
    pub help: Option<SharedString>,
    pub input: Option<AnyElement>,
}

impl std::fmt::Debug for FormFieldProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FormFieldProps")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("label", &self.label)
            .field("required", &self.required)
            .field("error", &self.error)
            .field("help", &self.help)
            .field("input", &self.input.as_ref().map(|_| "<AnyElement>"))
            .finish()
    }
}

pub fn form_field(
    id: impl Into<ElementId>,
    name: impl Into<SharedString>,
    _cx: &mut gpui::App,
) -> FormFieldProps {
    FormFieldProps {
        id: id.into(),
        name: name.into(),
        label: None,
        required: false,
        error: None,
        help: None,
        input: None,
    }
}

impl FormFieldProps {
    pub fn label(mut self, l: impl Into<SharedString>) -> Self {
        self.label = Some(l.into());
        self
    }
    pub fn required(mut self, v: bool) -> Self {
        self.required = v;
        self
    }
    pub fn error(mut self, e: impl Into<SharedString>) -> Self {
        self.error = Some(e.into());
        self
    }
    pub fn help(mut self, h: impl Into<SharedString>) -> Self {
        self.help = Some(h.into());
        self
    }
    /// Stash the caller-supplied input element. The renderer
    /// takes ownership of it during `compose` so it can drop the
    /// element into the right slot in the vertical stack
    /// (between label and help/error). Replaces any previously
    /// stashed input.
    pub fn input(mut self, el: impl IntoElement) -> Self {
        self.input = Some(el.into_any_element());
        self
    }
    pub fn apply(self, el: Div) -> Stateful<Div> {
        el.id(self.id)
    }

    /// Render the form field through the registered
    /// `FormFieldRenderer`. The renderer composes
    /// `[label, input, error, help]` in that order, using
    /// the input element previously supplied via `.input(...)`.
    pub fn render(mut self, cx: &App) -> Stateful<Div> {
        let r = cx
            .renderer_arc::<crate::renderer::markers::FormField, dyn crate::renderer::form_field::FormFieldRenderer>()
            .expect("FormFieldRenderer registered");
        r.compose(&mut self, cx)
    }
}
