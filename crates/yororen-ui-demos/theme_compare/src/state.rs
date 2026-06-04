//! Global state for the theme-compare demo.

use gpui::{App, AppContext, Entity, Global};

/// Whether the demo is currently showing the mini theme. `with_theme`
/// was removed (P0-2) so the demo flips the global theme instead of
/// per-element overrides.
pub struct ThemeCompareState {
    pub uses_mini: Entity<bool>,
}

impl Global for ThemeCompareState {}

impl ThemeCompareState {
    pub fn new(cx: &mut App) -> Self {
        Self {
            uses_mini: cx.new(|_| false),
        }
    }
}
