//! `KeybindingInputRenderer` — visual side of `KeybindingInput`.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::renderers::spec::Edges;
use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct KeybindingInputRenderState {
    pub capturing: bool,
    pub disabled: bool,
    pub custom_bg: Option<Hsla>,
    pub custom_border: Option<Hsla>,
    pub custom_focus_border: Option<Hsla>,
    pub custom_fg: Option<Hsla>,
}

pub trait KeybindingInputRenderer: Any + Send + Sync {
    fn bg(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Hsla;
    fn border(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Hsla;
    fn focus_border(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Hsla;
    fn kbd_bg(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Hsla;
    fn kbd_fg(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Hsla;
    fn kbd_padding(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Edges<Pixels>;
    fn kbd_min_width(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Pixels;
    fn min_height(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Pixels;
    fn border_radius(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Pixels;
    fn icon_size(&self, state: &KeybindingInputRenderState, theme: &Theme) -> Pixels;
}

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
    fn kbd_bg(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.hover").unwrap_or_default()
    }
    fn kbd_fg(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.primary").unwrap_or_default()
    }
    fn kbd_padding(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            gpui::px(theme.get_number("tokens.control.keybinding_input.kbd_padding_x").unwrap_or(0.0) as f32),
            gpui::px(theme.get_number("tokens.control.keybinding_input.kbd_padding_y").unwrap_or(0.0) as f32),
        )
    }
    fn kbd_min_width(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.keybinding_input.kbd_min_width").unwrap_or(0.0) as f32)
    }
    fn min_height(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.input.min_height").unwrap_or(0.0) as f32)
    }
    fn border_radius(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
    fn icon_size(&self, _state: &KeybindingInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.keybinding_input.icon_size").unwrap_or(0.0) as f32)
    }
}

pub fn arc_keybinding_input<T: KeybindingInputRenderer + 'static>(
    r: T,
) -> Arc<dyn KeybindingInputRenderer> {
    Arc::new(r)
}
