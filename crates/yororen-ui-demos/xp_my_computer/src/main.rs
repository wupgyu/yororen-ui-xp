//! yororen-ui Windows XP My Computer demo
//!
//! Visual acceptance demo for Explorer-style chrome: Luna title bar
//! (Modal), menu bar, function bar, address bar, task pane, and icon
//! view. Functionality is intentionally static.
//!
//! Run with: `cargo run -p xp-my-computer-demo`

use std::borrow::Cow;

use gpui::{App, AppContext, Application, AssetSource, SharedString, px, size};
use rust_embed::Embed;
use yororen_ui::assets::{CompositeAssetSource, UiAsset};
use yororen_ui::xp_renderer::{self, XpAppWindow};

mod app;
mod menu_data;

#[derive(Embed)]
#[folder = "assets/"]
#[include = "windowsIcons/**/*"]
#[exclude = "*.DS_Store"]
struct DemoAsset;

impl AssetSource for DemoAsset {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::get(path).map(|f| f.data))
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| {
                if p.starts_with(path) {
                    Some(p.into())
                } else {
                    None
                }
            })
            .collect())
    }
}

fn main() {
    let app = Application::new().with_assets(CompositeAssetSource::new(DemoAsset, UiAsset));

    app.run(|cx: &mut App| {
        xp_renderer::install(cx);

        let options = XpAppWindow::window_options(cx, "My Computer", size(px(800.0), px(560.0)));
        let app_entity = cx.new(app::MyComputerApp::new);
        let _ = cx.open_window(options, |_, _cx| app_entity);
    });
}
