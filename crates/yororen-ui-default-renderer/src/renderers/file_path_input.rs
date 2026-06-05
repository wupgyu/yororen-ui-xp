//! `FilePathInputRenderer` — visual side of `FilePathInput`.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::renderers::spec::Edges;
use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct FilePathInputRenderState {
    pub disabled: bool,
    pub focused: bool,
    pub custom_bg: Option<Hsla>,
    pub custom_border: Option<Hsla>,
    pub custom_focus_border: Option<Hsla>,
    pub custom_fg: Option<Hsla>,
}

pub trait FilePathInputRenderer: Any + Send + Sync {
    fn bg(&self, state: &FilePathInputRenderState, theme: &Theme) -> Hsla;
    fn border(&self, state: &FilePathInputRenderState, theme: &Theme) -> Hsla;
    fn focus_border(&self, state: &FilePathInputRenderState, theme: &Theme) -> Hsla;
    fn button_bg(&self, state: &FilePathInputRenderState, theme: &Theme) -> Hsla;
    fn button_fg(&self, state: &FilePathInputRenderState, theme: &Theme) -> Hsla;
    fn min_height(&self, state: &FilePathInputRenderState, theme: &Theme) -> Pixels;
    fn padding(&self, state: &FilePathInputRenderState, theme: &Theme) -> Edges<Pixels>;
    fn action_gap(&self, state: &FilePathInputRenderState, theme: &Theme) -> Pixels;
    fn border_radius(&self, state: &FilePathInputRenderState, theme: &Theme) -> Pixels;
    fn icon_size(&self, state: &FilePathInputRenderState, theme: &Theme) -> Pixels;
}

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
    fn button_bg(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("action.neutral.bg").unwrap_or_default()
    }
    fn button_fg(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Hsla {
        theme.get_color("action.neutral.fg").unwrap_or_default()
    }
    fn min_height(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.file_path_input.min_height").unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            gpui::px(theme.get_number("tokens.control.file_path_input.horizontal_padding").unwrap_or(0.0) as f32),
            gpui::px(theme.get_number("tokens.control.input.vertical_padding").unwrap_or(0.0) as f32),
        )
    }
    fn action_gap(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.file_path_input.action_gap").unwrap_or(0.0) as f32)
    }
    fn border_radius(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.radii.md").unwrap_or(0.0) as f32)
    }
    fn icon_size(&self, _state: &FilePathInputRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.file_path_input.icon_size").unwrap_or(0.0) as f32)
    }
}

pub fn arc_file_path_input<T: FilePathInputRenderer + 'static>(
    r: T,
) -> Arc<dyn FilePathInputRenderer> {
    Arc::new(r)
}
