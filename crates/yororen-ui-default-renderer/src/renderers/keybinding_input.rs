//! `KeybindingInputRenderer` — visual side of `KeybindingInput`.
//!
//! The text input / keymap / IME pipeline plus the Idle /
//! Capturing mode layout live in
//! `yororen-ui-core/src/headless/keybinding_input.rs` (and the
//! shared helpers in `text_input_element.rs`). This module
//! only provides the `TokenKeybindingInputRenderer` default
//! impl.

use std::sync::Arc;

use gpui::{Hsla, Pixels, px};

use yororen_ui_core::renderer::keybinding_input::{
    KeybindingInputRenderState, KeybindingInputRenderer,
};
use yororen_ui_core::theme::Theme;

pub struct TokenKeybindingInputRenderer;

impl KeybindingInputRenderer for TokenKeybindingInputRenderer {
    fn bg(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.base").unwrap_or_default()
    }
    fn border(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn focus_border(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.focus").unwrap_or_default()
    }
    fn hover_border(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn active_border(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn kbd_bg(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.hover").unwrap_or_default()
    }
    fn kbd_fg(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.primary").unwrap_or_default()
    }
    fn min_height(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.input.min_height")
            .unwrap_or(0.0) as f32)
    }
    fn border_radius(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
}

pub fn arc_keybinding_input<T: KeybindingInputRenderer + 'static>(
    r: T,
) -> Arc<dyn KeybindingInputRenderer> {
    Arc::new(r)
}
