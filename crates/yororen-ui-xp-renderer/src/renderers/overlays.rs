//! XP (Luna) overlay renderers: `Modal`, `Popover`,
//! `DropdownMenu`, `Disclosure`, `Overlay`, `Menu`.
//!
//! The `Modal` is the signature XP window: horizontal blue
//! gradient title bar with a white bold caption, beige
//! (`#ECE9D8`) content area, 1px dark-blue frame, 8px rounded
//! corners and a soft overlay shadow. Popovers, dropdowns and
//! menus are white panels with a 1px bevel border, 3px corners
//! and the same soft shadow; hovered rows use the pale-blue
//! `xp.selection.hover_bg` highlight.

use gpui::prelude::FluentBuilder;
use gpui::{
    App, Background, CursorStyle, Div, ElementId, FontWeight, Hsla, InteractiveElement,
    ParentElement, Pixels, Stateful, StatefulInteractiveElement, Styled, div, px,
};
use yororen_ui_core::animation::SlideDirection;
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

use crate::style::{
    XP_BORDER_WIDTH, XP_RADIUS, bevel_inner_dark, button_default_border, dialog_bg, hsl_fallback,
    selection_hover_bg, shadow_vec, titlebar_gradient, xp_color, xp_number, xp_shadow_overlay,
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
    /// 1px dark-blue window frame (`#003C74`).
    pub fn panel_border(&self, _: &ModalRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.button.default_border", button_default_border())
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
    /// Luna title-bar strip height (28px).
    pub fn title_bar_height(&self, _: &ModalRenderState, theme: &Theme) -> Pixels {
        px(xp_number(theme, "xp.titlebar.height", 28.0))
    }
    /// White caption text on the blue gradient.
    pub fn title_bar_fg(&self, _: &ModalRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.titlebar.text", hsl_fallback(0xFFFFFF))
    }
    /// Horizontal Luna blue gradient.
    pub fn title_bar_bg(&self, _: &ModalRenderState, theme: &Theme) -> Background {
        titlebar_gradient(theme)
    }
    pub fn title_font_size(&self, _: &ModalRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.modal.title_size")
            .unwrap_or(13.0) as f32)
    }
}

impl ModalRenderer for XpModalRenderer {
    fn compose(&self, props: &mut yororen_ui_core::headless::modal::ModalProps, cx: &App) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = ModalRenderState {};
        let panel_bg = self.panel_bg(&state, theme);
        let panel_border = self.panel_border(&state, theme);
        let pad = self.panel_padding(&state, theme);
        let r = self.panel_border_radius(&state, theme);

        if !props.state.read(cx).is_visible() {
            return div();
        }

        let title = props.state.read(cx).title.clone();
        let children = std::mem::take(&mut props.children);
        // The XP Modal renderer paints *only* the window (frame /
        // title bar / content area / soft overlay shadow). The
        // scrim and centering are the caller's responsibility —
        // same contract as `TokenModalRenderer` in the default
        // renderer.
        let mut panel = gpui::div()
            .bg(panel_bg)
            .border_color(panel_border)
            .border(px(XP_BORDER_WIDTH))
            .rounded(r)
            .overflow_hidden()
            .flex()
            .flex_col()
            .w_full();

        if let Some(title) = title {
            // Luna title bar: 28px horizontal blue gradient strip
            // with a white bold caption, clipped to the window's
            // rounded top corners by the panel's overflow_hidden.
            panel = panel.child(
                gpui::div()
                    .h(self.title_bar_height(&state, theme))
                    .w_full()
                    .bg(self.title_bar_bg(&state, theme))
                    .flex()
                    .flex_row()
                    .items_center()
                    .px(px(8.0))
                    .text_color(self.title_bar_fg(&state, theme))
                    .text_size(self.title_font_size(&state, theme))
                    .font_weight(FontWeight::BOLD)
                    .child(title),
            );
        }

        let panel = panel
            .child(
                gpui::div()
                    .p(pad.top)
                    .flex()
                    .flex_col()
                    .gap_2()
                    .w_full()
                    .children(children),
            )
            .shadow(shadow_vec(xp_shadow_overlay(theme)));

        div().child(AnimatedPresenceElement::new(
            props.state.clone(),
            props.id.clone(),
            SlideDirection::Down,
            px(theme.get_number("motion.slide_distance").unwrap_or(10.0) as f32),
            panel,
        ))
    }
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
        // XP menu rows highlight with the pale-blue selection
        // hover (`#C1D2EE`) and keep their dark text, like the
        // Luna start-menu / context-menu hot track.
        xp_color(theme, "xp.selection.hover_bg", selection_hover_bg())
    }
    pub fn item_hl_fg(&self, _state: &MenuRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.primary")
            .unwrap_or(hsl_fallback(0x000000))
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
                        theme.get_color("content.primary").unwrap_or(item_hl_fg)
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
