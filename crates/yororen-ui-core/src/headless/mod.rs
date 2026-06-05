//! Headless primitives for yororen-ui.
//!
//! Every function in this module returns a `XxxProps` struct that
//! describes the *behavior* of a UI element (focus, click, key
//! dispatch, internal state). The caller provides the *visual* by
//! composing the props with a `div()` via `.apply(props)`:
//!
//! ```ignore
//! use yororen_ui::headless::button;
//! use yororen_ui_renderer::DefaultButton;
//!
//! // Pure headless — caller chooses every visual:
//! div().bg(red).rounded(8).apply(button("save", cx).on_click(...)).child("Save")
//!
//! // Default rendered — uses the installed GlobalTheme:
//! button("save", cx).on_click(...).default_render(cx)
//! ```
//!
//! The `headless/` module is the **only** way to construct
//! interactive elements. There is no "pre-rendered `Button`" struct
//! in `yororen-ui-core`; the visual lives in the optional
//! `yororen-ui-renderer` crate, and the palette lives in the
//! `yororen-ui-theme-*` crates.
//!
//! ## Composites
//!
//! Multi-part components (popover, modal, select, dropdown menu,
//! tooltip, …) own a piece of state on a `gpui::Entity`. Callers
//! create the entity with `cx.new(|_| XxxState::new())`, read/write
//! it via the returned `Entity<XxxState>`, and ask the renderer to
//! produce a default-styled view. There is no pre-baked
//! `Modal` / `Popover` struct that builds its own trigger and
//! content divs — the caller passes them in.

pub mod button;
pub mod checkbox;
pub mod icon_button;
pub mod label;
pub mod radio;
pub mod switch;
pub mod text_input;
pub mod toggle_button;
