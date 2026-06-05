//! Accessibility (A11y) module for Yororen UI.
//!
//! This module provides accessibility utilities including:
//! - Focus management components (`FocusTrap`, `FocusTrapState`)
//! - Click-outside detection (`ClickOutsideGuard`,
//!   `ClickOutsideCapture`, `on_click_outside`)
//! - Body scroll lock (`ScrollLockGuard`)
//! - Keyboard navigation helpers (`FocusableList`, `cycle_focus`,
//!   `FocusRing`)
//!
//! # ARIA / screen reader support
//!
//! Yororen UI does **not** ship a screen-reader bridge on top of
//! `gpui-ce 0.3.3`. The underlying `gpui-ce` 0.3.3 release has no
//! public API to forward ARIA roles / labels to the OS
//! accessibility tree (NSAccessibility / UIA / AT-SPI), and
//! `gpui-ce`'s in-progress `main` branch which does have a full
//! [AccessKit]-based bridge is not yet published on crates.io. See
//! the [Roadmap wiki page][wiki-roadmap] for the up-to-date plan.
//!
//! [AccessKit]: https://accesskit.dev/
//! [wiki-roadmap]: https://github.com/MeowLynxSea/yororen-ui/wiki/Roadmap

mod click_outside;
mod focus_trap;
mod keyboard_nav;
mod scroll_lock;

pub use click_outside::*;
pub use focus_trap::*;
pub use keyboard_nav::*;
pub use scroll_lock::*;
