//! Headless `text` — a typed text span. No state.

use gpui::{Div, ElementId, InteractiveElement, ParentElement, SharedString, Stateful, Styled};

#[derive(Clone, Debug)]
pub struct TextProps {
    pub id: ElementId,
    pub text: SharedString,
    pub size: Option<gpui::Pixels>,
}

pub fn text(
    id: impl Into<ElementId>,
    text: impl Into<SharedString>,
    _cx: &mut gpui::App,
) -> TextProps {
    TextProps {
        id: id.into(),
        text: text.into(),
        size: None,
    }
}

impl TextProps {
    pub fn size(mut self, s: impl Into<gpui::Pixels>) -> Self {
        self.size = Some(s.into());
        self
    }
    pub fn apply(self, el: Div) -> Stateful<Div> {
        el.id(self.id)
    }

    /// Render the text span as a `Stateful<Div>` with the element
    /// id, the text as a child, and the optional `size` applied.
    /// Use this when you don't need to layer additional styles on
    /// top of the renderer output — the same shape as
    /// `IconProps::render` / `BadgeProps::render`.
    pub fn render(self) -> Stateful<Div> {
        let mut el = gpui::div().id(self.id).child(self.text.clone());
        if let Some(size) = self.size {
            el = el.text_size(size);
        }
        el
    }
}
