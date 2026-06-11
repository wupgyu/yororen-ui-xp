//! `GalleryApp` — the demo's root view.
//!
//! The render flow:
//! 1. **Per-render renderer install** — `install_renderer` is called
//!    at the top of every `Render::render`. The renderers' `register_arc`
//!    is `last-wins` and `install_with` is idempotent, so a single click
//!    on the toolbar's renderer toggle causes the next paint to read
//!    the new renderers' tokens. See `theme_switcher.rs`.
//! 2. **Host window registration** — `center.register_host_window`
//!    is called every render so the notification auto-dismiss
//!    timer has a window to refresh. Without this the timer is
//!    never scheduled and non-sticky toasts would never disappear.
//! 3. **Locale install on change** — when the toolbar picks a new
//!    locale, we call the corresponding `yororen_ui::locale_xx::install`
//!    to overwrite the global `I18n`.
//! 4. **7 section call** — actions / display / surfaces / inputs /
//!    controls / overlays / lists. Each lives in `sections/<name>.rs`.
//! 5. **Global notification host** —
//!    [`crate::notifications_host::deferred_host`] is the LAST child
//!    of the root, wrapped in `gpui::deferred` at priority 3 so it
//!    paints above the modal scrim and every other overlay. The
//!    toolbar's "Show toast" / "Show notification" buttons push into
//!    the global `NotificationCenter`; the host reads that queue and
//!    renders each item as a floating card in the top-right corner.

use gpui::{
    Context, Div, InteractiveElement, IntoElement, ParentElement, Render, Stateful,
    StatefulInteractiveElement, Styled, Window, div, hsla, px,
};

use yororen_ui::headless::heading::heading;
use yororen_ui::headless::heading::HeadingLevel;
use yororen_ui::i18n::Translate;
use yororen_ui::notification::center::{Notification, NotificationCenter, ToastKind};
use yororen_ui::theme::ActiveTheme;
use yororen_ui::ActionVariantKind;
use yororen_ui::headless::button::button;
use yororen_ui::headless::divider::divider;
use yororen_ui::headless::label::label;
use yororen_ui::headless::toggle_button::toggle_button;

use crate::sections;
use crate::state::{GalleryApp, LocaleChoice};
use crate::theme_switcher::{install_renderer, DarkMode, RendererKind};

impl Render for GalleryApp {
    // The `&mut **cx` recovers a `&mut gpui::App` from the
    // `&mut Context<Self>` (the v0.3 `DerefMut<Target = App>`
    // pattern — see `memory.md`). Clippy sees it as redundant
    // auto-deref but the conversion is intentional.
    #[allow(clippy::explicit_auto_deref)]
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 1. Per-render renderer + theme install. Cheap (39
        //    HashMap.inserts + 1 set_global) and guarantees the
        //    window always reflects the latest toolbar click.
        install_renderer(&mut **cx, self.current_renderer, self.dark_mode);

        // 2. Register the host window with the notification
        //    center. `maybe_schedule_auto_dismiss` returns
        //    early if `host_window` is `None`, so the global
        //    timer that auto-removes non-sticky toasts won't
        //    fire unless we bind the current window. Cheap
        //    (one `Mutex` lock + `Some` assignment).
        if let Some(center) = cx.try_global::<NotificationCenter>() {
            center.register_host_window(window.window_handle());
        }

        // 3. Surface color for the window background.
        let surface = cx.theme().get_color("surface.base").unwrap_or_default();

        // 4. Build the toolbar (renderer / dark / locale / toast triggers).
        let toolbar = build_toolbar(self, cx);

        // 5. Scrollable root. The 7 sections are vertical children;
        //    long content overflows. `relative` provides the
        //    containing block for the modal scrim rendered by
        //    the overlays section AND for the global notification
        //    host appended below.
        let scroll_root: Stateful<Div> = div()
            .id("gallery-scroll")
            .relative()
            .size_full()
            .bg(surface)
            .flex()
            .flex_col()
            .gap(px(24.))
            .p(px(24.))
            .overflow_y_scroll();

        scroll_root
            .child(toolbar)
            .child(divider("toolbar-divider", cx).apply(div()).my(px(8.)))
            .child(sections::actions(self, window, cx))
            .child(sections::display(self, window, cx))
            .child(sections::surfaces(self, window, cx))
            .child(sections::inputs(self, window, cx))
            .child(sections::controls(self, window, cx))
            .child(sections::overlays(self, window, cx))
            .child(sections::lists(self, window, cx))
            .child(footer_section(self, cx))
            // Global notification host: always the last child so
            // its deferred-paint (priority 3) lands on top of the
            // modal scrim (priority 2) and popover / dropdown
            // panels (priority 1). The host is empty when the
            // queue is empty, so adding it always is cheap.
            .child(crate::notifications_host::deferred_host(cx))
    }
}

/// Toolbar at the top of the window.
///
/// Layout (horizontal, gap 12px):
/// ```
/// [title] | [Default | Brutalism]  [Light | Dark]  [EN | 中文 | العربية]  [Show toast]
/// ```
fn build_toolbar(app: &mut GalleryApp, cx: &mut Context<GalleryApp>) -> Div {
    let entity = cx.entity().clone();
    let mut row = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(px(12.))
        .child(
            heading("title", HeadingLevel::H1, "yororen-ui gallery", cx)
                .apply(div())
                .mr(px(8.)),
        );

    // RendererKind toggle: 2 toggle_buttons, mutually exclusive via
    // state.current_renderer.
    let entity_for_renderer = entity.clone();
    row = row.child(
        toggle_button("renderer-default", cx)
            .selected(app.current_renderer == RendererKind::Default)
            .variant(ActionVariantKind::Primary)
            .on_toggle(move |_selected, _ev, _window, cx| {
                entity_for_renderer.update(cx, |s, _cx| {
                    s.current_renderer = RendererKind::Default;
                });
            })
            .render(cx)
            .child("Default"),
    );
    let entity_for_renderer = entity.clone();
    row = row.child(
        toggle_button("renderer-brutalism", cx)
            .selected(app.current_renderer == RendererKind::Brutalism)
            .variant(ActionVariantKind::Primary)
            .on_toggle(move |_selected, _ev, _window, cx| {
                entity_for_renderer.update(cx, |s, _cx| {
                    s.current_renderer = RendererKind::Brutalism;
                });
            })
            .render(cx)
            .child("Brutalism"),
    );

    // Dark mode toggle (2 toggle_buttons).
    let entity_for_dark = entity.clone();
    row = row.child(
        toggle_button("dark-light", cx)
            .selected(app.dark_mode == DarkMode::Light)
            .on_toggle(move |_selected, _ev, _window, cx| {
                entity_for_dark.update(cx, |s, _cx| {
                    s.dark_mode = DarkMode::Light;
                });
            })
            .render(cx)
            .child("Light"),
    );
    let entity_for_dark = entity.clone();
    row = row.child(
        toggle_button("dark-dark", cx)
            .selected(app.dark_mode == DarkMode::Dark)
            .on_toggle(move |_selected, _ev, _window, cx| {
                entity_for_dark.update(cx, |s, _cx| {
                    s.dark_mode = DarkMode::Dark;
                });
            })
            .render(cx)
            .child("Dark"),
    );

    // Locale: 3 toggle_buttons. Toggling calls the appropriate
    // `yororen_ui::locale_xx::install(cx)` to overwrite the global
    // I18n.
    for (id, choice, label) in [
        ("locale-en", LocaleChoice::En, "EN"),
        ("locale-zh", LocaleChoice::ZhCn, "中文"),
        ("locale-ar", LocaleChoice::Ar, "العربية"),
    ] {
        let entity_for_locale = entity.clone();
        let label = label.to_string();
        let selected = app.current_locale == choice;
        let tb = toggle_button(id, cx)
            .selected(selected)
            .on_toggle(move |_selected, _ev, _window, cx| {
                entity_for_locale.update(cx, |s, _cx| {
                    s.current_locale = choice;
                });
                match choice {
                    LocaleChoice::En => yororen_ui::locale_en::install(cx),
                    LocaleChoice::ZhCn => yororen_ui::locale_zh_cn::install(cx),
                    LocaleChoice::Ar => yororen_ui::locale_ar::install(cx),
                };
            })
            .render(cx)
            .child(label);
        row = row.child(tb);
    }

    // Show toast button.
    let entity_for_toast = entity.clone();
    row = row.child(
        button("show-toast", cx)
            .variant(ActionVariantKind::Danger)
            .on_click(move |_, _, cx| {
                let id = entity_for_toast.update(cx, |s, _cx| {
                    s.toast_count += 1;
                    s.toast_count
                });
                // Defer the global access into a separate
                // statement so the immutable borrow on `cx`
                // ends before `notify` takes a `&mut`.
                let center = cx.global::<NotificationCenter>().clone();
                let msg = format!("Toast #{id}: gallery is alive");
                center.notify(
                    Notification::new(msg)
                        .title("Gallery")
                        .kind(ToastKind::Info),
                    cx,
                );
            })
            .render(cx)
            .child("Show toast"),
    );

    // Show notification (sticky) button.
    let entity_for_notify = entity.clone();
    row = row.child(
        button("show-notification", cx)
            .on_click(move |_, _, cx| {
                let id = entity_for_notify.update(cx, |s, _cx| s.toast_count + 1);
                let center = cx.global::<NotificationCenter>().clone();
                let msg = format!("Sticky notification #{id}");
                center.notify(
                    Notification::new(msg)
                        .title("Gallery")
                        .kind(ToastKind::Success)
                        .sticky(true),
                    cx,
                );
            })
            .render(cx)
            .child("Show notification"),
    );

    row
}

/// Footer at the bottom: shows live counters so the user can
/// verify state changes are wired correctly.
fn footer_section(app: &GalleryApp, cx: &mut Context<GalleryApp>) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(4.))
        .mt(px(16.))
        .p(px(12.))
        .rounded(px(6.))
        .border_1()
        .border_color(hsla(0.0, 0.0, 0.5, 0.3))
        .child(
            label(
                "footer-title",
                cx.t("gallery.footer.live_counters"),
                cx,
            )
            .strong(true)
            .render(cx),
        )
        .child(label(
            "footer-form",
            format!(
                "form_submit_count: {}  |  email: {:?}  |  error: {:?}",
                app.form_submit_count, app.form_email_value, app.form_email_error
            ),
            cx,
        )
        .render(cx))
        .child(label(
            "footer-controls",
            format!(
                "checkbox: {}  |  switch: {}  |  radio: {}  |  slider: {:.1}",
                app.checkbox_value, app.switch_value, app.radio_value, app.slider_value
            ),
            cx,
        )
        .render(cx))
        .child(label(
            "footer-toast",
            format!("toast_count: {}  |  locale: {}", app.toast_count, app.current_locale.tag()),
            cx,
        )
        .render(cx))
}
