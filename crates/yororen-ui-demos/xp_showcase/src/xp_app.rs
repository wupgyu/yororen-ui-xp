//! yororen-ui Windows XP Showcase Demo
//!
//! One window of core components, all painted by the XP (Luna)
//! renderer. The code is 100% headless — the XP look comes
//! entirely from the installed renderer + `xp-luna.json`.

use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, px};

use yororen_ui::ActionVariantKind;
use yororen_ui::headless::badge::{BadgeVariant, badge};
use yororen_ui::headless::button::button;
use yororen_ui::headless::checkbox::checkbox;
use yororen_ui::headless::divider::divider;
use yororen_ui::headless::heading::{HeadingLevel, heading};
use yororen_ui::headless::label::label;
use yororen_ui::headless::layout::{AlignItems, Inset, Spacing, column, row, wrap};
use yororen_ui::headless::progress::progress;
use yororen_ui::headless::radio::radio;
use yororen_ui::headless::radio_group::radio_group;
use yororen_ui::headless::search_input::search_input;
use yororen_ui::headless::slider::slider;
use yororen_ui::headless::switch::switch;
use yororen_ui::headless::tag::tag;
use yororen_ui::headless::text_input::text_input;
use yororen_ui::theme::ActiveTheme;

pub struct XpApp {
    checkbox_value: bool,
    switch_value: bool,
    radio_value: usize,
    slider_value: f32,
    text_value: String,
    search_value: String,
    tag_selected: bool,
}

impl XpApp {
    pub fn new() -> Self {
        Self {
            checkbox_value: true,
            switch_value: false,
            radio_value: 0,
            slider_value: 35.0,
            text_value: String::new(),
            search_value: String::new(),
            tag_selected: true,
        }
    }
}

impl Render for XpApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        // Paint the root with the theme's dialog beige so the
        // window reads as an XP surface, not the OS chrome color.
        let surface = cx
            .theme()
            .get_color("surface.base")
            .unwrap_or_else(|| gpui::hsla(0.15, 0.2, 0.85, 1.0));

        // ---- Buttons: the gradient-face family ----
        let buttons = wrap("xp-row-buttons", cx)
            .items(AlignItems::Center)
            .gap(Spacing::Md)
            .child(
                button("btn-neutral", cx)
                    .on_click(|_, _, _| {})
                    .render(cx)
                    .child("OK"),
            )
            .child(
                button("btn-primary", cx)
                    .variant(ActionVariantKind::Primary)
                    .on_click(|_, _, _| {})
                    .render(cx)
                    .child("Default"),
            )
            .child(
                button("btn-danger", cx)
                    .variant(ActionVariantKind::Danger)
                    .on_click(|_, _, _| {})
                    .render(cx)
                    .child("Delete"),
            )
            .child(
                button("btn-disabled", cx)
                    .disabled(true)
                    .render(cx)
                    .child("Disabled"),
            );

        // ---- Progress: the green segmented bar ----
        let progress_col = column("xp-col-progress", cx)
            .gap(Spacing::Sm)
            .child(
                progress("prg-segments", cx)
                    .value(0.6)
                    .max(1.0)
                    .label("Downloading…")
                    .render(cx),
            )
            .child(progress("prg-marquee", cx).indeterminate(true).render(cx));

        // ---- Controls: checkbox / switch / radio / slider ----
        let entity_cb = entity.clone();
        let cb = checkbox("cbx-main", cx)
            .checked(self.checkbox_value)
            .on_toggle(move |v, _ev, _w, cx| {
                entity_cb.update(cx, |s, _cx| s.checkbox_value = v);
            })
            .render(cx);

        let entity_sw = entity.clone();
        let sw = switch("swt-main", cx)
            .checked(self.switch_value)
            .on_toggle(move |v, _ev, _w, cx| {
                entity_sw.update(cx, |s, _cx| s.switch_value = v);
            })
            .render(cx);

        let checkbox_row = row("xp-row-cb-sw", cx)
            .items_center()
            .gap(Spacing::Lg)
            .child(
                row("xp-row-cb", cx)
                    .items_center()
                    .gap(Spacing::Sm)
                    .child(cb)
                    .child(label("lbl-cbx", "Check me", cx).render(cx))
                    .render(cx),
            )
            .child(
                row("xp-row-sw", cx)
                    .items_center()
                    .gap(Spacing::Sm)
                    .child(sw)
                    .child(label("lbl-swt", "Switch", cx).render(cx))
                    .render(cx),
            );

        let rg = radio_group("rdg-choice", cx)
            .name("choice")
            .selected(self.radio_value)
            .apply(div().flex().flex_row().gap(px(16.)).items_center())
            .child(label("lbl-rg", "Pick one:", cx).render(cx));
        let rg = (0..3).fold(rg, |acc, i| {
            let entity_r = entity.clone();
            acc.child(
                row(format!("xp-row-rdo-{i}"), cx)
                    .items_center()
                    .gap(Spacing::Xs)
                    .child(
                        radio(format!("rdo-{i}"), cx)
                            .checked(self.radio_value == i)
                            .on_toggle(move |_v, _ev, _w, cx| {
                                entity_r.update(cx, |s, _cx| s.radio_value = i);
                            })
                            .render(cx),
                    )
                    .child(
                        label(format!("lbl-rdo-{i}"), format!("Option {}", i + 1), cx).render(cx),
                    )
                    .render(cx),
            )
        });

        let entity_sl = entity.clone();
        let slider_value = self.slider_value;
        let slider_col = column("xp-col-slider", cx)
            .gap(Spacing::Sm)
            .child(
                slider("sld-volume", cx)
                    .value(slider_value)
                    .range(0.0, 100.0)
                    .step(1.0)
                    .on_change(move |v, _w, cx| {
                        entity_sl.update(cx, |s, _cx| s.slider_value = v);
                    })
                    .render(cx),
            )
            .child(
                label("lbl-slider", format!("Volume: {:.0}", slider_value), cx)
                    .muted(true)
                    .render(cx),
            );

        // ---- Inputs: white sunken wells ----
        let entity_text = entity.clone();
        let ti = text_input("txi-name")
            .placeholder("Type your name…")
            .on_change(move |new: &str, _w, cx| {
                entity_text.update(cx, |s, _cx| s.text_value = new.to_string());
            })
            .render(cx, window);

        let entity_search = entity.clone();
        let si = search_input("sch-find")
            .placeholder("Search…")
            .on_change(move |new: &str, _w, cx| {
                entity_search.update(cx, |s, _cx| s.search_value = new.to_string());
            })
            .render(cx, window);

        let inputs_row = row("xp-row-inputs", cx)
            .items_center()
            .gap(Spacing::Md)
            .child(div().w(px(220.)).child(ti))
            .child(div().w(px(220.)).child(si));

        // ---- Badges + tag ----
        let entity_tag = entity.clone();
        let badge_row = wrap("xp-row-badges", cx)
            .items(AlignItems::Center)
            .gap(Spacing::Sm)
            .child(
                badge("bdg-info", "Info", cx)
                    .variant(BadgeVariant::Info)
                    .render(cx),
            )
            .child(
                badge("bdg-success", "Success", cx)
                    .variant(BadgeVariant::Success)
                    .render(cx),
            )
            .child(
                badge("bdg-warning", "Warning", cx)
                    .variant(BadgeVariant::Warning)
                    .render(cx),
            )
            .child(
                tag("tag-xp", "Windows XP", cx)
                    .selected(self.tag_selected)
                    .on_click(move |_, _, cx| {
                        entity_tag.update(cx, |s, _cx| s.tag_selected = !s.tag_selected);
                    })
                    .render(cx),
            );

        column("xp-root", cx)
            .w_full()
            .h_full()
            .p(Inset::Xl)
            .gap(Spacing::Lg)
            .child(heading("hdg-title", HeadingLevel::H2, "Windows XP Showcase", cx).render(cx))
            .child(
                label(
                    "lbl-blurb",
                    "Every component below is painted by the XP (Luna) renderer.",
                    cx,
                )
                .muted(true)
                .render(cx),
            )
            .child(divider("dvr-1", cx).render(cx))
            .child(buttons.render(cx))
            .child(progress_col.render(cx))
            .child(checkbox_row.render(cx))
            .child(rg)
            .child(slider_col.render(cx))
            .child(divider("dvr-2", cx).render(cx))
            .child(inputs_row.render(cx))
            .child(badge_row.render(cx))
            .render(cx)
            .bg(surface)
    }
}
