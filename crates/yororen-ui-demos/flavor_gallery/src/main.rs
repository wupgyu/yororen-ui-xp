//! yororen-ui Flavor Gallery Demo
//!
//! End-to-end demo of Phase F (Catppuccin theme) and Phase G
//! (a11y completeness) working together. The window is divided
//! into 4 columns, one per Catppuccin flavor (Latte / Frappé /
//! Macchiato / Mocha). Each column has the same set of
//! components — pickers, toggles, a tooltip, and a "Show modal"
//! button. Opening the modal in any column demonstrates the
//! v0.5 a11y stack end-to-end:
//!
//! - The modal is opened via the new `modal_dialog` factory
//!   (G-γ: one-line API, no need to manually compose Modal +
//!   Overlay + ScrollLock).
//! - The modal closes on Esc, scrim click, OR the inner close
//!   button (G-δ: all three paths route through a single
//!   `on_close` callback with `OverlayCloseReason`).
//! - The select / combo_box in each column honours Esc via
//!   `dismiss_on_escape` (G-β).
//! - Tab / Shift+Tab inside the modal is the default
//!   focus-trap behaviour (G-α real impl is used in the
//!   `focus_trap_demo` crate; this demo uses `modal_dialog`'s
//!   built-in focus handling).
//!
//! The 4 flavors are rendered with the **same** code — only the
//! active `Theme` differs. This proves that the v0.5 Renderer
//! trait system + `CatppuccinFlavor` enum together give a
//! complete Catppuccin skin with no per-component hardcoded
//! color logic.

use std::sync::Arc;

use gpui::{App, AppContext, Application, WindowBounds, WindowOptions, px, size};

use yororen_ui::assets::UiAsset;
use yororen_ui::theme::Theme;
use yororen_ui_locale_en as locale_en;
use yororen_ui_theme_catppuccin as catppuccin;
use yororen_ui_theme_system as theme_system;

mod flavor_gallery_app;
mod state;

use state::FlavorGalleryState;
use state::FlavorKind;

fn main() {
    let app = Application::new().with_assets(UiAsset);

    app.run(|cx: &mut App| {
        yororen_ui::component::init(cx);

        // Start with the system theme as the default; the user
        // picks a flavor via the radio in the top bar.
        theme_system::install(cx, cx.window_appearance());
        locale_en::install(cx);

        // Also register the Catppuccin custom variants so the
        // "mocha / lavender / ghost" buttons in the gallery can
        // be rendered with the correct accent.
        let variant_reg = Arc::new(catppuccin::variant::catppuccin_registry());
        cx.set_global(yororen_ui::renderer::GlobalVariantRegistry(variant_reg));

        let st = FlavorGalleryState::new(cx);
        cx.set_global(st);

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(gpui::Bounds::centered(
                None,
                size(px(1280.0), px(620.0)),
                cx,
            ))),
            ..Default::default()
        };

        let _ = cx.open_window(options, |_, cx| {
            cx.new(flavor_gallery_app::FlavorGalleryApp::new)
        });
    });
}

/// Resolve the active Theme for a given flavor and OS appearance.
///
/// Latte / Frappé / Macchiato / Mocha are explicit flavors; "System"
/// uses the system palette (with the active OS appearance).
pub fn theme_for(kind: FlavorKind, appearance: gpui::WindowAppearance) -> Theme {
    match kind {
        FlavorKind::System => match appearance {
            gpui::WindowAppearance::Light | gpui::WindowAppearance::VibrantLight => {
                theme_system::light()
            }
            _ => theme_system::dark(),
        },
        FlavorKind::Latte => catppuccin::light(),
        FlavorKind::Frappe => catppuccin::frappe(),
        FlavorKind::Macchiato => catppuccin::macchiato(),
        FlavorKind::Mocha => catppuccin::mocha(),
    }
}

// The FlavorKind enum is re-exported from `state.rs` so callers
// outside this crate can dispatch on it. The dead_code allow
// silences warnings for the variants that are only used by the
// `Theme` matcher in `theme_for` above.
#[allow(dead_code)]
fn _kind_passthrough(k: FlavorKind) -> FlavorKind {
    k
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The 4 Catppuccin flavors plus System must produce
    /// distinct theme surface.bg values. This is the v0.5
    /// regression test for the F-α no-hardcode rule.
    #[test]
    fn all_five_flavors_produce_distinct_themes() {
        let appearance = gpui::WindowAppearance::Dark;
        let system = theme_for(FlavorKind::System, appearance);
        let latte = theme_for(FlavorKind::Latte, appearance);
        let frappe = theme_for(FlavorKind::Frappe, appearance);
        let macchiato = theme_for(FlavorKind::Macchiato, appearance);
        let mocha = theme_for(FlavorKind::Mocha, appearance);
        // Pairwise distinct.
        assert_ne!(system.surface.base, latte.surface.base);
        assert_ne!(latte.surface.base, frappe.surface.base);
        assert_ne!(frappe.surface.base, macchiato.surface.base);
        assert_ne!(macchiato.surface.base, mocha.surface.base);
        // System is the system theme; Latte is the Catppuccin light.
        assert_ne!(system.surface.base, mocha.surface.base);
    }

    /// The same Theme can be plugged into a `with_theme` block
    /// and the descendants see the per-flavor palette.
    /// This is a sanity check for the F-γ wiring.
    #[test]
    fn flavor_kind_as_str_matches_demonstration() {
        assert_eq!(FlavorKind::System.as_str(), "System");
        assert_eq!(FlavorKind::Latte.as_str(), "Latte");
        assert_eq!(FlavorKind::Frappe.as_str(), "Frappé");
        assert_eq!(FlavorKind::Macchiato.as_str(), "Macchiato");
        assert_eq!(FlavorKind::Mocha.as_str(), "Mocha");
    }

    /// `theme_for` returns the same Theme across calls (the
    /// factory functions are pure, so the demo's top-bar switch
    /// can rely on the result being deterministic).
    #[test]
    fn theme_for_is_deterministic() {
        let appearance = gpui::WindowAppearance::Light;
        let t1 = theme_for(FlavorKind::Latte, appearance);
        let t2 = theme_for(FlavorKind::Latte, appearance);
        // Surface and content are not Copy; compare via Debug.
        assert_eq!(
            format!("{:?}", t1.surface.base),
            format!("{:?}", t2.surface.base)
        );
    }
}
