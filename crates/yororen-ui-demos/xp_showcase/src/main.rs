//! yororen-ui Windows XP Showcase Demo
//!
//! A single window rendered entirely by the XP (Luna) renderer:
//! gradient buttons, the green segmented progress bar, beveled
//! wells, white sunken inputs, and the classic `#ECE9D8` dialog
//! beige background. Install once at boot — the whole component
//! tree below is plain headless code with zero XP-specific API.
//!
//! Run with: `cargo run -p xp-showcase-demo`

use gpui::{App, AppContext, Application, WindowBounds, WindowOptions, px, size};

use yororen_ui::assets::UiAsset;
use yororen_ui::xp_renderer;

mod xp_app;

fn main() {
    let app = Application::new().with_assets(UiAsset);

    app.run(|cx: &mut App| {
        // Install the bundled Luna theme + all 55 XP renderers.
        xp_renderer::install(cx);

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(gpui::Bounds::centered(
                None,
                size(px(760.0), px(600.0)),
                cx,
            ))),
            ..Default::default()
        };
        let app_entity = cx.new(|_cx| xp_app::XpApp::new());
        let _ = cx.open_window(options, |_, _cx| app_entity);
    });
}
