//! XP (Luna) action renderers: `Button`, `IconButton`,
//! `ToggleButton`, `SplitButton`, `ButtonGroup`.
//!
//! The button family is the visual benchmark for the whole crate:
//! white → beige → cream three-stop gradient face (two stacked
//! 2-stop bands), 1px dark-blue border, 3px radius; hover keeps
//! the face and adds an orange inset ring; active reverses the
//! gradient (pressed). Primary uses the Luna blue gradient with
//! white text.

use gpui::{
    App, Background, CursorStyle, Div, ElementId, FocusHandle, Hsla, InteractiveElement,
    ParentElement, Pixels, Stateful, StatefulInteractiveElement, Styled, div, px, relative,
};
use yororen_ui_core::animation::SlideDirection;
use yororen_ui_core::headless::button::ButtonProps;
use yororen_ui_core::headless::icon::IconProps;
use yororen_ui_core::headless::icon_button::IconButtonProps;
use yororen_ui_core::headless::toggle_button::ToggleButtonProps;
use yororen_ui_core::renderer::spec::{BorderSpec, Edges};
use yororen_ui_core::renderer::variant::ActionVariantKind;
use yororen_ui_core::renderer::variant::VariantState;
use yororen_ui_core::theme::ActiveTheme;
use yororen_ui_core::theme::Theme;
use yororen_ui_default_renderer::animation::AnimatedPresenceElement;

use crate::style::{
    self, XP_BORDER_WIDTH, XP_RADIUS, bevel_inner_dark, bevel_outer_dark, button_border,
    button_default_border, button_face, button_face_pressed, button_face_stops, button_hover_ring,
    darken, input_focus_border, lighten, primary_face, primary_face_pressed, selection_hover_bg,
    shadow_vec, vgrad, xp_color, xp_shadow_overlay,
};

// =====================================================================
// Shared button-face logic (neutral / primary / danger)
// =====================================================================

fn action_variant_key(variant: ActionVariantKind) -> &'static str {
    match variant {
        ActionVariantKind::Neutral => "neutral",
        ActionVariantKind::Primary => "primary",
        ActionVariantKind::Danger => "danger",
    }
}

/// Resting face gradient for the variant.
fn face_bg(variant: ActionVariantKind, theme: &Theme) -> Background {
    match variant {
        ActionVariantKind::Neutral => button_face(theme),
        ActionVariantKind::Primary => primary_face(theme),
        ActionVariantKind::Danger => {
            let top = theme
                .get_color("action.danger.hover_bg")
                .unwrap_or(crate::style::hsl_fallback(0xE06858));
            let bottom = theme
                .get_color("action.danger.bg")
                .unwrap_or(crate::style::hsl_fallback(0xC75050));
            vgrad(top, bottom)
        }
    }
}

/// Hover face: brightened gradient (orange-tinted for neutral,
/// like the XP toolbar hot-track glow; the plain `Button` keeps
/// its face unchanged on hover and shows the inset ring instead).
fn face_hover_bg(variant: ActionVariantKind, theme: &Theme) -> Background {
    match variant {
        ActionVariantKind::Neutral => vgrad(
            xp_color(
                theme,
                "xp.button.face_from",
                crate::style::button_face_from(),
            ),
            xp_color(theme, "xp.button.hover_ring", button_hover_ring()),
        ),
        ActionVariantKind::Primary => vgrad(
            lighten(
                xp_color(
                    theme,
                    "xp.button.primary_from",
                    crate::style::primary_from(),
                ),
                0.08,
            ),
            lighten(
                xp_color(theme, "xp.button.primary_to", crate::style::primary_to()),
                0.08,
            ),
        ),
        ActionVariantKind::Danger => {
            let top = theme
                .get_color("action.danger.hover_bg")
                .unwrap_or(crate::style::hsl_fallback(0xE06858));
            let bottom = theme
                .get_color("action.danger.bg")
                .unwrap_or(crate::style::hsl_fallback(0xC75050));
            vgrad(lighten(top, 0.08), lighten(bottom, 0.08))
        }
    }
}

/// Pressed face: gradient reversed for the sunken look.
fn face_active_bg(variant: ActionVariantKind, theme: &Theme) -> Background {
    match variant {
        ActionVariantKind::Neutral => button_face_pressed(theme),
        ActionVariantKind::Primary => primary_face_pressed(theme),
        ActionVariantKind::Danger => {
            let top = theme
                .get_color("action.danger.active_bg")
                .unwrap_or(crate::style::hsl_fallback(0x983C28));
            let bottom = theme
                .get_color("action.danger.bg")
                .unwrap_or(crate::style::hsl_fallback(0xC75050));
            vgrad(top, bottom)
        }
    }
}

/// Resting border color for the variant.
fn face_border(variant: ActionVariantKind, theme: &Theme) -> Hsla {
    match variant {
        ActionVariantKind::Neutral => xp_color(theme, "xp.button.border", button_border()),
        ActionVariantKind::Primary => {
            xp_color(theme, "xp.button.default_border", button_default_border())
        }
        ActionVariantKind::Danger => {
            let base = theme
                .get_color("action.danger.bg")
                .unwrap_or(crate::style::hsl_fallback(0xC75050));
            darken(base, 0.18)
        }
    }
}

fn disabled_bg(variant: ActionVariantKind, theme: &Theme) -> Hsla {
    theme
        .get_color(&format!(
            "action.{}.disabled_bg",
            action_variant_key(variant)
        ))
        .unwrap_or(bevel_inner_dark())
}

fn disabled_fg(variant: ActionVariantKind, theme: &Theme) -> Hsla {
    theme
        .get_color(&format!(
            "action.{}.disabled_fg",
            action_variant_key(variant)
        ))
        .unwrap_or(bevel_outer_dark())
}

fn disabled_border(theme: &Theme) -> Hsla {
    xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
}

/// Blue hot-track outline used on hover.
fn hover_border(theme: &Theme) -> Hsla {
    xp_color(theme, "xp.input.focus_border", input_focus_border())
}

fn button_radius(theme: &Theme) -> Pixels {
    px(theme
        .get_number("tokens.control.button.radius")
        .unwrap_or(XP_RADIUS as f64) as f32)
}

fn button_border_width(theme: &Theme) -> Pixels {
    px(theme
        .get_number("tokens.control.button.border_width")
        .unwrap_or(XP_BORDER_WIDTH as f64) as f32)
}

// =====================================================================
// Button
// =====================================================================

pub use yororen_ui_core::renderer::button::{ButtonRenderState, ButtonRenderer};

pub struct XpButtonRenderer;

// Inherent helpers — *not* part of the `ButtonRenderer` trait
// surface. They exist so `compose` (below) can stay readable
// and so other code in this crate can share the palette
// lookups.
impl XpButtonRenderer {
    pub fn bg(&self, state: &ButtonRenderState, theme: &Theme) -> Background {
        if let Some(s) = &state.custom_style {
            return s
                .bg(&VariantState {
                    disabled: state.disabled,
                })
                .into();
        }
        if state.disabled {
            return disabled_bg(state.variant, theme).into();
        }
        face_bg(state.variant, theme)
    }

    pub fn fg(&self, state: &ButtonRenderState, theme: &Theme) -> Hsla {
        if let Some(s) = &state.custom_style {
            return s.fg(&VariantState {
                disabled: state.disabled,
            });
        }
        if state.disabled {
            return disabled_fg(state.variant, theme);
        }
        theme
            .get_color(&format!("action.{}.fg", action_variant_key(state.variant)))
            .unwrap_or(style::hsl_fallback(0x000000))
    }

    pub fn hover_bg(&self, state: &ButtonRenderState, theme: &Theme) -> Background {
        if let Some(s) = &state.custom_style {
            return s
                .bg(&VariantState {
                    disabled: state.disabled,
                })
                .into();
        }
        if state.disabled {
            return disabled_bg(state.variant, theme).into();
        }
        face_hover_bg(state.variant, theme)
    }

    pub fn active_bg(&self, state: &ButtonRenderState, theme: &Theme) -> Background {
        if let Some(s) = &state.custom_style {
            return s
                .bg(&VariantState {
                    disabled: state.disabled,
                })
                .into();
        }
        if state.disabled {
            return disabled_bg(state.variant, theme).into();
        }
        face_active_bg(state.variant, theme)
    }

    pub fn border(&self, state: &ButtonRenderState, theme: &Theme) -> Option<BorderSpec> {
        let color = if state.disabled {
            disabled_border(theme)
        } else {
            face_border(state.variant, theme)
        };
        Some(BorderSpec::new(button_border_width(theme), color))
    }

    pub fn padding(&self, _: &ButtonRenderState, theme: &Theme) -> Edges<Pixels> {
        let h = theme
            .get_number("tokens.control.button.horizontal_padding")
            .unwrap_or(14.0) as f32;
        let v = theme
            .get_number("tokens.control.button.vertical_padding")
            .unwrap_or(3.0) as f32;
        Edges::symmetric(px(h), px(v))
    }

    pub fn border_radius(&self, _: &ButtonRenderState, theme: &Theme) -> Pixels {
        button_radius(theme)
    }

    pub fn min_height(&self, _: &ButtonRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.button.min_height")
            .unwrap_or(23.0) as f32)
    }
}

impl ButtonRenderer for XpButtonRenderer {
    fn compose(&self, props: &ButtonProps, focus_handle: &FocusHandle, cx: &App) -> Stateful<Div> {
        let theme = cx.theme();
        let state = ButtonRenderState {
            variant: props.variant,
            disabled: props.disabled,
            ..Default::default()
        };
        let bg = self.bg(&state, theme);
        let fg = self.fg(&state, theme);
        let padding = self.padding(&state, theme);
        let radius = self.border_radius(&state, theme);
        let min_h = self.min_height(&state, theme);
        let hover_bg = self.hover_bg(&state, theme);
        let active_bg = self.active_bg(&state, theme);
        let border = self.border(&state, theme);
        let hover_border = hover_border(theme);
        let icon_gap = theme
            .get_number("tokens.control.button.icon_gap")
            .unwrap_or(6.0) as f32;

        let mut el: Stateful<Div> = div()
            .id(props.id.clone())
            .group("xp-btn")
            .relative()
            .text_color(fg)
            .min_h(min_h)
            .rounded(radius)
            .overflow_hidden()
            .px(padding.left)
            .py(padding.top)
            .gap(px(icon_gap))
            .flex()
            .items_center()
            .justify_center()
            .track_focus(focus_handle);

        if let Some(b) = border {
            el = el.border(b.width).border_color(b.color);
        }

        if state.disabled {
            // Flat disabled face; no hover ring / gradient play.
            el = el.bg(bg);
        } else if state.custom_style.is_some() {
            // Caller-owned visuals keep the legacy hover/active
            // behavior (no bands, no inset ring).
            el = el
                .bg(bg)
                .hover(move |s| s.bg(hover_bg).border_color(hover_border))
                .active(move |s| s.bg(active_bg));
        } else {
            match state.variant {
                ActionVariantKind::Neutral => {
                    // CSS face: `#FFF → #ECE9D8 (45%) → #DDD8C8`.
                    // gpui gradients have only two stops, so the
                    // 3-stop face is two stacked 2-stop bands;
                    // `active` flips them to the reversed gradient
                    // (`#DDD8C8 → #ECE9D8 55% → #FFF`) through the
                    // group style. (Group styles require the
                    // stateful flavor, hence the per-band ids.)
                    let (top, mid, bottom) = button_face_stops(theme);
                    el = el.child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .right_0()
                            .bottom_0()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .id(format!("{:?}-face-top", props.id))
                                    .h(relative(0.45))
                                    .w_full()
                                    .bg(vgrad(top, mid))
                                    .group_active("xp-btn", move |s| {
                                        s.h(relative(0.55)).bg(vgrad(bottom, mid))
                                    }),
                            )
                            .child(
                                div()
                                    .id(format!("{:?}-face-bottom", props.id))
                                    .flex_grow()
                                    .w_full()
                                    .bg(vgrad(mid, bottom))
                                    .group_active("xp-btn", move |s| s.bg(vgrad(mid, top))),
                            ),
                    );
                }
                ActionVariantKind::Primary | ActionVariantKind::Danger => {
                    el = el
                        .bg(bg)
                        .hover(move |s| s.bg(hover_bg))
                        .active(move |s| s.bg(active_bg));
                }
            }
            // Orange inset ring on hover (the CSS `box-shadow:
            // inset 0 0 0 1px #FFCF31`): a 1px frame painted just
            // inside the button border; the face stays unchanged.
            let ring = xp_color(theme, "xp.button.hover_ring", button_hover_ring());
            let ring_radius = if radius > px(1.) {
                radius - px(1.)
            } else {
                px(0.)
            };
            el = el.child(
                div()
                    .id(format!("{:?}-hover-ring", props.id))
                    .absolute()
                    .top(px(1.))
                    .left(px(1.))
                    .right(px(1.))
                    .bottom(px(1.))
                    .rounded(ring_radius)
                    .border(px(XP_BORDER_WIDTH))
                    .border_color(gpui::transparent_black())
                    .group_hover("xp-btn", move |s| s.border_color(ring)),
            );
        }

        if let Some(source) = props.icon.clone() {
            let icon_id: ElementId = format!("{:?}-icon", props.id).into();
            let icon_el = IconProps {
                id: icon_id,
                source,
                size: Some(props.icon_size),
                color: Some(fg),
            }
            .render(cx);
            el = el.child(icon_el);
        }
        if let Some(caption) = props.caption.clone() {
            el = el.child(caption);
        }

        el.cursor(if props.disabled {
            CursorStyle::OperationNotAllowed
        } else {
            CursorStyle::PointingHand
        })
    }
}

// =====================================================================
// IconButton
// =====================================================================

pub use yororen_ui_core::renderer::icon_button::{IconButtonRenderState, IconButtonRenderer};

pub struct XpIconButtonRenderer;

// Inherent helpers — *not* part of the `IconButtonRenderer`
// trait surface.
impl XpIconButtonRenderer {
    pub fn bg(&self, state: &IconButtonRenderState, theme: &Theme) -> Background {
        if let Some(s) = &state.custom_style {
            return s
                .bg(&VariantState {
                    disabled: state.disabled,
                })
                .into();
        }
        if state.disabled {
            return disabled_bg(state.variant, theme).into();
        }
        face_bg(state.variant, theme)
    }

    pub fn fg(&self, state: &IconButtonRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            return disabled_fg(state.variant, theme);
        }
        theme
            .get_color(&format!("action.{}.fg", action_variant_key(state.variant)))
            .unwrap_or(style::hsl_fallback(0x000000))
    }

    pub fn hover_bg(&self, state: &IconButtonRenderState, theme: &Theme) -> Background {
        if let Some(s) = &state.custom_style {
            return s
                .bg(&VariantState {
                    disabled: state.disabled,
                })
                .into();
        }
        if state.disabled {
            return disabled_bg(state.variant, theme).into();
        }
        face_hover_bg(state.variant, theme)
    }

    pub fn active_bg(&self, state: &IconButtonRenderState, theme: &Theme) -> Background {
        if let Some(s) = &state.custom_style {
            return s
                .bg(&VariantState {
                    disabled: state.disabled,
                })
                .into();
        }
        if state.disabled {
            return disabled_bg(state.variant, theme).into();
        }
        face_active_bg(state.variant, theme)
    }

    pub fn size(&self, _: &IconButtonRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.icon_button.size")
            .unwrap_or(26.0) as f32)
    }

    pub fn border_radius(&self, _: &IconButtonRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.icon_button.radius")
            .unwrap_or(XP_RADIUS as f64) as f32)
    }
}

impl IconButtonRenderer for XpIconButtonRenderer {
    fn compose(
        &self,
        props: &IconButtonProps,
        focus_handle: &FocusHandle,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = IconButtonRenderState {
            variant: props.variant,
            disabled: props.disabled,
            has_custom_bg: false,
            has_custom_hover_bg: false,
            custom_style: None,
        };
        let bg = self.bg(&state, theme);
        let fg = self.fg(&state, theme);
        let radius = self.border_radius(&state, theme);
        let hover_bg = self.hover_bg(&state, theme);
        let active_bg = self.active_bg(&state, theme);
        let side = self.size(&state, theme);
        let border_color = if props.disabled {
            disabled_border(theme)
        } else {
            face_border(state.variant, theme)
        };
        let hover_border = hover_border(theme);
        let border_w = button_border_width(theme);

        let mut el: Stateful<Div> = div()
            .id(props.id.clone())
            .bg(bg)
            .rounded(radius)
            .size(side)
            .border(border_w)
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .track_focus(focus_handle);

        if let Some(source) = props.icon.clone() {
            let icon_id: ElementId = format!("{:?}-icon", props.id).into();
            let icon_el = IconProps {
                id: icon_id,
                source,
                size: Some(props.icon_size),
                color: Some(fg),
            }
            .render(cx);
            el = el.child(icon_el);
        }

        el.hover(|s| s.bg(hover_bg).border_color(hover_border))
            .active(|s| s.bg(active_bg))
            .cursor(if props.disabled {
                CursorStyle::OperationNotAllowed
            } else {
                CursorStyle::PointingHand
            })
    }
}

// =====================================================================
// ToggleButton
// =====================================================================

pub use yororen_ui_core::renderer::toggle_button::{ToggleButtonRenderState, ToggleButtonRenderer};

pub struct XpToggleButtonRenderer;

// Inherent helpers — *not* part of the `ToggleButtonRenderer`
// trait surface.
impl XpToggleButtonRenderer {
    pub fn bg(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Background {
        if let Some(s) = &state.custom_style {
            if state.selected {
                return face_active_bg(ActionVariantKind::Neutral, theme);
            }
            return s
                .bg(&VariantState {
                    disabled: state.disabled,
                })
                .into();
        }
        if state.disabled {
            return disabled_bg(ActionVariantKind::Neutral, theme).into();
        }
        if state.selected {
            // Selected toggles sit sunken, like a held-down XP
            // toolbar button.
            return face_active_bg(ActionVariantKind::Neutral, theme);
        }
        face_bg(ActionVariantKind::Neutral, theme)
    }

    pub fn fg(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Hsla {
        if let Some(s) = &state.custom_style {
            return s.fg(&VariantState {
                disabled: state.disabled,
            });
        }
        if state.disabled {
            return disabled_fg(ActionVariantKind::Neutral, theme);
        }
        theme
            .get_color("action.neutral.fg")
            .unwrap_or(style::hsl_fallback(0x000000))
    }

    pub fn hover_bg(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Background {
        if state.disabled {
            return disabled_bg(ActionVariantKind::Neutral, theme).into();
        }
        if state.selected {
            return face_active_bg(ActionVariantKind::Neutral, theme);
        }
        face_hover_bg(ActionVariantKind::Neutral, theme)
    }

    pub fn active_bg(&self, state: &ToggleButtonRenderState, theme: &Theme) -> Background {
        if state.disabled {
            return disabled_bg(ActionVariantKind::Neutral, theme).into();
        }
        face_active_bg(ActionVariantKind::Neutral, theme)
    }

    pub fn min_height(&self, _: &ToggleButtonRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.toggle_button.min_height")
            .unwrap_or(26.0) as f32)
    }

    pub fn border_radius(&self, _: &ToggleButtonRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.toggle_button.radius")
            .unwrap_or(XP_RADIUS as f64) as f32)
    }
}

impl ToggleButtonRenderer for XpToggleButtonRenderer {
    fn compose(
        &self,
        props: &ToggleButtonProps,
        focus_handle: &FocusHandle,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = ToggleButtonRenderState {
            variant: props.variant,
            selected: props.selected,
            disabled: props.disabled,
            custom_style: None,
        };
        let bg = self.bg(&state, theme);
        let fg = self.fg(&state, theme);
        let min_h = self.min_height(&state, theme);
        let radius = self.border_radius(&state, theme);
        let hover_bg = self.hover_bg(&state, theme);
        let active_bg = self.active_bg(&state, theme);
        let icon_gap = theme
            .get_number("tokens.control.toggle_button.icon_gap")
            .unwrap_or(6.0) as f32;
        let border_color = if props.disabled {
            disabled_border(theme)
        } else {
            face_border(ActionVariantKind::Neutral, theme)
        };
        let hover_border = hover_border(theme);
        let border_w = button_border_width(theme);

        let mut el: Stateful<Div> = div()
            .id(props.id.clone())
            .bg(bg)
            .text_color(fg)
            .min_h(min_h)
            .rounded(radius)
            .px(px(10.))
            .py(px(4.))
            .gap(px(icon_gap))
            .border(border_w)
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .track_focus(focus_handle);

        if let Some(source) = props.icon.clone() {
            let icon_id: ElementId = format!("{:?}-icon", props.id).into();
            let icon_el = IconProps {
                id: icon_id,
                source,
                size: Some(props.icon_size),
                color: Some(fg),
            }
            .render(cx);
            el = el.child(icon_el);
        }
        if let Some(caption) = props.caption.clone() {
            el = el.child(caption);
        }

        el.hover(|s| s.bg(hover_bg).border_color(hover_border))
            .active(|s| s.bg(active_bg))
            .cursor(if props.disabled {
                CursorStyle::OperationNotAllowed
            } else {
                CursorStyle::PointingHand
            })
    }
}

// =====================================================================
// SplitButton
// =====================================================================

pub use yororen_ui_core::renderer::split_button::{SplitButtonRenderState, SplitButtonRenderer};

pub struct XpSplitButtonRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpSplitButtonRenderer {
    pub fn min_height(&self, _: &SplitButtonRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.split_button.min_height")
            .unwrap_or(26.0) as f32)
    }

    pub fn gap(&self, _: &SplitButtonRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.split_button.separator_w")
            .unwrap_or(1.0) as f32)
    }
}

impl SplitButtonRenderer for XpSplitButtonRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::split_button::SplitButtonProps,
        cx: &App,
    ) -> Div {
        use std::sync::Arc;
        use yororen_ui_core::headless::dropdown_menu::DropdownItem;
        use yororen_ui_core::headless::list_item::ListItemProps;
        use yororen_ui_core::headless::split_button::ClickCallback;
        use yororen_ui_core::theme::ActiveTheme;

        let theme = cx.theme();
        let (open, visible) = props
            .state
            .as_ref()
            .map(|s| {
                let s = s.read(cx);
                (s.is_open(), s.is_visible())
            })
            .unwrap_or((false, false));
        let state = SplitButtonRenderState {
            open,
            disabled: props.disabled,
        };

        // ---- Primary button ----
        let primary_id: ElementId = format!("{:?}-primary", props.id).into();
        let primary = ButtonProps {
            id: primary_id,
            focus_handle: props.primary_focus.clone(),
            on_click: Some(props.primary.clone()),
            disabled: props.disabled,
            clickable: true,
            variant: ActionVariantKind::Neutral,
            caption: props.caption.clone(),
            icon: None,
            icon_size: px(16.),
        }
        .render(cx);

        // ---- Chevron button ----
        let state_for_chevron = props.state.clone();
        let chevron_click: ClickCallback = Arc::new(
            move |_ev: &gpui::ClickEvent, _w: &mut gpui::Window, cx: &mut App| {
                if let Some(s) = state_for_chevron.as_ref() {
                    s.update(cx, |st, _cx| st.toggle());
                }
            },
        );
        let chevron_label = if open { "▴" } else { "▾" };
        let chevron_id: ElementId = format!("{:?}-chevron", props.id).into();
        let chevron_w = px(theme
            .get_number("tokens.control.split_button.chevron_width")
            .unwrap_or(24.0) as f32);
        let chevron = ButtonProps {
            id: chevron_id,
            focus_handle: props.chevron_focus.clone(),
            on_click: Some(chevron_click),
            disabled: props.disabled,
            clickable: true,
            variant: ActionVariantKind::Neutral,
            caption: Some(chevron_label.into()),
            icon: None,
            icon_size: px(16.),
        }
        .render(cx)
        .w(chevron_w)
        .px(px(0.));

        // ---- Dropdown body ----
        // `gpui::deferred(...)` so the popover paints *after*
        // every other sibling in the tree. `.absolute()` only
        // takes the menu out of layout flow; without `deferred`
        // any later sibling would draw on top of it.
        let gap = self.gap(&state, theme);
        let root = div()
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .gap(gap)
            .child(primary)
            .child(chevron);
        if visible {
            let panel_bg = theme
                .get_color("surface.popover")
                .or_else(|| theme.get_color("surface.raised"))
                .unwrap_or(style::hsl_fallback(0xFFFFFF));
            let panel_border = xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark());
            let item_hover_bg = xp_color(theme, "xp.selection.hover_bg", selection_hover_bg());
            let divider_color = theme
                .get_color("border.divider")
                .unwrap_or(bevel_inner_dark());
            let menu_w = px(theme
                .get_number("tokens.control.split_button.menu_width")
                .unwrap_or(180.0) as f32);
            let min_h = self.min_height(&state, theme);
            let menu_offset = min_h + px(2.);

            let mut menu = div()
                .absolute()
                .top(menu_offset)
                .left_0()
                .w(menu_w)
                .bg(panel_bg)
                .border(px(XP_BORDER_WIDTH))
                .border_color(panel_border)
                .rounded(px(XP_RADIUS))
                .p(px(2.))
                .flex()
                .flex_col()
                .shadow(shadow_vec(xp_shadow_overlay(theme)))
                // popover pattern: occlude_mouse blocks
                // events from reaching elements painted behind
                // the menu, and on_mouse_down_out fires when the
                // user clicks *anywhere* outside the menu (other
                // cells, the toolbar, the title) to dismiss.
                .occlude()
                .on_mouse_down_out({
                    let state_for_close = props.state.clone();
                    move |_ev, _window, cx| {
                        if let Some(st) = state_for_close.as_ref() {
                            st.update(cx, |s, _cx| s.close());
                        }
                    }
                });

            for it in &props.items {
                match it {
                    DropdownItem::Item(item) => {
                        let item_id_str = item.id.clone();
                        let item_label = item.label.clone();
                        let item_disabled = item.disabled;
                        let state_for_click = props.state.clone();
                        let on_select_for_click = props.on_select.clone();
                        let item_id_for_callback = item_id_str.clone();

                        let row_id: ElementId =
                            format!("{:?}-item-{}", props.id, item_id_str).into();
                        let list_item_el = ListItemProps {
                            id: row_id,
                            title: item_label,
                            description: None,
                            leading_icon: None,
                            trailing_icon: None,
                            selected: false,
                            disabled: item_disabled,
                            on_click: None,
                        }
                        .render(cx);

                        let item_el = if !item_disabled {
                            list_item_el
                                .w_full()
                                .cursor_pointer()
                                .hover(move |s| s.bg(item_hover_bg))
                                .on_click(move |_ev, window, cx| {
                                    if let Some(st) = state_for_click.as_ref() {
                                        st.update(cx, |s, _cx| s.close());
                                    }
                                    if let Some(cb) = on_select_for_click.as_ref() {
                                        cb(item_id_for_callback.clone(), window, cx);
                                    }
                                })
                        } else {
                            list_item_el.w_full()
                        };
                        menu = menu.child(item_el);
                    }
                    DropdownItem::Separator => {
                        menu = menu.child(div().h(px(1.)).bg(divider_color).my(px(2.)));
                    }
                    DropdownItem::Group(_) => {}
                }
            }

            // The animation wrapper is absolutely positioned at the
            // top-left of the root relative container so the menu
            // inside keeps its original `top/left` offset.
            let distance = px(theme.get_number("motion.slide_distance").unwrap_or(10.0) as f32);
            let state_entity = props
                .state
                .clone()
                .expect("visible implies state is present");
            root.child(
                gpui::deferred(div().absolute().top_0().left_0().child(
                    AnimatedPresenceElement::new(
                        state_entity,
                        (props.id.clone(), "menu"),
                        SlideDirection::Down,
                        distance,
                        div().child(menu),
                    ),
                ))
                .with_priority(1),
            )
        } else {
            root
        }
    }
}

// =====================================================================
// ButtonGroup
// =====================================================================

pub use yororen_ui_core::renderer::button_group::{ButtonGroupRenderState, ButtonGroupRenderer};

pub struct XpButtonGroupRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpButtonGroupRenderer {
    /// Gap between children in **detached** mode. XP toolbars keep
    /// buttons close together; in attached (segmented) mode the
    /// gap is always 0.
    pub fn gap(&self, _state: &ButtonGroupRenderState, theme: &Theme) -> f32 {
        theme
            .get_number("tokens.control.button_group.gap")
            .unwrap_or(2.0) as f32
    }

    /// Corner radius the first / last button inherit.
    pub fn radius(&self, _state: &ButtonGroupRenderState, theme: &Theme) -> Pixels {
        button_radius(theme)
    }

    /// Border colour for the shared group border (only drawn
    /// in attached mode).
    pub fn border_color(&self, _state: &ButtonGroupRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.button.border", button_border())
    }
}

impl ButtonGroupRenderer for XpButtonGroupRenderer {
    fn compose(
        &self,
        props: yororen_ui_core::headless::button_group::ButtonGroupProps,
        cx: &App,
    ) -> Stateful<Div> {
        use yororen_ui_core::headless::button_group::ButtonGroupOrientation;

        let theme = cx.theme();
        let state = ButtonGroupRenderState {
            orientation: props.orientation,
            attached: props.attached,
        };
        let n = props.children.len();
        let id = props.id;
        let children = props.children;

        // Container: flex row/column. The border, radius, and
        // overflow are only applied in attached mode — in
        // detached mode each child keeps its own XP frame.
        let mut container = match props.orientation {
            ButtonGroupOrientation::Horizontal => div().flex().flex_row().items_center(),
            ButtonGroupOrientation::Vertical => div().flex().flex_col().items_center(),
        };

        if props.attached && n > 0 {
            let radius = self.radius(&state, theme);
            let border = self.border_color(&state, theme);
            // In segmented mode the group itself owns a 1px
            // border around the whole row/column and the
            // children's individual borders butt up against it.
            container = container
                .overflow_hidden()
                .rounded(radius)
                .border(px(XP_BORDER_WIDTH))
                .border_color(border);
        } else {
            let gap = px(self.gap(&state, theme));
            container = container.gap(gap);
        }

        // Process children: in attached mode strip the inner
        // children's border-radius so they butt up cleanly and
        // re-add the outer corners to the first / last child.
        let mut iter = children.into_iter();
        for i in 0..n {
            let Some(mut child) = iter.next() else { break };
            if props.attached && n > 1 {
                let radius = self.radius(&state, theme);
                child = child.rounded(px(0.));
                if i == 0 {
                    if props.orientation == ButtonGroupOrientation::Horizontal {
                        child = child.rounded_tl(radius).rounded_bl(radius);
                    } else {
                        child = child.rounded_tl(radius).rounded_tr(radius);
                    }
                } else if i + 1 == n {
                    if props.orientation == ButtonGroupOrientation::Horizontal {
                        child = child.rounded_tr(radius).rounded_br(radius);
                    } else {
                        child = child.rounded_bl(radius).rounded_br(radius);
                    }
                }
            }
            container = container.child(child);
        }

        container.id(id)
    }
}
