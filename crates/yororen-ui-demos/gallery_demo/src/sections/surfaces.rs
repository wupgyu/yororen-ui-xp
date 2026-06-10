//! Section 3 — Surfaces. Each component is wrapped in a
//! labelled `cell` so the user can identify every instance.

use gpui::{Context, Div, ParentElement, Styled, div, hsla, px};

use yororen_ui::headless::avatar::avatar;
use yororen_ui::headless::button::button;
use yororen_ui::headless::card::card;
use yororen_ui::headless::empty_state::empty_state;
use yororen_ui::headless::focus_ring::focus_ring;
use yororen_ui::headless::image::image;
use yororen_ui::headless::image::ImageSource;
use yororen_ui::headless::keybinding_display::keybinding_display;
use yororen_ui::headless::label::label;
use yororen_ui::headless::panel::panel;
use yororen_ui::headless::shortcut_hint::shortcut_hint;

use crate::sections::cell;
use crate::state::GalleryApp;

pub fn render(_app: &mut GalleryApp, cx: &mut Context<GalleryApp>) -> Div {
    // --- avatars ---
    let avatars = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(12.))
        .child(cell("avatar / initials + status", avatar("av-1", cx).initials("AB").size(px(48.)).circle(true).has_status(true).apply(div().flex().items_center().justify_center()), cx))
        .child(cell("avatar / initials + square", avatar("av-2", cx).initials("CD").size(px(48.)).circle(false).apply(div().flex().items_center().justify_center()), cx))
        .child(cell("avatar / name (rendered)", avatar("av-3", cx).name("Jane Doe").size(px(48.)).apply(div().flex().items_center().justify_center()), cx));

    // --- card (interactive) ---
    let card_el = card("card-1", cx)
        .interactive(true)
        .apply(div().p(px(16.)).rounded(px(8.)).border_1().w(px(220.)))
        .child(label("card-title", "Interactive card", cx).strong(true).render(cx))
        .child(label("card-body", "Hover me to see the change.", cx).muted(true).render(cx))
        .child(button("card-btn", cx).on_click(|_, _, _| {}).render(cx).child("Action"));
    let card_wrapped = cell("card / interactive", card_el, cx);

    // --- panel with title ---
    let panel_el = panel("panel-1", cx)
        .title("Panel title")
        .padded(true)
        .apply(div().w(px(280.)).p(px(12.)).rounded(px(8.)).border_1())
        .child(label("panel-body", "Generic content surface.", cx).render(cx));
    let panel_wrapped = cell("panel", panel_el, cx);

    // --- empty_state ---
    let empty = empty_state("es-1", cx)
        .icon("info")
        .title("Nothing here yet")
        .description("When you have items, they will show up here.")
        .apply(div().w(px(280.)).p(px(16.)).rounded(px(8.)).border_1())
        .child(label("es-extra", "Custom child (label)", cx).muted(true).render(cx));
    let empty_wrapped = cell("empty_state", empty, cx);

    // --- focus_ring wrapping a button ---
    let ring_target = button("focus-btn", cx)
        .on_click(|_, _, _| {})
        .render(cx)
        .child("Focusable");
    let ring_focus_handle = cx.focus_handle();
    let ringed = focus_ring("ring-1", &ring_focus_handle, cx)
        .apply(div().child(ring_target));
    let ring_wrapped = cell("focus_ring (wraps button)", ringed, cx);

    // --- image (resource path; the file is not bundled in the
    //     demo, but the headless contract is shown) ---
    let img = image("img-1", ImageSource::Resource("images/sample.png".into()), cx)
        .alt("sample")
        .apply(div().w(px(120.)).h(px(80.)).bg(hsla(0.0, 0.0, 0.85, 1.0)).rounded(px(4.)).border_1());
    let img_wrapped = cell("image (resource path)", img, cx);

    // --- keybinding_display ---
    let kbd_disp = keybinding_display("kbd-1", vec!["Ctrl".to_string(), "S".to_string()], cx)
        .apply(div().flex().flex_row().gap(px(4.)).items_center());
    let kbd_wrapped = cell("keybinding_display", kbd_disp, cx);

    // --- shortcut_hint ---
    let sh = shortcut_hint("sh-1", "Save", vec!["Cmd".to_string(), "S".to_string()], cx)
        .apply(div().flex().flex_row().gap(px(8.)).items_center());
    let sh_wrapped = cell("shortcut_hint", sh, cx);

    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(avatars)
        .child(div().flex().flex_row().flex_wrap().gap(px(12.)).child(card_wrapped).child(panel_wrapped).child(empty_wrapped))
        .child(div().flex().flex_row().flex_wrap().gap(px(12.)).items_center().child(ring_wrapped).child(img_wrapped))
        .child(div().flex().flex_col().gap(px(8.)).child(kbd_wrapped).child(sh_wrapped))
}
