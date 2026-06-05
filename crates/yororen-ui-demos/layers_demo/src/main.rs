//! yororen-ui Layers Demo
//!
//! Three side-by-side panels showing the v0.3 three-layer
//! architecture in action:
//!
//! 1. **Headless only** — every visual decision is the
//!    caller's; the `headless::button` returns a `ButtonProps`
//!    that the caller composes with a raw `div()`.
//! 2. **Headless + default-renderer** — same `headless::button`,
//!    but `.default_render(cx)` reads the registered
//!    `TokenButtonRenderer` and applies the default look.
//! 3. **Headless + mini-renderer override** — same `headless::button`,
//!    but a custom `MiniButtonRenderer` is installed on top
//!    of the default; the button picks up the mini's `themeColor`
//!    while the surrounding label / div still come from
//!    default-renderer.
//!
//! The demo depends on the `yororen-ui-mini-renderer` crate
//! directly (not via the `mini` feature) so the install
//! pattern is visible — this is the third-party renderer
//! install shape.

use gpui::{App, AppContext, Application, WindowBounds, WindowOptions, px, size};

use yororen_ui::assets::UiAsset;
use yororen_ui_default_renderer as default_renderer;
use yororen_ui_mini_renderer as mini;

mod layers_app;

fn main() {
    let app = Application::new().with_assets(UiAsset);

    app.run(|cx: &mut App| {
        // Default renderer first (38 token-styled components).
        default_renderer::install(cx, cx.window_appearance());
        // Mini on top — overrides the 4 components it covers.
        mini::install(cx);

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(
                gpui::Bounds::centered(None, size(px(1200.0), px(500.0)), cx),
            )),
            ..Default::default()
        };
        let _ = cx.open_window(options, |_, cx| cx.new(|_cx| layers_app::LayersApp::new()));
    });
}
