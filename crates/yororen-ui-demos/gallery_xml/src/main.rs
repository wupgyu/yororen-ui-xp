//! Gallery Demo (XML edition) — a 1:1 XML-driven port of
//! `gallery_demo`. The layout lives in `src/ui/*.xml`; the
//! runtime renderer / locale switching and composite state
//! seeding live in Rust scaffolding.

mod controller;
mod i18n;
mod notifications_host;
mod state;
mod theme_switcher;
mod view;

use gpui::{
    App, AppContext, Application, InteractiveElement, IntoElement, WindowBounds, WindowOptions,
    div, px, size,
};

use yororen_ui::assets::UiAsset;
use yororen_ui::notification::center::NotificationCenter;

use crate::controller::Controller;
use crate::state::{GalleryState, StateRef};
use crate::view::GalleryApp;

/// Trivial custom widget used to exercise
/// `register_xml_component!` — the extension hook for
/// adding tags without touching the codegen schema.
fn render_counter_widget(id: String, _cx: &mut gpui::App) -> gpui::AnyElement {
    div().id(id).into_any_element()
}

yororen_ui::register_xml_component!(CounterWidget => render_counter_widget);

fn main() {
    let app = Application::new().with_assets(UiAsset);

    app.run(|cx: &mut App| {
        // 1. Install the default renderer + theme (light).
        //    The view will reinstall per render so toolbar
        //    toggles take effect immediately.
        theme_switcher::install_renderer(
            cx,
            theme_switcher::RendererKind::default(),
            theme_switcher::DarkMode::default(),
        );

        // 2. Bind the text-input keymap once (idempotent).
        yororen_ui::headless::text_input::init(cx);

        // 3. Install the notification center (toast /
        //    notification trigger from the toolbar).
        cx.set_global(NotificationCenter::new());

        // 4. Install English locale + the demo's own translations.
        crate::i18n::install_for_locale(cx, crate::state::LocaleChoice::En);

        // 5. Build the state + the controller that owns it.
        let state = cx.new(|cx| GalleryState::new_data(cx));
        let controller = Controller::new(state.clone(), cx);

        // 6. Make the state available to the view as a global.
        cx.set_global(StateRef { state });

        // 7. Open the main window.
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(gpui::Bounds::centered(
                None,
                size(px(1280.0), px(900.0)),
                cx,
            ))),
            ..Default::default()
        };
        let _ = cx.open_window(options, |_, cx| {
            cx.new(|cx| GalleryApp::new(cx, controller))
        });
    });
}
