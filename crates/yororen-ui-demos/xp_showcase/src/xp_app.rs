//! yororen-ui Windows XP Showcase Demo
//!
//! One window of core components, all painted by the XP (Luna)
//! renderer. The code is 100% headless — the XP look comes
//! entirely from the installed renderer + `xp-luna.json`.

use gpui::{
    Context, Div, FontWeight, Hsla, InteractiveElement, IntoElement, ParentElement, Render,
    Stateful, StatefulInteractiveElement, Styled, Window, WindowControlArea, div, hsla,
    linear_color_stop, linear_gradient, px,
};

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
use yororen_ui::theme::Theme;

/// One XP caption button (min / max / close). The action itself is
/// performed natively by Windows through the `WindowControlArea`
/// hitbox, so no `on_click` handler is needed; gpui hover/active
/// styles still apply on top.
///
/// The face is a glossy vertical gradient (light top → saturated
/// bottom) with a translucent white edge, matching the Luna
/// caption-button look.
fn caption_button(
    id: &'static str,
    glyph: &'static str,
    area: WindowControlArea,
    top: Hsla,
    bottom: Hsla,
    glyph_color: Hsla,
) -> Stateful<Div> {
    let hover_top = Hsla {
        l: (top.l + 0.07).clamp(0.0, 1.0),
        ..top
    };
    let hover_bottom = Hsla {
        l: (bottom.l + 0.07).clamp(0.0, 1.0),
        ..bottom
    };
    div()
        .id(id)
        .window_control_area(area)
        .w(px(21.))
        .h(px(21.))
        .rounded(px(3.))
        .border(px(1.))
        .border_color(hsla(0.0, 0.0, 1.0, 0.6))
        .bg(linear_gradient(
            180.0,
            linear_color_stop(top, 0.0),
            linear_color_stop(bottom, 1.0),
        ))
        .flex()
        .items_center()
        .justify_center()
        .text_color(glyph_color)
        .text_size(px(14.))
        .child(glyph)
        .hover(move |s| {
            s.bg(linear_gradient(
                180.0,
                linear_color_stop(hover_top, 0.0),
                linear_color_stop(hover_bottom, 1.0),
            ))
        })
        .active(move |s| {
            s.bg(linear_gradient(
                180.0,
                linear_color_stop(bottom, 0.0),
                linear_color_stop(top, 1.0),
            ))
        })
}

/// The XP Luna title bar, styled after the classic CSS recipe:
///
/// ```css
/// height: 26px;
/// border-radius: 7px 7px 0 0;   /* inside the 8px window frame */
/// background: linear-gradient(180deg, #0997ff 0%, #0053ee 8%,
///     #0050ee 40%, #06f 88%, #0058eb 100%);
/// padding: 0 4px 0 8px; gap: 6px;
/// ```
///
/// gpui's `linear_gradient` only supports two stops, so the
/// 5-stop vertical gradient is approximated with four stacked
/// 2-stop bands (heights proportional to the CSS percentages).
/// Dragging / min / max / close are native Windows behaviors via
/// `WindowControlArea` hitboxes (the demo window uses
/// `appears_transparent` client decorations).
fn xp_title_bar(_theme: &Theme) -> Stateful<Div> {
    // The five CSS gradient stops.
    let c0: Hsla = gpui::rgb(0x0997FF).into(); // 0%
    let c1: Hsla = gpui::rgb(0x0053EE).into(); // 8%
    let c2: Hsla = gpui::rgb(0x0050EE).into(); // 40%
    let c3: Hsla = gpui::rgb(0x0066FF).into(); // 88%
    let c4: Hsla = gpui::rgb(0x0058EB).into(); // 100%
    let title_fg: Hsla = gpui::rgb(0xFFFFFF).into();
    // Caption-button faces: glossy light blue over the bar blue;
    // the close button is the signature Luna orange-red.
    let btn_top: Hsla = gpui::rgb(0x3C8CFD).into();
    let btn_bottom: Hsla = gpui::rgb(0x1565E8).into();
    let close_top: Hsla = gpui::rgb(0xF08A6D).into();
    let close_bottom: Hsla = gpui::rgb(0xD84A28).into();

    // Four stacked bands ≈ the 5-stop vertical gradient
    // (2/8/12/4 px of the 26 px bar, matching 0-8-40-88-100%).
    let bands = div()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .bottom_0()
        .flex()
        .flex_col()
        .child(div().h(px(2.)).w_full().bg(linear_gradient(
            180.0,
            linear_color_stop(c0, 0.0),
            linear_color_stop(c1, 1.0),
        )))
        .child(div().h(px(8.)).w_full().bg(linear_gradient(
            180.0,
            linear_color_stop(c1, 0.0),
            linear_color_stop(c2, 1.0),
        )))
        .child(div().h(px(12.)).w_full().bg(linear_gradient(
            180.0,
            linear_color_stop(c2, 0.0),
            linear_color_stop(c3, 1.0),
        )))
        .child(div().h(px(4.)).w_full().bg(linear_gradient(
            180.0,
            linear_color_stop(c3, 0.0),
            linear_color_stop(c4, 1.0),
        )));

    // Mini 2x2 "window panes" app icon.
    let pane = |c: Hsla| div().w(px(7.)).h(px(7.)).bg(c);
    let pane_row = |a: Hsla, b: Hsla| {
        div()
            .flex()
            .flex_row()
            .gap(px(1.))
            .child(pane(a))
            .child(pane(b))
    };
    let icon = div()
        .flex()
        .flex_col()
        .gap(px(1.))
        .child(pane_row(
            hsla(0.02, 0.9, 0.55, 1.0),
            hsla(0.28, 0.8, 0.5, 1.0),
        ))
        .child(pane_row(
            hsla(0.12, 0.9, 0.55, 1.0),
            hsla(0.6, 0.85, 0.55, 1.0),
        ));

    div()
        .id("xp-titlebar")
        .window_control_area(WindowControlArea::Drag)
        .relative()
        .w_full()
        .h(px(26.))
        .rounded_tl(px(7.))
        .rounded_tr(px(7.))
        .overflow_hidden()
        .child(bands)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .w_full()
                .h_full()
                .pl(px(8.))
                .pr(px(4.))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(6.))
                        .child(icon)
                        .child(
                            div()
                                .text_color(title_fg)
                                .text_size(px(12.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Windows XP Showcase"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(2.))
                        .child(caption_button(
                            "xp-cap-min",
                            "_",
                            WindowControlArea::Min,
                            btn_top,
                            btn_bottom,
                            title_fg,
                        ))
                        .child(caption_button(
                            "xp-cap-max",
                            "□",
                            WindowControlArea::Max,
                            btn_top,
                            btn_bottom,
                            title_fg,
                        ))
                        .child(div().ml(px(2.)).child(caption_button(
                            "xp-cap-close",
                            "×",
                            WindowControlArea::Close,
                            close_top,
                            close_bottom,
                            title_fg,
                        ))),
                ),
        )
}

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

        let content = column("xp-root", cx)
            .w_full()
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
            .flex_grow();

        // Window chrome per css `.xp-window`: the frame carries
        // the 1px `#0058E6` border and 8px rounded top corners
        // (unpainted corner pixels stay transparent so the
        // desktop shows through); the body sits 3px inside it
        // with its own `#A09C8C` inner border (top edge open,
        // like `.xp-window-body`). No outer window shadow.
        let frame_border: Hsla = gpui::rgb(0x0058E6).into();
        let body_border: Hsla = gpui::rgb(0xA09C8C).into();
        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(surface)
            .border(px(1.))
            .border_color(frame_border)
            .rounded_tl(px(8.))
            .rounded_tr(px(8.))
            .overflow_hidden()
            .child(xp_title_bar(cx.theme()))
            .child(
                div()
                    .flex_grow()
                    .flex()
                    .flex_col()
                    .mx(px(3.))
                    .mb(px(3.))
                    .border(px(1.))
                    .border_t(px(0.))
                    .border_color(body_border)
                    .child(content),
            )
    }
}
