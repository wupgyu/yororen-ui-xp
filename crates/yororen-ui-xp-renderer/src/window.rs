//! Full-window XP application chrome ("Explorer window").
//!
//! Bundles the plumbing an XP-style app window needs so a new app
//! gets the complete look with three calls instead of hand-rolled
//! setup:
//!
//! 1. [`XpAppWindow::window_options`] — transparent OS title bar
//!    (the Luna bar is drawn by the renderer) + opaque client area
//!    (a transparent host reads as a gutter around the drawn frame).
//! 2. [`XpAppWindow::new`] — an always-open [`ModalState`] carrying
//!    the window title, with no Escape / scrim dismissal.
//! 3. [`XpAppWindow::render`] — frame-colored host, Luna title bar
//!    with optional leading icon and min / max / close wired to the
//!    real OS window, native drag region, and the caller's content
//!    flush inside the 1px `#A09C8C` body border.
//!
//! The three-side blue frame itself is painted by
//! [`XpModalRenderer`](crate::renderers::overlays::XpModalRenderer)'s
//! full-window path (`body_padded = false`): the panel wears the
//! `xp.window.border_active / inactive` frame color so the 3px body
//! inset reads as the Luna window frame, not the beige dialog
//! surface.

use gpui::{
    App, Div, Entity, InteractiveElement, IntoElement, ParentElement, Pixels, SharedString, Size,
    Stateful, Styled, TitlebarOptions, WindowBackgroundAppearance, WindowBounds, WindowControlArea,
    WindowOptions, div, px,
};
use yororen_ui_core::headless::modal::{ModalCaption, ModalState, modal};
use yororen_ui_core::theme::ActiveTheme;

use crate::style::{window_border_active, xp_color, xp_number};

/// XP application window scaffold: Luna title bar with OS-wired
/// caption buttons over the three-side blue frame.
///
/// One per OS window; store it on the app entity and call
/// [`XpAppWindow::render`] from `Render::render`.
pub struct XpAppWindow {
    modal_state: Entity<ModalState>,
}

impl XpAppWindow {
    /// Always-open modal configured as window chrome: title set,
    /// no Escape / scrim dismissal.
    pub fn new(cx: &mut App, title: &str) -> Self {
        let modal_state = ModalState::new(cx);
        modal_state.update(cx, |s, _| {
            s.set_title(title);
            s.set_dismiss_on_escape(false);
            s.set_dismiss_on_scrim(false);
            s.open();
        });
        Self { modal_state }
    }

    /// `WindowOptions` matching the chrome: the OS title bar is
    /// transparent (the renderer draws the Luna bar) and the client
    /// area is opaque so no host-colored gutter can appear around
    /// the drawn frame.
    pub fn window_options(
        cx: &App,
        title: impl Into<SharedString>,
        window_size: Size<Pixels>,
    ) -> WindowOptions {
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(gpui::Bounds::centered(
                None,
                window_size,
                cx,
            ))),
            titlebar: Some(TitlebarOptions {
                title: Some(title.into()),
                appears_transparent: true,
                traffic_light_position: None,
            }),
            window_background: WindowBackgroundAppearance::Opaque,
            ..Default::default()
        }
    }

    /// Compose the chrome over `content`: frame-colored host, Luna
    /// title bar with optional `title_leading` icon, min / max /
    /// close wired to the real OS window controls, and a native
    /// drag region spanning the title bar minus the caption strip.
    pub fn render(
        &self,
        title_leading: Option<impl IntoElement>,
        content: impl IntoElement,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        // Match the active frame so any residual 1–2px OS/client
        // gutter reads as frame, not beige/gray.
        let frame = xp_color(theme, "xp.window.border_active", window_border_active());
        let bar_h = xp_number(theme, "xp.titlebar.height", 26.0);
        // Reserve the caption strip (3 buttons + gaps + right
        // padding) so the drag region never covers the buttons.
        let caption_reserve = xp_number(theme, "xp.caption.size", 21.0) * 3.0 + 8.0;

        let caption = ModalCaption::new()
            .on_minimize(|window, _cx| {
                window.minimize_window();
            })
            .on_maximize(|window, _cx| {
                window.zoom_window();
            })
            .on_close(|window, _cx| {
                window.remove_window();
            });

        let mut window = modal("xp-app-window", self.modal_state.clone())
            .caption(caption)
            .body_padded(false)
            .child(content);
        if let Some(leading) = title_leading {
            window = window.title_leading(leading);
        }

        div()
            .id("xp-app-window-host")
            .relative()
            .w_full()
            .h_full()
            .bg(frame)
            .child(window.render(cx))
            .child(
                div()
                    .id("xp-app-window-drag")
                    .absolute()
                    .top_0()
                    .left_0()
                    .right(px(caption_reserve))
                    .h(px(bar_h))
                    .window_control_area(WindowControlArea::Drag),
            )
    }
}
