//! 55 component renderers grouped by visual category.
//!
//! Each submodule implements the `XxxRenderer` traits from
//! `yororen_ui_default_renderer::renderers` with Windows XP (Luna)
//! visual values: 3px rounded corners, blue gradients, beveled
//! borders, green segmented progress chunks, and Tahoma typography.
//!
//! Colors are read from the open theme (`xp.*` extension paths with
//! `style` constants as fallbacks); geometry lives in `style` and
//! `tokens.control.*` so the 55 renderers stay in stylistic lockstep.

pub mod actions;
pub mod controls;
pub mod display;
pub mod inputs;
pub mod lists;
pub mod notifications;
pub mod overlays;
pub mod surfaces;
