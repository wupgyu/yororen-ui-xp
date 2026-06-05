//! `GlobalTheme` — the single process-global theme accessor.
//!
//! Apps install a `Theme` once at boot via [`install`]; from
//! then on, `cx.theme()` returns an `&Arc<Theme>` so renderers
//! can read paths without cloning the whole tree.
//!
//! Renderers do **not** call `cx.theme()` directly — they
//! receive a `&Theme` argument from the component's render
//! path. `cx.theme()` is the way the component (or a glue
//! helper) gets a `Theme` reference in the first place.
//!
//! ```ignore
//! fn main() {
//!     yororen_ui_default_renderer::install(cx); // registers XxxRenderer impls
//!     let json = include_str!("../themes/system-light.json");
//!     yororen_ui_core::theme::install(cx, Theme::from_json(json)?);
//! }
//! ```

use std::sync::Arc;

use gpui::{App, Context, Global};

use super::Theme;

/// `cx.set_global(GlobalTheme(arc))` is the storage slot for
/// the active theme. Wrapping the `Arc` in a newtype keeps the
/// `cx.global::<GlobalTheme>()` projection namespaced.
pub struct GlobalTheme(pub Arc<Theme>);

impl Global for GlobalTheme {}

/// Install `theme` as the global. Replaces any previously
/// installed theme.
pub fn install(cx: &mut App, theme: Theme) {
    cx.set_global(GlobalTheme(Arc::new(theme)));
}

/// Read-only accessor for the active theme. Implemented for
/// [`App`] so `cx.theme()` works inside any render path that
/// already has an `&App` (e.g. via `&Window` -> `cx.app()`).
///
/// Returns `&Arc<Theme>` (not `&Theme`) so callers can cheaply
/// clone the `Arc` to move it into a `'static` closure or pass
/// it across threads without deep-cloning the underlying
/// `serde_json::Value` tree.
pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        &self.global::<GlobalTheme>().0
    }
}

// `Context<T>` is the closure parameter passed to
// `Render::render(&mut self, _: &mut Window, cx: &mut Context<Self>)`.
// It derefs to `App` for global access, so the same
// `cx.theme()` API works inside render closures. Without this
// impl, render bodies would have to do
// `&*cx as &App` casts just to read the theme.
impl<'a, T> ActiveTheme for Context<'a, T> {
    fn theme(&self) -> &Arc<Theme> {
        &self.global::<GlobalTheme>().0
    }
}
