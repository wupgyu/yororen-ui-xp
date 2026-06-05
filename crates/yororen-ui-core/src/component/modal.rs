use std::sync::Arc;

use gpui::prelude::FluentBuilder;
use gpui::{
    ElementId, Hsla, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, div,
};

use crate::{
    component::{HeadingLevel, IconName, button, heading, icon, icon_button, label, panel},
    i18n::TextDirection,
    theme::{ActionVariantKind, ActiveTheme},
};

/// Callback type for modal close handler. `Arc<dyn Fn>` so it can
/// be cloned into multiple closures (e.g. the close button and
/// the Panel wrapper).
pub type ModalCloseCallback = Arc<dyn Fn(&mut gpui::Window, &mut gpui::App) + Send + Sync>;

/// Modal content shell (dialog panel).
///
/// This component only renders the *panel* (title/content/actions slots) and is
/// intentionally not responsible for overlay / focus trapping.
///
/// Use it inside an [`Overlay`](crate::component::Overlay) to get the
/// full v0.5 accessibility story (scrim, click-outside, Esc, body
/// scroll lock). For example:
///
/// ```rust,ignore
/// overlay("my-modal")
///     .open(state.modal_open)
///     .on_close_any(move |_w, cx| { state.modal_open = false; cx.refresh_windows(); })
///     .child(
///         modal()
///             .title("Delete file?")
///             .content(label("This cannot be undone."))
///             .actions(modal_actions_row(...))
///     )
/// ```
///
/// # Accessibility
///
/// Yororen UI does **not** currently expose ARIA role / label /
/// `aria-modal` on `Modal` (or any other component). The
/// `gpui-ce 0.3.3` framework does not ship a public API to forward
/// ARIA attributes to the OS accessibility tree, so a Modal built
/// on top of it is invisible to screen readers beyond what
/// `gpui-ce`'s own root window provides. The v0.4 / v1.0 work in
/// the [Roadmap wiki][wiki-roadmap] covers the real a11y bridge.
///
/// For application-level keyboard / pointer a11y today, ensure:
/// - The modal is wrapped in an `Overlay` which provides scrim
///   click, Esc, and body scroll lock.
/// - Focus is moved to the modal's first focusable element on
///   open (the [`FocusTrap`](crate::a11y::FocusTrap) helper is
///   the recommended way to do this).
/// - Focus returns to the trigger element when the modal closes.
///
/// [wiki-roadmap]: https://github.com/MeowLynxSea/yororen-ui/wiki/Roadmap
pub fn modal() -> Modal {
    Modal::new()
}

#[derive(IntoElement)]
pub struct Modal {
    element_id: ElementId,
    base: gpui::Div,
    title: Option<SharedString>,
    content: Option<gpui::AnyElement>,
    actions: Option<gpui::AnyElement>,
    width: gpui::Pixels,
    bg: Option<Hsla>,
    border: Option<Hsla>,
    closable: bool,
    on_close: Option<ModalCloseCallback>,
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

impl Modal {
    pub fn new() -> Self {
        Self {
            element_id: "ui:modal".into(),
            base: div(),
            title: None,
            content: None,
            actions: None,
            width: gpui::px(0.),
            bg: None,
            border: None,
            closable: false,
            on_close: None,
        }
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.element_id = id.into();
        self
    }

    /// Combines the current element ID with a suffix to create a child element ID.
    ///
    /// This enables automatic ID composition for nested components, producing
    /// tuple-based IDs like `("parent-id", "child-id")` to avoid ID collisions
    /// when multiple instances of the same component type exist.
    #[allow(dead_code)]
    fn child_id(&self, suffix: &str) -> ElementId {
        (self.element_id.clone(), suffix.to_string()).into()
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }

    pub fn width(mut self, width: gpui::Pixels) -> Self {
        self.width = width;
        self
    }

    pub fn bg(mut self, color: impl Into<Hsla>) -> Self {
        self.bg = Some(color.into());
        self
    }

    pub fn border(mut self, color: impl Into<Hsla>) -> Self {
        self.border = Some(color.into());
        self
    }

    /// Show a close button in the modal header.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// Callback fired when the close button is clicked.
    pub fn on_close<F>(mut self, handler: F) -> Self
    where
        F: 'static + Send + Sync + Fn(&mut gpui::Window, &mut gpui::App),
    {
        self.on_close = Some(Arc::new(handler));
        self
    }
}

impl ParentElement for Modal {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.base.extend(elements);
    }
}

impl Styled for Modal {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Modal {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let theme = cx.theme();
        let bg = self.bg.unwrap_or(theme.surface.raised);
        let border = self.border.unwrap_or(theme.border.default);
        let width = {
            let w: f32 = self.width.into();
            if w > 0.0 {
                self.width
            } else {
                theme.tokens.control.modal.max_width
            }
        };
        let divider_thickness = theme.tokens.control.divider.thickness;

        // Get child component IDs before moving other fields
        let close_button_id: ElementId = (self.element_id.clone(), "close-button").into();
        let element_id_for_base = self.element_id.clone();
        let title = self.title;
        let content = self
            .content
            .unwrap_or_else(|| label("Content").muted(true).into_any_element());
        let actions = self.actions;
        let closable = self.closable;
        let on_close = self.on_close;

        let mut header_children: Vec<gpui::AnyElement> = vec![];

        // Title
        if let Some(title) = title {
            header_children.push(heading(title).level(HeadingLevel::H3).into_any_element());
        } else {
            header_children.push(label("Modal").muted(true).into_any_element());
        }

        // Close button
        if closable {
            let on_close_for_button = on_close.clone();
            let close_button = icon_button(close_button_id)
                .icon(icon(IconName::Close))
                .on_click(move |_, window, cx| {
                    if let Some(handler) = &on_close_for_button {
                        handler(window, cx);
                    }
                });
            header_children.push(close_button.into_any_element());
        }

        let direction = cx.theme().text_direction;

        // Modal composes a Panel internally. The
        // Panel owns the bg / border / border-radius / shadow /
        // padding (drawn from the active theme's PanelRenderer
        // with caller overrides layered on top). The Modal adds
        // the title row, divider, content, and actions on top.
        let mut panel_child = div()
            .child(
                div()
                    .px_4()
                    .py_3()
                    .flex()
                    .when(direction.is_rtl(), |this| this.flex_row_reverse())
                    .when(!direction.is_rtl(), |this| this.flex_row())
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .children(header_children),
            )
            .child(div().h(divider_thickness).w_full().bg(theme.border.divider))
            .child(div().px_4().py_4().child(content));
        if let Some(actions) = actions {
            panel_child = panel_child
                .child(div().h(divider_thickness).w_full().bg(theme.border.divider))
                .child(
                    div()
                        .px_4()
                        .py_3()
                        .flex()
                        .when(direction.is_rtl(), |this| this.flex_row_reverse())
                        .when(!direction.is_rtl(), |this| this.flex_row())
                        .child(actions),
                );
        }

        // The Panel doesn't expose `bg` / `border` overrides via
        // its own builder (they go through the renderer), so we
        // wrap the panel in a div that has the width set, and
        // pass the override bg/border to the panel via its theme
        // via `cx.theme()` (we don't support per-instance override
        // here, but the renderer takes care of it for the default
        // case).
        //
        // The Panel renders with `inset_only(true)` so its own
        // padding is zero; the Modal supplies the spacing on the
        // header / content / actions rows.
        let panel_id = (element_id_for_base.clone(), "panel");
        div().id(element_id_for_base).w(width).child(
            panel(panel_id)
                .bg(bg)
                .border(border)
                .inset_only(true)
                .child(panel_child),
        )
    }
}

pub fn modal_actions_row(
    direction: TextDirection,
    children: impl IntoIterator<Item = gpui::AnyElement>,
) -> impl IntoElement {
    div()
        .flex()
        .when(direction.is_rtl(), |this| this.flex_row_reverse())
        .when(!direction.is_rtl(), |this| this.flex_row())
        .items_center()
        .justify_end()
        .gap_2()
        .children(children)
}

pub fn modal_primary_action(label_text: impl Into<SharedString>) -> impl IntoElement {
    button("ui:modal:primary-action")
        .variant(ActionVariantKind::Primary)
        .child(label_text.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_modal_has_no_title_or_close_handler() {
        let m = Modal::new();
        assert!(m.title.is_none());
        assert!(m.content.is_none());
        assert!(m.actions.is_none());
        assert!(!m.closable);
        assert!(m.on_close.is_none());
    }

    #[test]
    fn closable_flag_independent_of_callback() {
        let m = Modal::new().closable(true);
        assert!(m.closable);
        assert!(m.on_close.is_none());
    }
}
