//! `PopoverRenderer` — visual contract for `Popover`.
//!
//! Trait surface is just `compose`. Inherent helpers
//! (bg / border / shadow_alpha / border_radius / offset)
//! stay on the concrete renderer type.
//!
//! `compose` takes `&mut PopoverProps` so the renderer can
//! `take()` the stored `trigger` and `content` `AnyElement`s
//! (which are single-use, owned GPUI element boxes). The
//! headless `PopoverProps::render` is the only caller; it
//! passes the freshly-built `self` by mutable reference.

use std::any::Any;

use gpui::{App, Div};

use crate::headless::popover::PopoverProps;

#[derive(Clone, Copy, Debug, Default)]
pub struct PopoverRenderState {}

pub trait PopoverRenderer: Any + Send + Sync {
    fn compose(&self, props: &mut PopoverProps, cx: &App) -> Div;
}
