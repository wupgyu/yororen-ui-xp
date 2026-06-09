//! Neo-Brutalism style renderer for yororen-ui.
//!
//! Implements all 38 `XxxRenderer` traits with sharp corners, thick
//! black borders, hard offset shadows, and monospace typography.
//!
//! Two bundled themes: `brutalism-light.json` and `brutalism-dark.json`.
//! Use [`install_with_default_theme`] to get the light theme, or
//! [`install`] to pick light/dark by system appearance.
//!
//! ```ignore
//! use yororen_ui_brutalism_renderer as brutalism;
//! brutalism::install(cx);
//! ```

// The crate is built up across several commits; until every
// `XxxRenderer` is wired in, individual helpers and constants
// will appear unused.
#![allow(dead_code)]

mod style;

pub mod renderers;

use gpui::{App, WindowAppearance};
use yororen_ui_core::theme::{Theme, install as install_theme};

const BRUTAL_LIGHT: &str = include_str!("../themes/brutalism-light.json");
const BRUTAL_DARK: &str = include_str!("../themes/brutalism-dark.json");

/// Install the brutalism renderer with a theme matching the system
/// appearance.
pub fn install(cx: &mut App) {
    install_with(cx, brutal_theme_for(cx.window_appearance()));
}

/// Install the brutalism renderer with the bundled light theme
/// (regardless of system appearance).
pub fn install_with_default_theme(cx: &mut App) {
    let theme = Theme::from_json(BRUTAL_LIGHT).expect("brutalism-light.json is valid");
    install_with(cx, theme);
}

/// Install the brutalism renderer with a custom theme.
pub fn install_with(cx: &mut App, theme: Theme) {
    install_theme(cx, theme);
    register_brutal_renderers(cx);
}

fn brutal_theme_for(appearance: WindowAppearance) -> Theme {
    let json = match appearance {
        WindowAppearance::Dark | WindowAppearance::VibrantDark => BRUTAL_DARK,
        _ => BRUTAL_LIGHT,
    };
    Theme::from_json(json).expect("brutalism theme json is valid")
}

fn register_brutal_renderers(_cx: &mut App) {
    // Renderers are registered in subsequent commits.
    // Currently the brutalism crate is a skeleton — install() above
    // sets the theme but registers no component renderers, so the
    // default-renderer's `TokenXxxRenderer` impls are still in
    // effect for every component.
}
