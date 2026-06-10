//! Section 5 — Controls. Each component is wrapped in a labelled
//! `cell` so the user can identify every instance.

use gpui::{Context, Div, ParentElement, Styled, div, px};

use yororen_ui::headless::checkbox::checkbox;
use yororen_ui::headless::label::label;
use yororen_ui::headless::radio::radio;
use yororen_ui::headless::radio_group::radio_group;
use yororen_ui::headless::slider::slider;
use yororen_ui::headless::switch::switch;

use crate::sections::cell;
use crate::state::GalleryApp;

pub fn render(app: &mut GalleryApp, cx: &mut Context<GalleryApp>) -> Div {
    let entity = cx.entity().clone();

    // --- checkbox ---
    let entity_cb = entity.clone();
    let cb = checkbox("ctrl-cb", cx)
        .checked(app.checkbox_value)
        .on_toggle(move |v, _ev, _w, cx| {
            entity_cb.update(cx, |s, _cx| s.checkbox_value = v);
        })
        .render(cx);

    // --- switch ---
    let entity_sw = entity.clone();
    let sw = switch("ctrl-sw", cx)
        .checked(app.switch_value)
        .on_toggle(move |v, _ev, _w, cx| {
            entity_sw.update(cx, |s, _cx| s.switch_value = v);
        })
        .render(cx);

    // --- 3 radio buttons in a radio_group ---
    let rg_label = label("rg-current", format!("selected: {}", app.radio_value), cx)
        .muted(true)
        .render(cx);
    let rg_with_label = radio_group("ctrl-rg", cx)
        .name("choice")
        .selected(app.radio_value)
        .apply(div().flex().flex_row().gap(px(8.)).items_center())
        .child(rg_label);
    let rg_with_radios = (0..3).fold(rg_with_label, |acc, i| {
        let entity_r = entity.clone();
        acc.child(
            radio(format!("ctrl-radio-{i}"), cx)
                .checked(app.radio_value == i)
                .on_toggle(move |_v, _ev, _w, cx| {
                    entity_r.update(cx, |s, _cx| s.radio_value = i);
                })
                .render(cx),
        )
    });

    // --- slider (headless slider has no default_render; we
    //     build a track + knob div) ---
    let entity_sl = entity.clone();
    let slider_value = app.slider_value;
    let slider_track = slider("ctrl-slider", cx)
        .value(slider_value)
        .range(0.0, 100.0)
        .step(1.0)
        .on_change(move |v, _w, cx| {
            entity_sl.update(cx, |s, _cx| s.slider_value = v);
        })
        .apply(
            div()
                .w(px(240.))
                .h(px(8.))
                .rounded(px(4.))
                .bg(gpui::hsla(0.0, 0.0, 0.85, 1.0))
                .flex()
                .items_center()
                .child(
                    div()
                        .h(px(8.))
                        .w(px(slider_value * 2.4))
                        .rounded(px(4.))
                        .bg(gpui::rgb(0x0A0A0A)),
                ),
        );

    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(div().flex().flex_row().flex_wrap().gap(px(12.)).items_center().child(cell("checkbox", cb, cx)).child(cell("switch", sw, cx)))
        .child(cell("radio_group (3 radios)", rg_with_radios, cx))
        .child(div().flex().flex_col().gap(px(4.)).child(cell("slider (track + knob)", slider_track, cx)).child(label("slider-lbl", format!("slider: {slider_value:.1}"), cx).muted(true).render(cx)))
}
