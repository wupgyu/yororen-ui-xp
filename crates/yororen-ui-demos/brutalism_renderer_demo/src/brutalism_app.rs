//! `BrutalismApp` — the single-window component the demo
//! renders. Builds a vertical stack of every component that
//! has a `DefaultXxx` sugar trait, so the user can eyeball the
//! style at a glance.
//!
//! Every component on screen is a `headless::Xxx` factory +
//! `.default_render(cx)`. No custom `div()` composition — the
//! point of the demo is that the renderer alone defines the
//! visual vocabulary.

use gpui::{
    Context, IntoElement, ParentElement, Render, Styled, Window, div, px,
};
use yororen_ui::ActionVariantKind;
use yororen_ui::headless::{
    button::button, checkbox::checkbox, heading::{HeadingLevel, heading}, icon_button::icon_button,
    label::label, radio::radio, switch::switch, text_input::text_input, toggle_button::toggle_button,
};
use yororen_ui::renderer::{
    DefaultButton, DefaultCheckbox, DefaultIconButton, DefaultLabel, DefaultRadio,
    DefaultSwitch, DefaultTextInput, DefaultToggleButton,
};

pub struct BrutalismApp;

impl BrutalismApp {
    pub fn new() -> Self {
        Self
    }
}

impl Render for BrutalismApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Build a column of children. Each block scopes its
        // `&mut **cx` borrow so the next one is allowed.
        let mut children: Vec<gpui::AnyElement> = Vec::new();

        // --- Title block ---
        children.push(
            heading("brutalism-title", HeadingLevel::H1, "BRUTALISM", &mut **cx)
                .apply(div())
                .into_any_element(),
        );
        children.push(
            label(
                "brutalism-subtitle",
                "Neo-brutalism — sharp corners, 3px black borders, hard offset shadows, monospace.",
                &mut **cx,
            )
            .default_render(cx)
            .into_any_element(),
        );

        // --- Buttons row (4 variants) ---
        children.push(
            div()
                .flex()
                .gap(px(8.0))
                .child(
                    button("btn-neutral", &mut **cx)
                        .variant(ActionVariantKind::Neutral)
                        .on_click(|_, _, _| {})
                        .default_render(cx)
                        .child("NEUTRAL"),
                )
                .child(
                    button("btn-primary", &mut **cx)
                        .variant(ActionVariantKind::Primary)
                        .on_click(|_, _, _| {})
                        .default_render(cx)
                        .child("PRIMARY"),
                )
                .child(
                    button("btn-danger", &mut **cx)
                        .variant(ActionVariantKind::Danger)
                        .on_click(|_, _, _| {})
                        .default_render(cx)
                        .child("DANGER"),
                )
                .child(
                    button("btn-disabled", &mut **cx)
                        .variant(ActionVariantKind::Primary)
                        .disabled(true)
                        .on_click(|_, _, _| {})
                        .default_render(cx)
                        .child("DISABLED"),
                )
                .into_any_element(),
        );

        // --- IconButton + ToggleButton row ---
        children.push(
            div()
                .flex()
                .gap(px(8.0))
                .items_center()
                .child(
                    icon_button("ibtn-1", &mut **cx)
                        .on_click(|_, _, _| {})
                        .default_render(cx)
                        .child("◆"),
                )
                .child(
                    toggle_button("tbtn-1", &mut **cx)
                        .selected(true)
                        .on_toggle(|_, _, _, _| {})
                        .default_render(cx)
                        .child("TOGGLED ON"),
                )
                .child(
                    toggle_button("tbtn-2", &mut **cx)
                        .selected(false)
                        .on_toggle(|_, _, _, _| {})
                        .default_render(cx)
                        .child("TOGGLED OFF"),
                )
                .into_any_element(),
        );

        // --- Form controls row ---
        children.push(
            div()
                .flex()
                .gap(px(16.0))
                .items_center()
                .child(
                    switch("sw-on", &mut **cx)
                        .checked(true)
                        .on_toggle(|_, _, _, _| {})
                        .default_render(cx)
                        .child("ON"),
                )
                .child(
                    switch("sw-off", &mut **cx)
                        .checked(false)
                        .on_toggle(|_, _, _, _| {})
                        .default_render(cx)
                        .child("OFF"),
                )
                .child(
                    checkbox("cb-1", &mut **cx)
                        .checked(true)
                        .on_toggle(|_, _, _, _| {})
                        .default_render(cx)
                        .child("CHECKED"),
                )
                .child(
                    radio("rd-1", &mut **cx)
                        .checked(true)
                        .on_toggle(|_, _, _, _| {})
                        .default_render(cx)
                        .child("RADIO"),
                )
                .into_any_element(),
        );

        // --- TextInput ---
        children.push(
            text_input("ti-1")
                .placeholder("TYPE HERE…")
                .on_change(|_, _, _| {})
                .default_render(&mut **cx, _window)
                .into_any_element(),
        );

        div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .p(px(24.0))
            .children(children)
    }
}
