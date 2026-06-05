//! yororen-ui Variant Showcase Demo
//!
//! Three side-by-side buttons, one per `ActionVariantKind`.
//! The renderer reads `action.<variant>.<field>` paths from
//! the theme JSON, so the only thing the variant changes
//! here is the `ButtonRenderState.variant` field.

use gpui::{
    Context, IntoElement, ParentElement, Render, Styled, Window, div, px,
};
use yororen_ui::headless::button::button;
use yororen_ui::headless::label::label;
use yororen_ui::renderer::{
    DefaultButton, DefaultLabel, ButtonRenderState, ButtonRenderer,
};
use yororen_ui::Edges;
use yororen_ui::RendererContext;
use yororen_ui::ActiveTheme;
use yororen_ui::markers::Button as ButtonMarker;
use yororen_ui::Theme;
use yororen_ui::ActionVariantKind;
use gpui::div as gpui_div;
use std::sync::Arc;

pub struct VariantApp;

impl VariantApp {
    pub fn new() -> Self {
        Self
    }
}

impl Render for VariantApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app: &mut gpui::App =
            unsafe { &mut *(cx as *mut Context<Self> as *mut gpui::App) };

        // Pull the registered ButtonRenderer + theme once, in
        // the outer scope. We can't borrow `cx` from inside a
        // closure without an unsafe lifetime extension, so
        // build the per-variant buttons inline rather than via
        // a helper closure.
        let r: &Arc<dyn ButtonRenderer> = cx
            .renderer_arc::<ButtonMarker, dyn ButtonRenderer>()
            .expect("ButtonRenderer registered");
        let theme: &Theme = cx.theme();

        let primary_state = ButtonRenderState {
            variant: ActionVariantKind::Primary,
            ..Default::default()
        };
        let primary_bg = r.bg(&primary_state, theme);
        let primary_fg = r.fg(&primary_state, theme);
        let primary_pad = r.padding(&primary_state, theme);
        let primary_radius = r.border_radius(&primary_state, theme);
        let primary_min_h = r.min_height(&primary_state, theme);

        let primary = button("primary-btn", app)
            .on_click(|_, _, _| {})
            .apply(
                gpui_div()
                    .bg(primary_bg)
                    .text_color(primary_fg)
                    .px(primary_pad.left)
                    .py(primary_pad.top)
                    .rounded(primary_radius)
                    .min_h(primary_min_h)
                    .child("Primary"),
            );

        let danger_state = ButtonRenderState {
            variant: ActionVariantKind::Danger,
            ..Default::default()
        };
        let danger_bg = r.bg(&danger_state, theme);
        let danger_fg = r.fg(&danger_state, theme);
        let danger_pad = r.padding(&danger_state, theme);
        let danger_radius = r.border_radius(&danger_state, theme);
        let danger_min_h = r.min_height(&danger_state, theme);

        let danger = button("danger-btn", app)
            .on_click(|_, _, _| {})
            .apply(
                gpui_div()
                    .bg(danger_bg)
                    .text_color(danger_fg)
                    .px(danger_pad.left)
                    .py(danger_pad.top)
                    .rounded(danger_radius)
                    .min_h(danger_min_h)
                    .child("Danger"),
            );

        let neutral = button("neutral-btn", app)
            .default_render(cx);

        div()
            .size_full()
            .p(px(24.))
            .flex()
            .flex_col()
            .gap_3()
            .child(label("title", "Variant showcase", app).default_render(cx))
            .child(
                label(
                    "blurb",
                    "Same headless::button, different ButtonRenderState.variant → different action.<key>.* paths from the JSON.",
                    app,
                )
                .default_render(cx),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(label("n", "Neutral (default_render):", app).default_render(cx))
                    .child(neutral),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(label("p", "Primary (hand-rolled apply):", app).default_render(cx))
                    .child(primary),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(label("d", "Danger (hand-rolled apply):", app).default_render(cx))
                    .child(danger),
            )
    }
}
