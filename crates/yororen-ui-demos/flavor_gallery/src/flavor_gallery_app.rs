//! Root component for the flavor gallery demo.
//!
//! Renders 4 columns side-by-side, one per Catppuccin flavor. Each
//! column is wrapped in `with_theme(theme, ...)` so its descendants
//! pick up the per-flavor Theme without touching the process
//! global. Inside each column we render the same set of components
//! (select, combo_box, "Show modal" button, tooltip) so the visual
//! difference between flavors is unambiguous.

use gpui::prelude::FluentBuilder;
use gpui::{
    App, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window, div, px,
};

use yororen_ui::component::{
    ComboBoxOption, OverlayCloseReason, SelectOption, button, combo_box, label, modal_actions_row,
    modal_dialog, select, with_theme,
};
use yororen_ui::theme::{ActionVariantKind, ActiveTheme, Theme};

use crate::state::{ActiveModal, FlavorGalleryState, FlavorKind};
use crate::theme_for;

const COLUMN_WIDTH: f32 = 290.0;
const GAP: f32 = 12.0;

pub struct FlavorGalleryApp;

impl FlavorGalleryApp {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for FlavorGalleryApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = cx.global::<FlavorGalleryState>();
        let active_modal = *state.active_modal.read(cx);
        let theme = cx.theme();
        let appearance = cx.window_appearance();

        // Top bar: 5 buttons (one per FlavorKind) for switching the
        // demo's active theme. We use the process-global theme for
        // the top bar; the 4 columns each get their own
        // with_theme override.
        let top_bar = div()
            .flex()
            .gap(px(GAP))
            .items_center()
            .child(label("Active flavor:").strong(true))
            .children(FlavorKind::ALL.iter().map(|kind| {
                let label_text: SharedString = kind.to_string().into();
                let appearance_for_handler = appearance;
                button(format!("flavor:{}", kind.as_str()))
                    .when(matches!(active_modal, ActiveModal::None), |this| {
                        // Only enable the flavor switcher when no
                        // modal is open. Modals take over the input
                        // focus.
                        this
                    })
                    .variant(if kind == &FlavorKind::default() {
                        ActionVariantKind::Primary
                    } else {
                        ActionVariantKind::Neutral
                    })
                    .child(label_text)
                    .on_click(move |_ev, window, cx| {
                        // Switch the process-global theme to the
                        // picked flavor. Note: this is a process-
                        // global effect; the 4 columns will
                        // re-render with their per-flavor
                        // with_theme override.
                        let theme = theme_for(*kind, appearance_for_handler);
                        // We don't have install_flavor; use a manual
                        // set_global to switch.
                        yororen_ui_core_theme_install(window, cx, theme);
                    })
                    .into_any_element()
            }))
            .child(label(
                " (top bar uses the active theme; the 4 columns are independent with_theme overrides)",
            ));

        // 4 columns, each wrapped in with_theme so its descendants
        // pick up the picked flavor. We also wrap the active
        // modal in the matching column.
        let flavor_columns = FlavorKind::ALL
            .iter()
            .filter(|k| !matches!(k, FlavorKind::System))
            .map(|kind| {
                let column_theme = theme_for(*kind, appearance);
                let column_active = active_modal == ActiveModal::Column(*kind);
                let state_for_open = state.active_modal.clone();
                let state_for_close = state.active_modal.clone();
                with_theme(column_theme, move || {
                    render_column(
                        *kind,
                        column_active,
                        state_for_open.clone(),
                        state_for_close.clone(),
                    )
                })
            })
            .collect::<Vec<_>>();

        // Show the active modal in its own full-window with_theme
        // block so its colors match the column it came from.
        let modal_overlay = if let ActiveModal::Column(kind) = active_modal {
            let modal_theme = theme_for(kind, appearance);
            let state_for_close = state.active_modal.clone();
            Some(with_theme(modal_theme, move || {
                render_modal(kind, state_for_close.clone())
            }))
        } else {
            None
        };

        div()
            .size_full()
            .bg(theme.surface.canvas)
            .flex()
            .flex_col()
            .gap(px(GAP))
            .p(px(20.0))
            .child(top_bar)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(GAP))
                    .children(flavor_columns),
            )
            .when_some(modal_overlay, |this, overlay| this.child(overlay))
    }
}

/// Render one of the 4 flavor columns.
fn render_column(
    kind: FlavorKind,
    modal_open: bool,
    state_for_open: Entity<ActiveModal>,
    state_for_close: Entity<ActiveModal>,
) -> gpui::AnyElement {
    let column_title: SharedString = format!("{} column", kind).into();
    let variant_kind = ActionVariantKind::Primary;
    let open_handler_state = state_for_open;
    let close_handler_state = state_for_close;
    let inner_button_id: SharedString = format!("flavor:{}:show-modal", kind.as_str()).into();
    let inner_select_id: SharedString = format!("flavor:{}:select", kind.as_str()).into();
    let inner_combo_id: SharedString = format!("flavor:{}:combo", kind.as_str()).into();

    // The same select, combo_box, button components are used
    // across all 4 columns. The only difference between columns
    // is the active Theme (set by with_theme in the parent).
    div()
        .w(px(COLUMN_WIDTH))
        .flex()
        .flex_col()
        .gap(px(8.0))
        .p(px(12.0))
        .rounded_lg()
        .border_1()
        .child(label(column_title.clone()).strong(true).text_size(px(16.0)))
        .child(
            label(
                "G-β: select honors Esc via dismiss_on_escape. \
                 Open it, then press Esc to close.",
            )
            .muted(true),
        )
        .child(
            select(inner_select_id.clone())
                .options([
                    SelectOption::new().value("apple").label("Apple"),
                    SelectOption::new().value("banana").label("Banana"),
                    SelectOption::new().value("cherry").label("Cherry"),
                ])
                .placeholder("Pick a fruit…"),
        )
        .child(
            label(
                "G-β: combo_box also honors Esc. \
                 Try typing then pressing Esc.",
            )
            .muted(true),
        )
        .child(
            combo_box(inner_combo_id.clone())
                .options([
                    ComboBoxOption::new("cat", "Cat"),
                    ComboBoxOption::new("dog", "Dog"),
                    ComboBoxOption::new("fish", "Fish"),
                ])
                .placeholder("Pick a pet…"),
        )
        .child(label("G-γ + G-δ: modal_dialog one-line a11y shell."))
        .child(
            button(inner_button_id.clone())
                .variant(variant_kind)
                .child("Show modal")
                .on_click(move |_ev, _w, cx| {
                    open_handler_state.update(cx, |v, _| {
                        *v = ActiveModal::Column(kind);
                    });
                    cx.refresh_windows();
                }),
        )
        .when(modal_open, |this| {
            // Render the modal embedded in this column's with_theme
            // context. The modal_dialog factory auto-composes the
            // Overlay + ScrollLock.
            this.child(
                modal_dialog(format!("flavor:{}:modal", kind.as_str()))
                    .title(format!("{} modal", kind))
                    .content(label(format!(
                        "Modal rendered inside the {} flavor. \
                             Press Esc / click the scrim / click the X to close. \
                             All three paths route to a single on_close.",
                        kind
                    )))
                    .actions(modal_actions_row(
                        yororen_ui::i18n::TextDirection::Ltr,
                        [
                            button(format!("flavor:{}:modal:cancel", kind.as_str()))
                                .child("Cancel")
                                .into_any_element(),
                            button(format!("flavor:{}:modal:ok", kind.as_str()))
                                .variant(ActionVariantKind::Primary)
                                .child("OK")
                                .into_any_element(),
                        ],
                    ))
                    .open(true)
                    .on_close(move |reason: &OverlayCloseReason, _w, cx| {
                        // G-δ: all close paths (scrim / Esc / X)
                        // route through this single callback.
                        eprintln!("[{} modal] closed via {:?}", kind, reason);
                        close_handler_state.update(cx, |v, _| {
                            *v = ActiveModal::None;
                        });
                        cx.refresh_windows();
                    })
                    .into_any_element(),
            )
        })
        .into_any_element()
}

/// Render the active modal in its own with_theme block.
///
/// Actually this is unused: we render the modal inline in
/// `render_column` so the modal sits inside the column's
/// with_theme context (the column's flavor). Kept here for
/// future use (e.g. a 'Show all modals at once' mode).
#[allow(dead_code)]
fn render_modal(kind: FlavorKind, state_for_close: Entity<ActiveModal>) -> gpui::AnyElement {
    modal_dialog(format!("flavor:{}:modal", kind.as_str()))
        .title(format!("{} modal", kind))
        .content(label(format!("Modal for {}", kind)))
        .on_close(move |_reason, _w, cx| {
            state_for_close.update(cx, |v, _| {
                *v = ActiveModal::None;
            });
            cx.refresh_windows();
        })
        .open(true)
        .into_any_element()
}

/// Helper to install a Theme into the App as the process-global
/// theme. The standard `install` / `install_flavor` helpers
/// expect a flavor or a fixed system palette; for ad-hoc theme
/// switching (the user picks a flavor in the top bar) we set
/// the global directly.
fn yororen_ui_core_theme_install(_window: &mut Window, cx: &mut App, theme: Theme) {
    use yororen_ui::theme::GlobalTheme;
    let appearance = cx.window_appearance();
    cx.set_global(GlobalTheme::new_with_themes(
        appearance,
        yororen_ui::theme::ThemeSet::new(theme),
    ));
}
