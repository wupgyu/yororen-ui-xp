//! Section 1 — Actions.
//!
//! Each component is wrapped in a `cell` helper (defined in
//! `sections/mod.rs`) that shows a small `name` label above the
//! component itself so the user can identify what they're
//! looking at.

use gpui::{Context, Div, ParentElement, Styled, div, px};

use yororen_ui::ActionVariantKind;
use yororen_ui::headless::button::button;
use yororen_ui::headless::button_group::button_group;
use yororen_ui::headless::icon::icon;
use yororen_ui::headless::icon_button::icon_button;
use yororen_ui::headless::label::label;
use yororen_ui::headless::split_button::split_button;
use yororen_ui::headless::toggle_button::toggle_button;

use crate::sections::cell;
use crate::state::GalleryApp;

pub fn render(app: &mut GalleryApp, cx: &mut Context<GalleryApp>) -> Div {
    let entity = cx.entity().clone();

    // --- 3 button variants + disabled ---
    let row_buttons = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(px(12.))
        .child(cell("button / Neutral", button("btn-neutral", cx).variant(ActionVariantKind::Neutral).on_click(|_, _, _| {}).render(cx).child("Neutral"), cx))
        .child(cell("button / Primary", button("btn-primary", cx).variant(ActionVariantKind::Primary).on_click(|_, _, _| {}).render(cx).child("Primary"), cx))
        .child(cell("button / Danger", button("btn-danger", cx).variant(ActionVariantKind::Danger).on_click(|_, _, _| {}).render(cx).child("Danger"), cx))
        .child(cell("button / Disabled", button("btn-disabled", cx).disabled(true).on_click(|_, _, _| {}).render(cx).child("Disabled"), cx));

    // --- icon_button with a builtin "check" SVG ---
    let row_icon_button = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(px(12.))
        .child(cell("icon_button (check)", icon_button("icon-btn-check", cx).on_click(|_, _, _| {}).render(cx).child(icon("icon-check-inside", yororen_ui::headless::icon::IconSource::Builtin("check".into()), cx).size(px(16.)).color(gpui::rgb(0xFFFFFF)).render()), cx))
        .child(cell("icon_button / Primary (circle)", icon_button("icon-btn-circle", cx).variant(ActionVariantKind::Primary).on_click(|_, _, _| {}).render(cx).child(icon("icon-circle-inside", yororen_ui::headless::icon::IconSource::Builtin("circle".into()), cx).size(px(16.)).color(gpui::rgb(0xFFFFFF)).render()), cx));

    // --- toggle_button ---
    let entity_for_tb = entity.clone();
    let row_toggle = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.))
        .child(cell("toggle_button", toggle_button("toggle-1", cx).selected(app.toggle_btn_selected).on_toggle(move |_selected, _ev, _window, cx| { entity_for_tb.update(cx, |s, _cx| { s.toggle_btn_selected = !s.toggle_btn_selected; }); }).render(cx).child("Press me"), cx));

    // --- split_button (primary action + secondary on chevron) ---
    let entity_for_split = entity.clone();
    let split = split_button("split-1", move |_ev, _w, cx| { entity_for_split.update(cx, |s, _cx| { s.toast_count += 1; }); }, cx)
        .on_secondary(|_ev, _w, _cx| {})
        .apply(div().flex().flex_row().items_center().gap(px(2.)))
        .child(div().px(px(12.)).py(px(6.)).child(label("split-label", "Save", cx).strong(true).render(cx)))
        .child(div().px(px(8.)).py(px(6.)).border_l_1().border_color(gpui::hsla(0.0, 0.0, 0.0, 0.2)).child(label("split-arrow", "▾", cx).render(cx)));
    let row_split = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.))
        .child(cell("split_button", split, cx));

    // --- button_group ---
    let row_group = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.))
        .child(cell("button_group (3 buttons)", button_group("btn-group-1", cx)
            .apply(div().flex().flex_row().rounded(px(6.)).overflow_hidden().border_1())
            .child(button("bg-left", cx).variant(ActionVariantKind::Neutral).on_click(|_, _, _| {}).apply(div().px(px(12.)).py(px(6.))).child("Left"))
            .child(button("bg-mid", cx).variant(ActionVariantKind::Neutral).on_click(|_, _, _| {}).apply(div().px(px(12.)).py(px(6.)).border_l_1().border_r_1()).child("Mid"))
            .child(button("bg-right", cx).variant(ActionVariantKind::Neutral).on_click(|_, _, _| {}).apply(div().px(px(12.)).py(px(6.))).child("Right")), cx));

    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(row_buttons)
        .child(row_icon_button)
        .child(row_toggle)
        .child(row_split)
        .child(row_group)
}
