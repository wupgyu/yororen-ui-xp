//! Brutalist overlay renderers: `Modal`, `Popover`,
//! `DropdownMenu`, `Disclosure`.

use gpui::prelude::FluentBuilder;
use gpui::{
    App, Div, Hsla, InteractiveElement, ParentElement, Pixels, Stateful, Styled, div, px,
};
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

use crate::style::{BRUTAL_BORDER, BRUTAL_RADIUS, brutal_border_color, brutal_shadow_overlay};

// =====================================================================
// Modal
// =====================================================================

pub use yororen_ui_core::renderer::modal::{ModalRenderState, ModalRenderer};

pub struct BrutalModalRenderer;

// Inherent helpers — *not* part of the trait surface.
impl BrutalModalRenderer {
    pub fn scrim(&self, _: &ModalRenderState, _: &Theme) -> Hsla {
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.5,
        }
    }
    pub fn panel_bg(&self, _: &ModalRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.raised").unwrap_or(BRUTAL_BORDER)
    }
    pub fn panel_border(&self, _: &ModalRenderState, theme: &Theme) -> Hsla {
        brutal_border_color(theme)
    }
    pub fn panel_padding(&self, _: &ModalRenderState, theme: &Theme) -> Edges<Pixels> {
        let p = theme
            .get_number("tokens.control.modal.padding")
            .unwrap_or(24.0) as f32;
        Edges::all(px(p))
    }
    pub fn panel_border_radius(&self, _: &ModalRenderState, _: &Theme) -> Pixels {
        px(BRUTAL_RADIUS)
    }
    pub fn panel_shadow_alpha(&self, _: &ModalRenderState, _: &Theme) -> f32 {
        1.0
    }
}

impl ModalRenderer for BrutalModalRenderer {
    fn compose(
        &self,
        _props: &yororen_ui_core::headless::modal::ModalProps,
        cx: &App,
    ) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = ModalRenderState {};
        let panel_bg = self.panel_bg(&state, theme);
        let panel_border = self.panel_border(&state, theme);
        let pad = self.panel_padding(&state, theme);
        let r = self.panel_border_radius(&state, theme);
        let shadow = brutal_shadow_overlay(theme);
        // Brutalism Modal renderer paints *only* the panel
        // (bg / border / padding / radius / hard offset shadow).
        // The scrim and centering are the caller's responsibility
        // — same contract as `TokenModalRenderer` in the default
        // renderer.
        gpui::div()
            .bg(panel_bg)
            .border_color(panel_border)
            .border_2()
            .p(pad.top)
            .rounded(r)
            .shadow(vec![gpui::BoxShadow {
                color: shadow.color,
                blur_radius: shadow.blur,
                spread_radius: gpui::px(0.0),
                offset: gpui::Point {
                    x: gpui::px(0.0),
                    y: shadow.offset_y,
                },
            }])
    }
}

// =====================================================================
// Popover
// =====================================================================

pub use yororen_ui_core::renderer::popover::{PopoverRenderState, PopoverRenderer};

pub struct BrutalPopoverRenderer;

// Inherent helpers — *not* part of the trait surface.
impl BrutalPopoverRenderer {
    pub fn bg(&self, _: &PopoverRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.raised").unwrap_or(BRUTAL_BORDER)
    }
    pub fn border(&self, _: &PopoverRenderState, theme: &Theme) -> Hsla {
        brutal_border_color(theme)
    }
    pub fn shadow_alpha(&self, _: &PopoverRenderState, _: &Theme) -> f32 {
        1.0
    }
    pub fn border_radius(&self, _: &PopoverRenderState, _: &Theme) -> Pixels {
        px(BRUTAL_RADIUS)
    }
    pub fn offset(&self, _: &PopoverRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.popover.offset")
            .unwrap_or(8.0) as f32)
    }
}

impl PopoverRenderer for BrutalPopoverRenderer {
    fn compose(
        &self,
        props: &mut yororen_ui_core::headless::popover::PopoverProps,
        cx: &App,
    ) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = PopoverRenderState {};
        let bg = self.bg(&state, theme);
        let border = self.border(&state, theme);
        let r = self.border_radius(&state, theme);
        let alpha = self.shadow_alpha(&state, theme);
        let open = props.state.read(cx).is_open();
        let offset_px = self.offset(&state, theme);

        let mut outer = gpui::div().relative();

        if let Some(t) = props.trigger.take() {
            outer = outer.child(t);
        }

        if open
            && let Some(c) = props.content.take()
        {
            // Capture outside-clicks to close the popover.
            let state_for_close = props.state.clone();
            outer = outer.on_mouse_down_out(move |_ev, _window, cx| {
                state_for_close.update(cx, |s, _cx| s.close());
            });
            let shadow = crate::style::brutal_shadow_overlay(theme);
            let panel: Div = gpui::div()
                .absolute()
                .top(offset_px)
                .left_0()
                .bg(bg)
                .border_color(border)
                .border_2()
                .rounded(r)
                .shadow(vec![gpui::BoxShadow {
                    color: gpui::hsla(0.0, 0.0, 0.0, alpha),
                    blur_radius: gpui::px(0.0),
                    spread_radius: gpui::px(0.0),
                    offset: gpui::Point {
                        x: gpui::px(0.0),
                        y: shadow.offset_y,
                    },
                }])
                .occlude()
                .child(c);
            outer = outer.child(gpui::deferred(panel).with_priority(1));
        }

        outer
    }
}

// =====================================================================
// DropdownMenu
// =====================================================================

pub use yororen_ui_core::renderer::dropdown_menu::{DropdownMenuRenderState, DropdownMenuRenderer};

pub struct BrutalDropdownMenuRenderer;

// Inherent helpers — *not* part of the trait surface.
impl BrutalDropdownMenuRenderer {
    pub fn trigger_bg(&self, _: &DropdownMenuRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.bg")
            .unwrap_or(BRUTAL_BORDER)
    }
    pub fn trigger_hover_bg(&self, _: &DropdownMenuRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.hover_bg")
            .unwrap_or(BRUTAL_BORDER)
    }
    pub fn trigger_fg(&self, _: &DropdownMenuRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.fg")
            .unwrap_or(BRUTAL_BORDER)
    }
    pub fn min_height(&self, _: &DropdownMenuRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.dropdown_menu.min_height")
            .unwrap_or(44.0) as f32)
    }
    pub fn border_radius(&self, _: &DropdownMenuRenderState, _: &Theme) -> Pixels {
        px(BRUTAL_RADIUS)
    }
    pub fn chevron_rotation(&self, state: &DropdownMenuRenderState, _: &Theme) -> f32 {
        if state.open { 180.0 } else { 0.0 }
    }
}

impl DropdownMenuRenderer for BrutalDropdownMenuRenderer {
    fn compose(
        &self,
        props: &mut yororen_ui_core::headless::dropdown_menu::DropdownMenuProps,
        cx: &App,
    ) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = DropdownMenuRenderState {
            open: props.state.read(cx).is_open(),
        };
        let _border = self.trigger_bg(&state, theme);
        let _fg = self.trigger_fg(&state, theme);
        let r = self.border_radius(&state, theme);

        // Outer container is `relative` so the absolute panel
        // below is positioned relative to it.
        let mut outer = gpui::div().relative();

        // 1) Trigger — always rendered in normal flow.
        if let Some(t) = props.trigger.take() {
            outer = outer.child(t);
        }

        // 2) Body — only when open, floated with
        //    `gpui::deferred` so it paints over subsequent
        //    sibling cells in the gallery.
        if state.open
            && let Some(c) = props.content.take()
        {
            let shadow = crate::style::brutal_shadow_overlay(theme);
            let state_for_close = props.state.clone();
            // The body is a `menu` element which already paints
            // its own border + bg; the brutalism dropdown panel
            // only adds the brutalist hard offset shadow and
            // the click-outside dismissal. Avoid double borders
            // by NOT setting `border_2` / `border_color` here.
            let panel: Div = gpui::div()
                .absolute()
                .top(gpui::px(4.0))
                .left_0()
                .rounded(r)
                .shadow(vec![gpui::BoxShadow {
                    color: gpui::hsla(0.0, 0.0, 0.0, 1.0),
                    blur_radius: gpui::px(0.0),
                    spread_radius: gpui::px(0.0),
                    offset: gpui::Point {
                        x: gpui::px(0.0),
                        y: shadow.offset_y,
                    },
                }])
                .occlude()
                .on_mouse_down_out(move |_ev, _window, cx| {
                    state_for_close.update(cx, |s, _cx| s.close());
                })
                .child(c);
            outer = outer.child(gpui::deferred(panel).with_priority(1));
        }

        outer
    }
}

// =====================================================================
// Disclosure
// =====================================================================

pub use yororen_ui_core::renderer::disclosure::{DisclosureRenderState, DisclosureRenderer};

pub struct BrutalDisclosureRenderer;

// Inherent helpers — *not* part of the trait surface.
impl BrutalDisclosureRenderer {
    pub fn trigger_bg(&self, _: &DisclosureRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.bg")
            .unwrap_or(BRUTAL_BORDER)
    }
    pub fn trigger_fg(&self, _: &DisclosureRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.fg")
            .unwrap_or(BRUTAL_BORDER)
    }
    pub fn trigger_hover_bg(&self, _: &DisclosureRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.hover_bg")
            .unwrap_or(BRUTAL_BORDER)
    }
    pub fn min_height(&self, _: &DisclosureRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.disclosure.min_height")
            .unwrap_or(44.0) as f32)
    }
    pub fn border_radius(&self, _: &DisclosureRenderState, _: &Theme) -> Pixels {
        px(BRUTAL_RADIUS)
    }
    pub fn chevron_rotation(&self, state: &DisclosureRenderState, _: &Theme) -> f32 {
        if state.open { 90.0 } else { 0.0 }
    }
    pub fn body_padding(&self, _: &DisclosureRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.disclosure.padding")
            .unwrap_or(12.0) as f32)
    }
}

impl DisclosureRenderer for BrutalDisclosureRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::disclosure::DisclosureProps,
        cx: &App,
    ) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = DisclosureRenderState { open: props.open };
        let fg = self.trigger_fg(&state, theme);
        let chev_str = if props.open { "▼" } else { "▶" };
        // Lightweight trigger: chevron + title, no button-like
        // background / min-height / radius. Matches the default
        // renderer (a normal-weight flex row) so disclosure cells
        // look consistent with button / popover / dropdown cells.
        div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .text_color(fg)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(6.0))
                    .child(chev_str)
                    .child(props.title.clone()),
            )
    }
}

// =====================================================================
// Overlay
// =====================================================================

pub use yororen_ui_core::renderer::overlay::{OverlayRenderState, OverlayRenderer};

pub struct BrutalOverlayRenderer;

impl BrutalOverlayRenderer {
    pub fn scrim_color(&self, _state: &OverlayRenderState, theme: &Theme) -> Hsla {
        // Same fallback as the default renderer (50% black) so the
        // gallery shows a visible scrim even if the theme omits the
        // `surface.scrim` key.
        theme
            .get_color("surface.scrim")
            .unwrap_or_else(|| Hsla {
                h: 0.0,
                s: 0.0,
                l: 0.0,
                a: 0.5,
            })
    }
}

impl OverlayRenderer for BrutalOverlayRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::overlay::OverlayProps,
        cx: &App,
    ) -> gpui::Stateful<Div> {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = OverlayRenderState { open: props.open };
        let scrim = self.scrim_color(&state, theme);
        // `relative().size_full()` keeps the scrim within the
        // cell's box (the cell is now `position: relative`, see
        // `sections/mod.rs::cell`). The brutalism flavor is just
        // the scrim color — the overlay has no border / radius.
        div()
            .id(props.id.clone())
            .relative()
            .size_full()
            .bg(scrim)
            .when(!props.open, |el: Stateful<Div>| el.invisible())
    }
}
