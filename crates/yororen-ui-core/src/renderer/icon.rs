//! `IconRenderer` — visual side of `Icon`.

use std::sync::Arc;

use gpui::{Hsla, Pixels};

use crate::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct IconRenderState {
    /// `true` if the user supplied `.color(...)`.
    pub has_custom_color: bool,
    /// Icon size preset (or `None` for user-supplied `.size(...)`).
    pub size_preset: Option<IconSizePreset>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconSizePreset {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

pub trait IconRenderer: Send + Sync {
    fn color(&self, state: &IconRenderState, theme: &Theme) -> Hsla;
    fn size(&self, state: &IconRenderState, theme: &Theme) -> Pixels;
    fn size_xs(&self, state: &IconRenderState, theme: &Theme) -> Pixels;
    fn size_sm(&self, state: &IconRenderState, theme: &Theme) -> Pixels;
    fn size_md(&self, state: &IconRenderState, theme: &Theme) -> Pixels;
    fn size_lg(&self, state: &IconRenderState, theme: &Theme) -> Pixels;
    fn size_xl(&self, state: &IconRenderState, theme: &Theme) -> Pixels;
}

pub struct TokenIconRenderer;

impl IconRenderer for TokenIconRenderer {
    fn color(&self, _state: &IconRenderState, theme: &Theme) -> Hsla {
        theme.content.primary
    }
    fn size(&self, state: &IconRenderState, theme: &Theme) -> Pixels {
        match state.size_preset {
            Some(IconSizePreset::Xs) => theme.tokens.sizes.icon_xs,
            Some(IconSizePreset::Sm) => theme.tokens.sizes.icon_sm,
            Some(IconSizePreset::Md) => theme.tokens.sizes.icon_md,
            Some(IconSizePreset::Lg) => theme.tokens.sizes.icon_lg,
            Some(IconSizePreset::Xl) => theme.tokens.sizes.icon_xl,
            None => theme.tokens.sizes.icon_md,
        }
    }
    fn size_xs(&self, _state: &IconRenderState, theme: &Theme) -> Pixels {
        theme.tokens.sizes.icon_xs
    }
    fn size_sm(&self, _state: &IconRenderState, theme: &Theme) -> Pixels {
        theme.tokens.sizes.icon_sm
    }
    fn size_md(&self, _state: &IconRenderState, theme: &Theme) -> Pixels {
        theme.tokens.sizes.icon_md
    }
    fn size_lg(&self, _state: &IconRenderState, theme: &Theme) -> Pixels {
        theme.tokens.sizes.icon_lg
    }
    fn size_xl(&self, _state: &IconRenderState, theme: &Theme) -> Pixels {
        theme.tokens.sizes.icon_xl
    }
}

pub fn arc_icon<T: IconRenderer + 'static>(r: T) -> Arc<dyn IconRenderer> {
    Arc::new(r)
}
