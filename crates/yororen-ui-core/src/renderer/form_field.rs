//! `FormFieldRenderer` — visual contract for `FormField`.
//!
//! Trait surface is just `compose`. The renderer lays out the
//! label, required indicator, the caller-supplied input element
//! (taken out of `props.input`), and the help / error text — in
//! that vertical order.

use std::any::Any;

use gpui::{App, Div, Stateful};

use crate::headless::form_field::FormFieldProps;

#[derive(Clone, Copy, Debug, Default)]
pub struct FormFieldRenderState {
    pub has_error: bool,
    pub required: bool,
}

pub trait FormFieldRenderer: Any + Send + Sync {
    /// Build the full `Stateful<Div>` for the form field. Takes
    /// `&mut props` so the renderer can move the caller-supplied
    /// input element (`props.input`, an `AnyElement`) into place.
    fn compose(&self, props: &mut FormFieldProps, cx: &App) -> Stateful<Div>;
}
