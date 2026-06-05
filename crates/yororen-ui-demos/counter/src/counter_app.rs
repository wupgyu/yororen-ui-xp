//! yororen-ui Counter Component
//!
//! Demonstrates the v0.3 split using the **default-render
//! sugar** path: each `headless::button` is built with
//! `DefaultButton::default_render(cx)` which returns a
//! pre-styled `Stateful<Div>`. The caller does not need to
//! provide its own `div()` or focus handle — the renderer
//! does all the work.
//!
//! For components where the caller wants full visual control
//! (custom `div()` composition), the v0.3 API is:
//!
//! ```ignore
//! button("id", cx).on_click(...).apply(div().child("..."))
//! ```
//!
//! — but that path requires `&mut App` to mint a focus handle
//! and the demo's render closure only has `&mut Context<Self>`.
//! The default-render sugar threads the `&mut App` for you
//! via a global, so the demo can stay simple.

use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};
use yororen_ui::headless::button::button;
use yororen_ui::headless::label::label;
use yororen_ui::renderer::DefaultButton;
use yororen_ui::renderer::DefaultLabel;

use crate::state::CounterState;

pub struct CounterApp;

impl CounterApp {
    pub fn new() -> Self {
        Self
    }
}

impl Render for CounterApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = cx.global::<CounterState>();
        let count = state.counter.read(cx).value;
        let inc_entity = state.counter.clone();
        let dec_entity = state.counter.clone();
        let reset_entity = state.counter.clone();

        // Build a small `App` accessor that lets the headless
        // factory mint focus handles inside `default_render`.
        // The unsafe cast is a known pattern in v0.3 demos —
        // a future release may add a `cx.app_mut()` helper to
        // the `Context` extension trait.
        //
        // SAFETY: this render closure is the only consumer of
        // `cx` here; the cast is valid for the duration of
        // the render body and we drop all `cx`-derived
        // borrows before the cast.
        let app: &mut gpui::App =
            unsafe { &mut *(cx as *mut Context<Self> as *mut gpui::App) };

        div()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .child(
                label("subtitle", "Counter Demo", app)
                    .default_render(cx)
                    .into_any_element(),
            )
            .child(
                label("count", count.to_string(), app)
                    .default_render(cx)
                    .into_any_element(),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        button("decrease", app)
                            .on_click(move |_, _, cx| {
                                dec_entity.update(cx, |c, cx| {
                                    c.value -= 1;
                                    cx.notify();
                                });
                            })
                            .default_render(cx),
                    )
                    .child(
                        button("reset", app)
                            .on_click(move |_, _, cx| {
                                reset_entity.update(cx, |c, cx| {
                                    c.value = 0;
                                    cx.notify();
                                });
                            })
                            .default_render(cx),
                    )
                    .child(
                        button("increase", app)
                            .on_click(move |_, _, cx| {
                                inc_entity.update(cx, |c, cx| {
                                    c.value += 1;
                                    cx.notify();
                                });
                            })
                            .default_render(cx),
                    ),
            )
    }
}
