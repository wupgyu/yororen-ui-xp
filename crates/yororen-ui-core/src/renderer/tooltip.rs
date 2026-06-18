//! `TooltipRenderer` — visual contract for `Tooltip`.
//!
//! Trait surface is just `compose`. Inherent helpers
//! (bg / fg / padding / font_size / border_radius) stay
//! on the concrete renderer type.
//!
//! `compose` takes `&mut TooltipProps` so the renderer can
//! `take()` the stored `trigger` `AnyElement` (single-use).

use std::any::Any;

use gpui::{App, Div};

use crate::headless::tooltip::TooltipProps;

#[derive(Clone, Copy, Debug, Default)]
pub struct TooltipRenderState {
    pub has_custom_bg: bool,
    pub has_custom_fg: bool,
}

pub trait TooltipRenderer: Any + Send + Sync {
    fn compose(&self, props: &mut TooltipProps, cx: &App) -> Div;
}
