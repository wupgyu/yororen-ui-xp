//! Section 7 — Notifications.
//!
//! Demonstrates `Notification` + `NotificationCenter` integration.
//! The headless layer doesn't expose a `headless::toast` /
//! `headless::notification` factory; instead the app fires
//! notifications into the global center, and the host renders
//! the list. The toolbar's "Show toast" / "Show notification"
//! buttons mutate `state.toast_count` and call
//! `center.notify(...)`; this section reads `center.items()` and
//! draws a stack of cards using the current `cx.theme()` colors
//! (so a renderer switch re-themes the toasts on the next paint).

use gpui::{Context, Div, InteractiveElement, ParentElement, Styled, div, hsla, px};

use yororen_ui::headless::label::label;
use yororen_ui::notification::center::{Notification, NotificationCenter, ToastKind};
use yororen_ui::theme::ActiveTheme;
use yororen_ui::i18n::Translate;

use crate::state::GalleryApp;

pub fn render(cx: &mut Context<GalleryApp>) -> Div {
    let center = cx.global::<NotificationCenter>();
    let items: Vec<Notification> = center.items();

    // Theme-derived background for the toast card. We pick
    // `surface.raised` and tint with the `kind` color.
    let card_bg = cx
        .theme()
        .get_color("surface.raised")
        .unwrap_or_else(|| hsla(0.0, 0.0, 0.98, 1.0));
    let card_border = cx
        .theme()
        .get_color("border.default")
        .unwrap_or_else(|| hsla(0.0, 0.0, 0.5, 0.3));
    let text_color = cx
        .theme()
        .get_color("content.primary")
        .unwrap_or_else(|| hsla(0.0, 0.0, 0.05, 1.0));

    let mut stack = div()
        .flex()
        .flex_col()
        .gap(px(8.))
        .child(
            label("notif-info", cx.t("notification.click_buttons"), cx)
                .muted(true)
                .render(cx),
        );

    if items.is_empty() {
        stack = stack.child(
            label("notif-empty", cx.t("notification.empty"), cx)
                .muted(true)
                .render(cx),
        );
    } else {
        for n in items.iter() {
            let (kind_label, kind_color) = match n.kind {
                ToastKind::Success => ("success", hsla(0.33, 0.6, 0.4, 1.0)),
                ToastKind::Warning => ("warning", hsla(0.1, 0.7, 0.45, 1.0)),
                ToastKind::Error => ("error", hsla(0.0, 0.6, 0.45, 1.0)),
                ToastKind::Info => ("info", hsla(0.55, 0.5, 0.45, 1.0)),
                ToastKind::Neutral => ("neutral", hsla(0.0, 0.0, 0.4, 1.0)),
            };
            let id = format!("toast-{}", n.id.raw());
            stack = stack.child(
                div()
                    .id(id)
                    .w(px(320.))
                    .p(px(12.))
                    .rounded(px(6.))
                    .border_1()
                    .border_color(card_border)
                    .bg(card_bg)
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.))
                            .child(
                                div()
                                    .px(px(6.))
                                    .py(px(2.))
                                    .rounded(px(4.))
                                    .bg(kind_color)
                                    .child(
                                        label(
                                            "toast-kind",
                                            kind_label.to_string(),
                                            cx,
                                        )
                                        .strong(true)
                                        .render(cx),
                                    ),
                            )
                            .child(
                                label(
                                    "toast-msg",
                                    n.message.to_string(),
                                    cx,
                                )
                                .render(cx)
                                .text_color(text_color),
                            ),
                    ),
            );
        }
    }

    stack
}
