//! XP (Luna) overlay renderers: `Modal`, `Popover`,
//! `DropdownMenu`, `Disclosure`, `Overlay`, `Menu`.
//!
//! The `Modal` is the signature XP window: vertical blue
//! multi-stop title bar (stacked 2-stop bands) with a white bold
//! caption and optional min / max / close caption buttons, beige
//! (`#ECE9D8`) content area, 1px frame (active `#0058E6` /
//! inactive `#98A8C0`), 8px rounded corners and a soft overlay
//! shadow. Popovers, dropdowns and menus are white panels with a
//! 1px bevel border, 3px corners and the same soft shadow;
//! hovered menu rows flip to the solid selection blue with white
//! text.

use gpui::prelude::FluentBuilder;
use gpui::{
    App, Background, CursorStyle, Div, ElementId, FontWeight, Hsla, InteractiveElement,
    ParentElement, Pixels, Stateful, StatefulInteractiveElement, Styled, div, px, relative,
};
use yororen_ui_core::animation::SlideDirection;
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

use crate::style::{
    XP_BORDER_WIDTH, XP_RADIUS, bevel_inner_dark, caption_border, caption_close_from,
    caption_close_to, caption_from, caption_to, dialog_bg, hsl_fallback, lighten, menu_hover_bg,
    menu_hover_fg, selection_hover_bg, shadow_vec, titlebar_bands, titlebar_inactive_gradient,
    vgrad, window_body_border, window_border_active, window_border_inactive, xp_color, xp_number,
    xp_shadow_overlay,
};
use yororen_ui_default_renderer::animation::{AnimatedPresenceElement, fade_in_on_mount};

// =====================================================================
// Modal
// =====================================================================

pub use yororen_ui_core::renderer::modal::{ModalRenderState, ModalRenderer};

pub struct XpModalRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpModalRenderer {
    pub fn scrim(&self, _: &ModalRenderState, _: &Theme) -> Hsla {
        Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.5,
        }
    }
    /// XP windows use the classic dialog beige, not raised white.
    pub fn panel_bg(&self, _: &ModalRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.base").unwrap_or(dialog_bg())
    }
    /// 1px window frame: active `#0058E6`, inactive `#98A8C0`.
    pub fn panel_border(&self, _: &ModalRenderState, theme: &Theme, active: bool) -> Hsla {
        if active {
            xp_color(theme, "xp.window.border_active", window_border_active())
        } else {
            xp_color(theme, "xp.window.border_inactive", window_border_inactive())
        }
    }
    pub fn panel_padding(&self, _: &ModalRenderState, theme: &Theme) -> Edges<Pixels> {
        let p = theme
            .get_number("tokens.control.modal.padding")
            .unwrap_or(12.0) as f32;
        Edges::all(px(p))
    }
    pub fn panel_border_radius(&self, _: &ModalRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.modal.radius")
            .unwrap_or(8.0) as f32)
    }
    /// Luna title-bar strip height (26px).
    pub fn title_bar_height(&self, _: &ModalRenderState, theme: &Theme) -> Pixels {
        px(xp_number(theme, "xp.titlebar.height", 26.0))
    }
    /// White caption text on the blue gradient.
    pub fn title_bar_fg(&self, _: &ModalRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.titlebar.text", hsl_fallback(0xFFFFFF))
    }
    /// Active title bar: `(fraction, from, to)` bands stacked to
    /// approximate the 5-stop vertical Luna gradient.
    pub fn title_bar_bands(&self, _: &ModalRenderState, theme: &Theme) -> [(f32, Hsla, Hsla); 4] {
        titlebar_bands(theme)
    }
    /// Inactive title bar: plain 2-stop vertical gradient.
    pub fn title_bar_inactive_bg(&self, _: &ModalRenderState, theme: &Theme) -> Background {
        titlebar_inactive_gradient(theme)
    }
    pub fn title_font_size(&self, _: &ModalRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.modal.title_size")
            .unwrap_or(13.0) as f32)
    }
    /// Caption button faces: `(from, to, close_from, close_to)`.
    pub fn caption_faces(&self, _: &ModalRenderState, theme: &Theme) -> (Hsla, Hsla, Hsla, Hsla) {
        (
            xp_color(theme, "xp.caption.from", caption_from()),
            xp_color(theme, "xp.caption.to", caption_to()),
            xp_color(theme, "xp.caption.close_from", caption_close_from()),
            xp_color(theme, "xp.caption.close_to", caption_close_to()),
        )
    }
    /// Inner border of the window body (`#A09C8C`).
    pub fn body_border(&self, _: &ModalRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.window.body_border", window_body_border())
    }
}

impl ModalRenderer for XpModalRenderer {
    fn compose(&self, props: &mut yororen_ui_core::headless::modal::ModalProps, cx: &App) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = ModalRenderState {};
        let active = props.window_active;
        let panel_bg = self.panel_bg(&state, theme);
        let panel_border = self.panel_border(&state, theme, active);
        let pad = self.panel_padding(&state, theme);
        let r = self.panel_border_radius(&state, theme);

        if !props.state.read(cx).is_visible() {
            return div();
        }

        let title = props.state.read(cx).title.clone();
        let children = std::mem::take(&mut props.children);
        let caption = props.caption.clone();
        // The XP Modal renderer paints *only* the window (frame /
        // title bar / content area / soft overlay shadow). The
        // scrim and centering are the caller's responsibility —
        // same contract as `TokenModalRenderer` in the default
        // renderer.
        // Full-window chrome (Explorer) must fill the host window;
        // dialog-style modals size to their content.
        let body_padded = props.body_padded;
        // Full-window chrome paints the panel itself in the frame
        // color so the 3px body inset below reads as the Luna window
        // frame (active blue / inactive gray-blue) instead of the
        // beige dialog surface. Dialog-style modals keep the beige
        // panel from the xp.css recipe.
        let panel_surface = if body_padded { panel_bg } else { panel_border };
        let mut panel = gpui::div()
            .bg(panel_surface)
            .border_color(panel_border)
            .border(px(XP_BORDER_WIDTH))
            // css `.xp-window`: `border-radius: 8px 8px 0 0` —
            // only the top corners are rounded.
            .rounded_tl(r)
            .rounded_tr(r)
            .overflow_hidden()
            .flex()
            .flex_col()
            .w_full();
        if !body_padded {
            // 3px frame inset (css `.xp-window-body` margins) as panel
            // padding so the blue frame fill is continuous to the host
            // edge and only the *inner* body is inset.
            panel = panel
                .h_full()
                .min_h_0()
                .flex_1()
                .pt(px(0.0))
                .px(px(3.0))
                .pb(px(3.0));
        }

        if title.is_some() || caption.is_some() {
            // Luna title bar: 26px strip, clipped to the window's
            // rounded top corners by the panel's overflow_hidden.
            // Active windows wear the 5-stop vertical blue
            // gradient (4 stacked 2-stop bands), inactive ones a
            // flat 2-stop gray-blue.
            let mut bar = gpui::div()
                .relative()
                .h(self.title_bar_height(&state, theme))
                .w_full()
                .flex()
                .flex_row()
                .items_center()
                .pl(px(8.0))
                .pr(px(4.0))
                .text_color(self.title_bar_fg(&state, theme))
                .text_size(self.title_font_size(&state, theme))
                .font_weight(FontWeight::BOLD);
            if active {
                // Solid underpaint first so flex-band rounding can never
                // leak the beige panel bg as a 1px "white" hairline.
                let underpaint = xp_color(theme, "xp.titlebar.mid_2", hsl_fallback(0x0050EE));
                let bands = self.title_bar_bands(&state, theme);
                let mut bands_layer = gpui::div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .bottom_0()
                    .bg(underpaint)
                    .flex()
                    .flex_col();
                for (frac, from, to) in bands {
                    // Overlap each band by 1px equivalent via slight
                    // flex growth; solid underpaint covers residual gaps.
                    bands_layer = bands_layer
                        .child(gpui::div().h(relative(frac)).w_full().bg(vgrad(from, to)));
                }
                bar = bar.child(bands_layer);
            } else {
                bar = bar.bg(self.title_bar_inactive_bg(&state, theme));
            }
            let title_leading = props.title_leading.take();
            if let Some(leading) = title_leading {
                bar = bar.child(
                    gpui::div()
                        .mr(px(4.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(leading),
                );
            }
            bar = bar.child(
                gpui::div()
                    .flex_grow()
                    .overflow_hidden()
                    .children(title.clone()),
            );
            if let Some(caption) = caption {
                let (from, to, close_from, close_to) = self.caption_faces(&state, theme);
                let mut buttons = gpui::div().flex().flex_row().gap(px(2.0));
                if let Some(on_minimize) = caption.on_minimize {
                    buttons = buttons.child(xp_caption_button(
                        format!("{:?}-cap-min", props.id).into(),
                        "_",
                        from,
                        to,
                        on_minimize,
                        theme,
                    ));
                }
                if let Some(on_maximize) = caption.on_maximize {
                    buttons = buttons.child(xp_caption_button(
                        format!("{:?}-cap-max", props.id).into(),
                        "□",
                        from,
                        to,
                        on_maximize,
                        theme,
                    ));
                }
                if let Some(on_close) = caption.on_close {
                    buttons = buttons.child(xp_caption_button(
                        format!("{:?}-cap-close", props.id).into(),
                        "×",
                        close_from,
                        close_to,
                        on_close,
                        theme,
                    ));
                }
                bar = bar.child(buttons);
            }
            panel = panel.child(bar);
        }

        // Window body: inset 3px from the frame (open at the
        // top), wrapped in its own 1px `#A09C8C` border, like
        // `.xp-window-body` in the CSS (`margin: 0 3px 3px;
        // border: 1px solid #a09c8c; border-top: none`).
        //
        // Dialog-style modals keep theme padding + gap + soft shadow.
        // Full-window chrome (Explorer) sets `body_padded = false`:
        // no dialog padding/gap, fill remaining height, no overlay
        // shadow (shadow would read as a transparent gutter under
        // the window when the panel is the host root).
        let mut body = gpui::div()
            .border(px(XP_BORDER_WIDTH))
            .border_t(px(0.0))
            .border_color(self.body_border(&state, theme))
            .flex()
            .flex_col()
            .children(children);
        if body_padded {
            // Dialog: classic 3px body inset via margin + content padding.
            body = body.mx(px(3.0)).mb(px(3.0)).p(pad.top).gap_2();
        } else {
            // Full-window: use panel padding for the 3px inset instead of
            // body margin. Margins can leave a 1–2px host-colored gutter
            // against the OS client edge on some DPI layouts.
            body = body
                .p(px(0.0))
                .gap(px(0.0))
                .flex_1()
                .w_full()
                .min_h_0()
                .overflow_hidden();
        }
        let mut panel = panel.child(body);
        if body_padded {
            panel = panel.shadow(shadow_vec(xp_shadow_overlay(theme)));
            // Dialog-style: keep enter animation.
            return div().child(AnimatedPresenceElement::new(
                props.state.clone(),
                props.id.clone(),
                SlideDirection::Down,
                px(theme.get_number("motion.slide_distance").unwrap_or(10.0) as f32),
                panel,
            ));
        }

        // Full-window chrome: paint the frame as the root surface.
        // Skipping AnimatedPresence avoids an extra layout wrapper that
        // can leave a 1–2px host gutter on some sides. `ModalProps::render`
        // applies the element id via `.apply()`, so this stays a plain `Div`.
        div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            .w_full()
            .h_full()
            .child(panel.w_full().h_full())
    }
}

/// One caption (title-bar) button: a glossy gradient square with
/// a translucent white edge. Hover lightens the gradient, active
/// reverses it — the Luna caption look.
fn xp_caption_button(
    id: ElementId,
    glyph: &'static str,
    from: Hsla,
    to: Hsla,
    on_press: yororen_ui_core::headless::modal::ModalCaptionCallback,
    theme: &Theme,
) -> Stateful<Div> {
    let border = xp_color(theme, "xp.caption.border", caption_border());
    let size = px(xp_number(theme, "xp.caption.size", 21.0));
    let radius = px(xp_number(theme, "xp.caption.radius", 3.0));
    let fg = xp_color(theme, "xp.titlebar.text", hsl_fallback(0xFFFFFF));
    let hover_from = lighten(from, 0.07);
    let hover_to = lighten(to, 0.07);
    gpui::div()
        .id(id)
        .w(size)
        .h(size)
        .rounded(radius)
        .border(px(XP_BORDER_WIDTH))
        .border_color(border)
        .bg(vgrad(from, to))
        .flex()
        .items_center()
        .justify_center()
        .text_color(fg)
        .text_size(px(13.))
        .cursor(CursorStyle::PointingHand)
        .child(glyph)
        .hover(move |s| s.bg(vgrad(hover_from, hover_to)))
        .active(move |s| s.bg(vgrad(to, from)))
        .on_click(move |_ev, window, cx| on_press(window, cx))
}

// =====================================================================
// Popover
// =====================================================================

pub use yororen_ui_core::renderer::popover::{PopoverRenderState, PopoverRenderer};

pub struct XpPopoverRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpPopoverRenderer {
    pub fn bg(&self, _: &PopoverRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("surface.raised")
            .unwrap_or(hsl_fallback(0xFFFFFF))
    }
    pub fn border(&self, _: &PopoverRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
    }
    pub fn border_radius(&self, _: &PopoverRenderState, _: &Theme) -> Pixels {
        px(XP_RADIUS)
    }
    pub fn offset(&self, _: &PopoverRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.popover.offset")
            .unwrap_or(8.0) as f32)
    }
}

impl PopoverRenderer for XpPopoverRenderer {
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
        let visible = props.state.read(cx).is_visible();
        let offset_px = self.offset(&state, theme);

        let mut outer = gpui::div().relative();

        if let Some(t) = props.trigger.take() {
            outer = outer.child(t);
        }

        if visible && let Some(c) = props.content.take() {
            // Capture outside-clicks to close the popover.
            let state_for_close = props.state.clone();
            outer = outer.on_mouse_down_out(move |_ev, _window, cx| {
                state_for_close.update(cx, |s, _cx| s.close());
            });
            // XP popover: white panel, 1px bevel border, 3px
            // corners, soft overlay shadow.
            let panel: Div = gpui::div()
                .absolute()
                .top(offset_px)
                .left_0()
                .bg(bg)
                .border_color(border)
                .border(px(XP_BORDER_WIDTH))
                .rounded(r)
                .shadow(shadow_vec(xp_shadow_overlay(theme)))
                .occlude()
                .child(c);
            let distance = px(theme.get_number("motion.slide_distance").unwrap_or(10.0) as f32);
            // The animation wrapper is absolutely positioned at the
            // top-left of the outer relative container so the panel
            // inside keeps its original `top/left` offset.
            outer = outer.child(
                gpui::deferred(div().absolute().top_0().left_0().child(
                    AnimatedPresenceElement::new(
                        props.state.clone(),
                        (props.id.clone(), "content"),
                        SlideDirection::Down,
                        distance,
                        panel,
                    ),
                ))
                .with_priority(1),
            );
        }

        outer
    }
}

// =====================================================================
// DropdownMenu
// =====================================================================

pub use yororen_ui_core::renderer::dropdown_menu::{DropdownMenuRenderState, DropdownMenuRenderer};

pub struct XpDropdownMenuRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpDropdownMenuRenderer {
    pub fn trigger_bg(&self, _: &DropdownMenuRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.bg")
            .unwrap_or(hsl_fallback(0xFFFFFF))
    }
    pub fn trigger_hover_bg(&self, _: &DropdownMenuRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.hover_bg")
            .unwrap_or(selection_hover_bg())
    }
    pub fn trigger_fg(&self, _: &DropdownMenuRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.fg")
            .unwrap_or(hsl_fallback(0x000000))
    }
    pub fn min_height(&self, _: &DropdownMenuRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.dropdown_menu.min_height")
            .unwrap_or(23.0) as f32)
    }
    pub fn border_radius(&self, _: &DropdownMenuRenderState, _: &Theme) -> Pixels {
        px(XP_RADIUS)
    }
    pub fn chevron_rotation(&self, state: &DropdownMenuRenderState, _: &Theme) -> f32 {
        if state.open { 180.0 } else { 0.0 }
    }
}

impl DropdownMenuRenderer for XpDropdownMenuRenderer {
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

        // 2) Body — only when visible, floated with
        //    `gpui::deferred` so it paints over subsequent
        //    sibling cells in the gallery.
        if props.state.read(cx).is_visible()
            && let Some(c) = props.content.take()
        {
            let state_for_close = props.state.clone();
            // The body is a `menu` element which already paints
            // its own border + bg; the XP dropdown panel only
            // adds the soft overlay shadow and the click-outside
            // dismissal. Avoid double borders by NOT setting
            // `border` / `border_color` here.
            let panel: Div = gpui::div()
                .absolute()
                .top(gpui::px(4.0))
                .left_0()
                .rounded(r)
                .shadow(shadow_vec(xp_shadow_overlay(theme)))
                .occlude()
                .on_mouse_down_out(move |_ev, _window, cx| {
                    state_for_close.update(cx, |s, _cx| s.close());
                })
                .child(c);
            let distance = px(theme.get_number("motion.slide_distance").unwrap_or(10.0) as f32);
            // The animation wrapper is absolutely positioned at the
            // top-left of the outer relative container so the panel
            // inside keeps its original `top/left` offset.
            outer = outer.child(
                gpui::deferred(div().absolute().top_0().left_0().child(
                    AnimatedPresenceElement::new(
                        props.state.clone(),
                        (props.id.clone(), "body"),
                        SlideDirection::Down,
                        distance,
                        panel,
                    ),
                ))
                .with_priority(1),
            );
        }

        outer
    }
}

// =====================================================================
// Disclosure
// =====================================================================

pub use yororen_ui_core::renderer::disclosure::{DisclosureRenderState, DisclosureRenderer};

pub struct XpDisclosureRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpDisclosureRenderer {
    pub fn trigger_bg(&self, _: &DisclosureRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.bg")
            .unwrap_or(hsl_fallback(0xFFFFFF))
    }
    pub fn trigger_fg(&self, _: &DisclosureRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.fg")
            .unwrap_or(hsl_fallback(0x000000))
    }
    pub fn trigger_hover_bg(&self, _: &DisclosureRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.hover_bg")
            .unwrap_or(selection_hover_bg())
    }
    pub fn min_height(&self, _: &DisclosureRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.disclosure.min_height")
            .unwrap_or(23.0) as f32)
    }
    pub fn border_radius(&self, _: &DisclosureRenderState, _: &Theme) -> Pixels {
        px(XP_RADIUS)
    }
    pub fn chevron_rotation(&self, state: &DisclosureRenderState, _: &Theme) -> f32 {
        if state.open { 90.0 } else { 0.0 }
    }
    pub fn body_padding(&self, _: &DisclosureRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.disclosure.padding")
            .unwrap_or(8.0) as f32)
    }
}

impl DisclosureRenderer for XpDisclosureRenderer {
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
            .cursor(CursorStyle::PointingHand)
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

pub struct XpOverlayRenderer;

impl XpOverlayRenderer {
    pub fn scrim_color(&self, _state: &OverlayRenderState, theme: &Theme) -> Hsla {
        // Same fallback as the default renderer (50% black) so the
        // gallery shows a visible scrim even if the theme omits the
        // `surface.scrim` key. 50% black is also the classic XP
        // dim behind modal windows.
        theme.get_color("surface.scrim").unwrap_or(Hsla {
            h: 0.0,
            s: 0.0,
            l: 0.0,
            a: 0.5,
        })
    }
}

impl OverlayRenderer for XpOverlayRenderer {
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
        // `sections/mod.rs::cell`). The XP flavor is just the
        // scrim color — the overlay has no border / radius.
        let scrim_el = div()
            .relative()
            .size_full()
            .bg(scrim)
            .when(!props.open, |el: Div| el.invisible());

        if !props.open {
            return div().id(props.id.clone()).child(scrim_el);
        }

        let duration_ms = theme
            .get_number("motion.duration_modal_fade")
            .unwrap_or(200.0) as u64;
        let el = fade_in_on_mount(
            scrim_el,
            props.id.clone(),
            std::time::Duration::from_millis(duration_ms),
            yororen_ui_core::animation::ease_out_quad,
        );
        div().id(props.id.clone()).child(el)
    }
}

// =====================================================================
// Menu
// =====================================================================

pub use yororen_ui_core::renderer::menu::{MenuRenderState, MenuRenderer};

pub struct XpMenuRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpMenuRenderer {
    pub fn bg(&self, _state: &MenuRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("surface.raised")
            .unwrap_or(hsl_fallback(0xFFFFFF))
    }
    pub fn border(&self, _state: &MenuRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
    }
    pub fn border_width(&self, _state: &MenuRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.menu.border_width")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32)
    }
    pub fn border_radius(&self, _state: &MenuRenderState, _theme: &Theme) -> Pixels {
        px(XP_RADIUS)
    }
    pub fn padding(&self, _state: &MenuRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.menu.padding")
            .unwrap_or(2.0) as f32)
    }
    pub fn min_width(&self, _state: &MenuRenderState, theme: &Theme) -> Pixels {
        // Floor the menu shell so an `absolute()` panel
        // (dropdown, popover) cannot collapse below a usable
        // width. 120 px keeps XP context menus compact compared
        // to the brutalism shell (200 px).
        px(theme
            .get_number("tokens.control.menu.min_width")
            .unwrap_or(120.0) as f32)
    }
    pub fn item_padding_x(&self, _state: &MenuRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.menu.item_padding_x")
            .unwrap_or(16.0) as f32)
    }
    pub fn item_padding_y(&self, _state: &MenuRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.menu.item_padding_y")
            .unwrap_or(3.0) as f32)
    }
    pub fn item_gap(&self, _state: &MenuRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.control.menu.gap").unwrap_or(0.0) as f32)
    }
    pub fn item_hover_bg(&self, _state: &MenuRenderState, theme: &Theme) -> Hsla {
        // XP menu rows highlight with the solid selection blue
        // (`#316AC5`) and flip to white text, like the classic
        // context-menu hot track.
        xp_color(theme, "xp.menu.hover_bg", menu_hover_bg())
    }
    pub fn item_hl_fg(&self, _state: &MenuRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.menu.hover_fg", menu_hover_fg())
    }
    pub fn group_label_fg(&self, _state: &MenuRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.tertiary")
            .unwrap_or(hsl_fallback(0x56554A))
    }
}

impl MenuRenderer for XpMenuRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::menu::MenuProps,
        cx: &App,
    ) -> Stateful<Div> {
        use yororen_ui_core::headless::dropdown_menu::DropdownItem;
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = MenuRenderState {};
        let bg = self.bg(&state, theme);
        let border = self.border(&state, theme);
        let bw = self.border_width(&state, theme);
        let radius = self.border_radius(&state, theme);
        let pad = self.padding(&state, theme);
        let item_px = self.item_padding_x(&state, theme);
        let item_py = self.item_padding_y(&state, theme);
        let item_gap = self.item_gap(&state, theme);
        let item_hover_bg = self.item_hover_bg(&state, theme);
        let item_hl_fg = self.item_hl_fg(&state, theme);
        let group_label_fg = self.group_label_fg(&state, theme);
        let min_w = self.min_width(&state, theme);
        let divider = theme
            .get_color("border.divider")
            .unwrap_or(bevel_inner_dark());

        let items = props.state.read(cx).items.clone();
        let highlighted = props.state.read(cx).highlighted_index;

        // Build the menu body (flex column) with one row per
        // item. Highlighted row is painted using `item_hover_bg`
        // directly so keyboard navigation matches mouse hover
        // without an extra theme key.
        let mut body: Div = gpui::div().flex().flex_col().gap(item_gap);

        for (i, item) in items.iter().enumerate() {
            match item {
                DropdownItem::Item(menu_item) => {
                    let is_highlighted = highlighted == Some(i);
                    let state_for_pick = props.state.clone();
                    let id = menu_item.id.clone();
                    let label = menu_item.label.to_string();
                    let row_bg = if is_highlighted { item_hover_bg } else { bg };
                    let row_fg = if is_highlighted {
                        item_hl_fg
                    } else {
                        theme
                            .get_color("content.primary")
                            .unwrap_or(hsl_fallback(0x000000))
                    };
                    let mut row: Stateful<Div> = gpui::div()
                        .id(ElementId::Name(format!("xp-menu-item-{}", i).into()))
                        .w_full()
                        .px(item_px)
                        .py(item_py)
                        .rounded(px(XP_RADIUS))
                        .bg(row_bg)
                        .text_color(row_fg)
                        .cursor(CursorStyle::PointingHand)
                        .hover(move |s| s.bg(item_hover_bg))
                        .child(label);
                    row = row.on_click(move |_ev, window, cx| {
                        let cb = state_for_pick.read(cx).on_select().cloned();
                        if let Some(f) = cb {
                            f(id.clone(), window, cx);
                        }
                    });
                    body = body.child(row);
                }
                DropdownItem::Separator => {
                    // A 1-pixel etched separator matches the
                    // classic XP menu divider.
                    let sep = gpui::div()
                        .id(ElementId::Name(format!("xp-menu-sep-{}", i).into()))
                        .w_full()
                        .h(px(1.0))
                        .my(px(2.0))
                        .bg(divider);
                    body = body.child(sep);
                }
                DropdownItem::Group(group) => {
                    let group_label = group.label.to_string();
                    let header = gpui::div()
                        .id(ElementId::Name(format!("xp-menu-group-{}", i).into()))
                        .w_full()
                        .px(item_px)
                        .py(px(4.0))
                        .text_color(group_label_fg)
                        .text_size(px(11.0))
                        .child(group_label);
                    body = body.child(header);
                }
            }
        }

        // XP menu shell: white panel + 1px bevel border + 3px
        // corners + soft overlay shadow (menus float above other
        // content, so they use the overlay shadow tier).
        gpui::div()
            .id(props.id.clone())
            .min_w(min_w)
            .bg(bg)
            .border(bw)
            .border_color(border)
            .rounded(radius)
            .p(pad)
            .shadow(shadow_vec(xp_shadow_overlay(theme)))
            .child(body)
    }
}
