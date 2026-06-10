//! `FilePathInputRenderer` — visual side of `FilePathInput`.
//!
//! The text input / keymap / IME pipeline plus the leading
//! folder icon and trailing browse-button layout live in
//! `yororen-ui-core/src/headless/file_path_input.rs` (and the
//! shared helpers in `text_input_element.rs`). This module
//! only provides the `TokenFilePathInputRenderer` default impl.

use std::sync::Arc;

use gpui::{Hsla, Pixels, px};

use yororen_ui_core::renderer::file_path_input::{
    FilePathInputRenderState, FilePathInputRenderer,
};
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

pub struct TokenFilePathInputRenderer;

impl FilePathInputRenderer for TokenFilePathInputRenderer {
    fn bg(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.base").unwrap_or_default()
    }
    fn border(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn focus_border(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.focus").unwrap_or_default()
    }
    fn hover_border(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.muted").unwrap_or_default()
    }
    fn active_border(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.default").unwrap_or_default()
    }
    fn button_bg(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        // Match the input surface so the icon doesn't compete
        // with the typed path.
        theme.get_color("surface.base").unwrap_or_default()
    }
    fn button_fg(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("content.primary").unwrap_or_default()
    }
    fn button_hover_bg(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.hover").unwrap_or_default()
    }
    fn min_height(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.file_path_input.min_height")
            .unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            px(theme
                .get_number("tokens.control.file_path_input.horizontal_padding")
                .unwrap_or(0.0) as f32),
            px(theme
                .get_number("tokens.control.input.vertical_padding")
                .unwrap_or(0.0) as f32),
        )
    }
    fn action_gap(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.file_path_input.action_gap")
            .unwrap_or(0.0) as f32)
    }
    fn border_radius(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
    fn icon_size(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.file_path_input.icon_size")
            .unwrap_or(0.0) as f32)
    }
}

pub fn arc_file_path_input<T: FilePathInputRenderer + 'static>(
    r: T,
) -> Arc<dyn FilePathInputRenderer> {
    Arc::new(r)
}
