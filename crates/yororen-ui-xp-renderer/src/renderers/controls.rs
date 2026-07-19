//! XP (Luna) control renderers: `Switch`, `Checkbox`, `Radio`,
//! `RadioGroup`, `Slider`.
//!
//! Checkboxes and radios are the classic 13px white sunken boxes
//! with a thin gray-blue edge — a blue check / blue dot carries
//! the checked state. The switch has no XP counterpart, so it is
//! approximated with a short beveled track and a cream-gradient
//! knob; the slider pairs a thin etched channel (dark top/left
//! edge, light bottom/right edge) with the same gradient thumb.
//! Plain controls never cast a shadow in XP.

use std::sync::{Arc, Mutex};

use gpui::{
    App, Background, BorderStyle, Bounds, Corners, CursorStyle, Div, Edges as GpuiEdges, Element,
    FocusHandle, GlobalElementId, Hsla, InteractiveElement, IntoElement, LayoutId, PaintQuad,
    ParentElement, Pixels, Stateful, StatefulInteractiveElement, Style, Styled, Window, div, hsla,
    point, px, size,
};
use yororen_ui_core::theme::ActiveTheme;
use yororen_ui_core::theme::Theme;
use yororen_ui_default_renderer::animation::{AnimatedMarginElement, AnimatedOpacityElement};

use crate::style::{
    XP_BORDER_WIDTH, XP_RADIUS, bevel_inner_dark, bevel_inner_light, bevel_outer_light,
    button_border, button_face, check_glyph, hsl_fallback, input_bg, input_border,
    input_focus_border, selection_fg, selection_hover_bg, xp_color,
};

// =====================================================================
// Switch
// =====================================================================

pub use yororen_ui_core::renderer::switch::{SwitchRenderState, SwitchRenderer};

pub struct XpSwitchRenderer;

// Inherent helpers — *not* part of the `SwitchRenderer` trait surface.
impl XpSwitchRenderer {
    pub fn track_w(&self, _: &SwitchRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.switch.track_w")
            .unwrap_or(40.0) as f32)
    }
    pub fn track_h(&self, _: &SwitchRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.switch.track_h")
            .unwrap_or(16.0) as f32)
    }
    pub fn knob_size(&self, _: &SwitchRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.switch.knob_size")
            .unwrap_or(12.0) as f32)
    }
    pub fn padding(&self, _: &SwitchRenderState, _: &Theme) -> Pixels {
        px(1.0)
    }
    pub fn track_radius(&self, _: &SwitchRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.switch.radius")
            .unwrap_or(XP_RADIUS as f64) as f32)
    }
    pub fn track_bg(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme
                .get_color("action.neutral.disabled_bg")
                .unwrap_or(bevel_inner_dark())
        } else if state.checked {
            if state.has_custom_tone {
                state.custom_tone.unwrap_or(selection_hover_bg())
            } else {
                // A pale-blue wash marks the "on" side; the track
                // itself stays a shallow sunken groove.
                xp_color(theme, "xp.selection.hover_bg", selection_hover_bg())
            }
        } else {
            xp_color(theme, "xp.input.bg", input_bg())
        }
    }
    pub fn track_border(&self, _state: &SwitchRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
    }
    pub fn track_hover_bg(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla {
        if state.checked {
            xp_color(theme, "xp.selection.hover_border", hsl_fallback(0x99B4D1))
        } else {
            xp_color(theme, "xp.selection.hover_bg", selection_hover_bg())
        }
    }
    pub fn track_active_bg(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla {
        if state.checked {
            xp_color(theme, "xp.selection.hover_border", hsl_fallback(0x99B4D1))
        } else {
            xp_color(theme, "xp.bevel.inner_light", bevel_inner_light())
        }
    }
    /// The knob is a tiny XP button: cream vertical gradient,
    /// falling back to a flat solid colour when disabled.
    pub fn knob_face(&self, state: &SwitchRenderState, theme: &Theme) -> Background {
        if state.disabled {
            theme
                .get_color("action.neutral.disabled_bg")
                .unwrap_or(bevel_inner_dark())
                .into()
        } else {
            button_face(theme)
        }
    }
    pub fn knob_border(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
        } else {
            xp_color(theme, "xp.button.border", button_border())
        }
    }
    pub fn focus_color(&self, _: &SwitchRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("border.focus")
            .unwrap_or(input_focus_border())
    }
    pub fn disabled_opacity(&self, _: &SwitchRenderState, _: &Theme) -> f32 {
        // XP disabled controls use flat solid colours, not
        // translucency.
        1.0
    }
}

impl SwitchRenderer for XpSwitchRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::switch::SwitchProps,
        focus_handle: &FocusHandle,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = SwitchRenderState {
            checked: props.checked,
            disabled: props.disabled,
            has_custom_tone: props.has_custom_tone,
            custom_tone: props.custom_tone,
        };
        let track = self.track_bg(&state, theme);
        let w = self.track_w(&state, theme);
        let h = self.track_h(&state, theme);
        let knob_size = self.knob_size(&state, theme);
        let pad = self.padding(&state, theme);
        let radius = self.track_radius(&state, theme);
        let track_hover = self.track_hover_bg(&state, theme);
        let track_active = self.track_active_bg(&state, theme);
        let border = self.track_border(&state, theme);
        let knob_border = self.knob_border(&state, theme);
        let border_w = px(theme
            .get_number("tokens.control.switch.border_width")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32);

        // Cross-fade the knob face between unchecked and checked
        // states while it slides (both states use the same cream
        // gradient — the wrapper is kept for structural parity
        // with the reference renderer).
        let unchecked_knob_face = self.knob_face(
            &SwitchRenderState {
                checked: false,
                ..state
            },
            theme,
        );
        let checked_knob_face = self.knob_face(
            &SwitchRenderState {
                checked: true,
                ..state
            },
            theme,
        );
        let knob_off = div()
            .absolute()
            .inset_0()
            .bg(unchecked_knob_face)
            .border(border_w)
            .border_color(knob_border)
            .rounded(px(2.));
        let knob_on = div()
            .absolute()
            .inset_0()
            .bg(checked_knob_face)
            .border(border_w)
            .border_color(knob_border)
            .rounded(px(2.));
        let knob_inner = div()
            .relative()
            .size(knob_size)
            .child(AnimatedOpacityElement::new(
                (props.id.clone(), "knob-off"),
                !props.checked,
                knob_off,
            ))
            .child(AnimatedOpacityElement::new(
                (props.id.clone(), "knob-on"),
                props.checked,
                knob_on,
            ));

        let slide_distance = {
            let w_f: f32 = w.into();
            let knob_f: f32 = knob_size.into();
            let pad_f: f32 = pad.into();
            px((w_f - knob_f - pad_f * 2.0).max(0.0))
        };
        let knob_animated = AnimatedMarginElement::new(
            (props.id.clone(), "knob-slide"),
            props.checked,
            slide_distance,
            knob_inner,
        );

        div()
            .id(props.id.clone())
            .bg(track)
            .border(border_w)
            .border_color(border)
            .rounded(radius)
            .w(w)
            .h(h)
            .p(pad)
            .flex()
            .items_center()
            .justify_start()
            .track_focus(focus_handle)
            .child(knob_animated)
            .hover(|s| s.bg(track_hover))
            .active(|s| s.bg(track_active))
            .cursor(if props.disabled {
                CursorStyle::OperationNotAllowed
            } else {
                CursorStyle::PointingHand
            })
    }
}

// =====================================================================
// Checkbox
// =====================================================================

pub use yororen_ui_core::renderer::checkbox::{CheckboxRenderState, CheckboxRenderer};

pub struct XpCheckboxRenderer;

// Inherent helpers — *not* part of the `CheckboxRenderer` trait surface.
impl XpCheckboxRenderer {
    pub fn box_size(&self, _: &CheckboxRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.checkbox.size")
            .unwrap_or(13.0) as f32)
    }
    /// Point size of the check glyph — a little smaller than the
    /// box so it sits comfortably inside the 1px border.
    pub fn check_size(&self, _: &CheckboxRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.checkbox.size")
            .unwrap_or(13.0) as f32
            * 0.8)
    }
    pub fn box_bg(&self, state: &CheckboxRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme
                .get_color("action.neutral.disabled_bg")
                .unwrap_or(bevel_inner_dark())
        } else if state.checked && state.has_custom_tone {
            state.custom_tone.unwrap_or(input_bg())
        } else {
            // XP checkboxes stay white in every enabled state —
            // the blue glyph carries the checked state.
            xp_color(theme, "xp.check.box_bg", input_bg())
        }
    }
    pub fn box_border(&self, state: &CheckboxRenderState, theme: &Theme) -> Hsla {
        if state.checked && state.has_custom_tone {
            state.custom_tone.unwrap_or(input_border())
        } else {
            xp_color(theme, "xp.check.box_border", input_border())
        }
    }
    pub fn box_hover_bg(&self, state: &CheckboxRenderState, theme: &Theme) -> Hsla {
        if state.checked {
            xp_color(theme, "xp.selection.hover_bg", selection_hover_bg())
        } else {
            theme
                .get_color("surface.hover")
                .unwrap_or(selection_hover_bg())
        }
    }
    pub fn box_active_bg(&self, state: &CheckboxRenderState, theme: &Theme) -> Hsla {
        if state.checked {
            xp_color(theme, "xp.selection.hover_border", hsl_fallback(0x99B4D1))
        } else {
            xp_color(theme, "xp.bevel.inner_light", bevel_inner_light())
        }
    }
    pub fn check_fg(&self, state: &CheckboxRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme
                .get_color("content.disabled")
                .unwrap_or(hsl_fallback(0xA1A192))
        } else if state.checked && state.has_custom_tone {
            // Custom-tone boxes are filled, so the glyph flips to
            // the "on primary" colour for contrast.
            theme
                .get_color("action.primary.fg")
                .unwrap_or(selection_fg())
        } else {
            xp_color(theme, "xp.check.glyph", check_glyph())
        }
    }
    pub fn focus_color(&self, _: &CheckboxRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("border.focus")
            .unwrap_or(input_focus_border())
    }
    pub fn disabled_opacity(&self, _: &CheckboxRenderState, _: &Theme) -> f32 {
        1.0
    }
}

impl CheckboxRenderer for XpCheckboxRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::checkbox::CheckboxProps,
        focus_handle: &FocusHandle,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = CheckboxRenderState {
            checked: props.checked,
            disabled: props.disabled,
            has_custom_tone: props.has_custom_tone,
            custom_tone: props.custom_tone,
        };
        let bg = self.box_bg(&state, theme);
        let border = self.box_border(&state, theme);
        let size = self.box_size(&state, theme);
        let check_size = self.check_size(&state, theme);
        let hover_bg = self.box_hover_bg(&state, theme);
        let active_bg = self.box_active_bg(&state, theme);
        let border_w = px(theme
            .get_number("tokens.control.checkbox.border_width")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32);
        let radius = px(theme
            .get_number("tokens.control.checkbox.radius")
            .unwrap_or(0.0) as f32);

        // The check glyph is always mounted and faded in/out so
        // the checked state transition is animated.
        let glyph_color = self.check_fg(&state, theme);
        let check = div()
            .flex()
            .items_center()
            .justify_center()
            .size(check_size)
            .text_size(check_size)
            .text_color(glyph_color)
            .child("✓");
        let animated_check =
            AnimatedOpacityElement::new((props.id.clone(), "check"), props.checked, check);

        div()
            .id(props.id.clone())
            .bg(bg)
            .border(border_w)
            .border_color(border)
            .size(size)
            .rounded(radius)
            .flex()
            .items_center()
            .justify_center()
            .track_focus(focus_handle)
            .child(animated_check)
            .hover(|s| s.bg(hover_bg))
            .active(|s| s.bg(active_bg))
            .cursor(if props.disabled {
                CursorStyle::OperationNotAllowed
            } else {
                CursorStyle::PointingHand
            })
    }
}

// =====================================================================
// Radio
// =====================================================================

pub use yororen_ui_core::renderer::radio::{RadioRenderState, RadioRenderer};

pub struct XpRadioRenderer;

// Inherent helpers — *not* part of the `RadioRenderer` trait surface.
impl XpRadioRenderer {
    pub fn ring_size(&self, _: &RadioRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.radio.size")
            .unwrap_or(13.0) as f32)
    }
    /// The dot is a small filled circle centered in the ring —
    /// roughly 40% of the ring diameter.
    pub fn dot_size(&self, _: &RadioRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.radio.size")
            .unwrap_or(13.0) as f32
            * 0.4)
    }
    pub fn ring_bg(&self, state: &RadioRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme
                .get_color("action.neutral.disabled_bg")
                .unwrap_or(bevel_inner_dark())
        } else {
            xp_color(theme, "xp.check.box_bg", input_bg())
        }
    }
    pub fn ring_border(&self, state: &RadioRenderState, theme: &Theme) -> Hsla {
        if state.checked && state.has_custom_tone {
            state.custom_tone.unwrap_or(input_border())
        } else {
            xp_color(theme, "xp.check.box_border", input_border())
        }
    }
    pub fn ring_hover_bg(&self, _: &RadioRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("surface.hover")
            .unwrap_or(selection_hover_bg())
    }
    pub fn ring_active_bg(&self, _: &RadioRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_light", bevel_inner_light())
    }
    pub fn dot_fg(&self, state: &RadioRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme
                .get_color("content.disabled")
                .unwrap_or(hsl_fallback(0xA1A192))
        } else if state.has_custom_tone {
            state.custom_tone.unwrap_or(check_glyph())
        } else {
            xp_color(theme, "xp.check.glyph", check_glyph())
        }
    }
    pub fn focus_color(&self, _: &RadioRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("border.focus")
            .unwrap_or(input_focus_border())
    }
    pub fn disabled_opacity(&self, _: &RadioRenderState, _: &Theme) -> f32 {
        1.0
    }
}

impl RadioRenderer for XpRadioRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::radio::RadioProps,
        focus_handle: &FocusHandle,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = RadioRenderState {
            checked: props.checked,
            disabled: props.disabled,
            has_custom_tone: props.has_custom_tone,
            custom_tone: props.custom_tone,
        };
        let bg = self.ring_bg(&state, theme);
        let border = self.ring_border(&state, theme);
        let ring_size = self.ring_size(&state, theme);
        let dot_size = self.dot_size(&state, theme);
        let dot_fg = self.dot_fg(&state, theme);
        let hover_bg = self.ring_hover_bg(&state, theme);
        let active_bg = self.ring_active_bg(&state, theme);
        let border_w = px(theme
            .get_number("tokens.control.radio.border_width")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32);

        let mut el: Stateful<Div> = div()
            .id(props.id.clone())
            .bg(bg)
            .border(border_w)
            .border_color(border)
            .size(ring_size)
            .rounded(px(9999.))
            .flex()
            .items_center()
            .justify_center()
            .track_focus(focus_handle);
        if props.checked {
            el = el.child(div().bg(dot_fg).size(dot_size).rounded(px(9999.)));
        }
        el.hover(|s| s.bg(hover_bg))
            .active(|s| s.bg(active_bg))
            .cursor(if props.disabled {
                CursorStyle::OperationNotAllowed
            } else {
                CursorStyle::PointingHand
            })
    }
}

// =====================================================================
// RadioGroup
// =====================================================================

pub use yororen_ui_core::renderer::radio_group::{RadioGroupRenderState, RadioGroupRenderer};

pub struct XpRadioGroupRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpRadioGroupRenderer {
    /// Horizontal gap between radio buttons.
    pub fn gap(&self, _state: &RadioGroupRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.spacing.gap_3")
            .or_else(|| theme.get_number("tokens.spacing.normal"))
            .unwrap_or(12.0) as f32)
    }
}

impl RadioGroupRenderer for XpRadioGroupRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::radio_group::RadioGroupProps,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = RadioGroupRenderState {
            selected_index: props.selected_index,
        };
        let gap = self.gap(&state, theme);

        div()
            .id(props.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .gap(gap)
    }
}

// =====================================================================
// Slider
// =====================================================================

pub use yororen_ui_core::renderer::slider::{
    SliderRenderOutput, SliderRenderState, SliderRenderer,
};

pub struct XpSliderRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpSliderRenderer {
    pub fn track_h(&self, _state: &SliderRenderState, theme: &Theme) -> f32 {
        theme
            .get_number("tokens.control.slider.track_h")
            .unwrap_or(6.0) as f32
    }
    pub fn knob_size(&self, _state: &SliderRenderState, theme: &Theme) -> f32 {
        theme
            .get_number("tokens.control.slider.thumb_size")
            .unwrap_or(20.0) as f32
    }
    pub fn track_w(&self, _state: &SliderRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.slider.track_w")
            .unwrap_or(240.0) as f32)
    }
    pub fn border_width(&self, _state: &SliderRenderState, theme: &Theme) -> f32 {
        theme
            .get_number("tokens.control.slider.border_width")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32
    }
    pub fn track_bg(&self, state: &SliderRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme
                .get_color("action.neutral.disabled_bg")
                .unwrap_or(bevel_inner_dark())
        } else {
            xp_color(theme, "xp.input.bg", input_bg())
        }
    }
    /// XP trackbars have no filled portion — the fill quad stays
    /// for structural parity with the reference renderer but just
    /// repaints the well colour (inset so it never covers the
    /// etched edges).
    pub fn fill_bg(&self, state: &SliderRenderState, theme: &Theme) -> Hsla {
        self.track_bg(state, theme)
    }
    /// The thumb is a tiny XP button: cream vertical gradient,
    /// falling back to a flat solid colour when disabled.
    pub fn knob_bg(&self, state: &SliderRenderState, theme: &Theme) -> Background {
        if state.disabled {
            theme
                .get_color("action.neutral.disabled_bg")
                .unwrap_or(bevel_inner_dark())
                .into()
        } else {
            button_face(theme)
        }
    }
    pub fn track_border(&self, _state: &SliderRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
    }
    /// White bottom/right edge completing the sunken double
    /// border; transparent when disabled (flat look).
    pub fn track_highlight(&self, state: &SliderRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            hsla(0., 0., 0., 0.)
        } else {
            xp_color(theme, "xp.bevel.outer_light", bevel_outer_light())
        }
    }
    pub fn knob_border(&self, state: &SliderRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
        } else {
            xp_color(theme, "xp.button.border", button_border())
        }
    }
}

impl SliderRenderer for XpSliderRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::slider::SliderProps,
        cx: &App,
    ) -> SliderRenderOutput {
        let theme = cx.theme();
        let state = SliderRenderState {
            disabled: props.disabled,
        };
        let track_h = self.track_h(&state, theme);
        let knob_size = self.knob_size(&state, theme);
        let track_w = self.track_w(&state, theme);
        let border_w = self.border_width(&state, theme);
        let track_bg = self.track_bg(&state, theme);
        let fill_bg = self.fill_bg(&state, theme);
        let knob_bg = self.knob_bg(&state, theme);
        let track_border = self.track_border(&state, theme);
        let track_highlight = self.track_highlight(&state, theme);
        let knob_border = self.knob_border(&state, theme);

        let pct = ((props.value - props.min) / (props.max - props.min)).clamp(0.0, 1.0);

        let bounds_store: Arc<Mutex<Option<Bounds<Pixels>>>> = Arc::new(Mutex::new(None));

        // The total row height is enough for the thumb plus the
        // etched track edges, with a small gap.
        let row_h = (knob_size + border_w * 2.0 + 4.0).max(30.0);

        let track_element = XpSliderTrackElement {
            bounds: bounds_store.clone(),
            pct,
            row_h,
            track_h,
            knob_size,
            border_w,
            track_bg,
            fill_bg,
            knob_bg,
            track_border,
            track_highlight,
            knob_border,
        };

        let visual = div()
            .id(props.id.clone())
            .w(track_w)
            .h(px(row_h))
            .cursor(if props.disabled {
                CursorStyle::OperationNotAllowed
            } else {
                CursorStyle::PointingHand
            })
            .child(track_element);

        SliderRenderOutput {
            visual,
            track_bounds: bounds_store,
        }
    }
}

/// Internal `Element` painting the XP slider track, fill, and
/// thumb. Same bounds-publishing contract as the default
/// renderer so the headless drag handlers in
/// `SliderProps::render` can resolve mouse positions.
struct XpSliderTrackElement {
    bounds: Arc<Mutex<Option<Bounds<Pixels>>>>,
    pct: f32,
    row_h: f32,
    track_h: f32,
    knob_size: f32,
    border_w: f32,
    track_bg: Hsla,
    fill_bg: Hsla,
    knob_bg: Background,
    track_border: Hsla,
    track_highlight: Hsla,
    knob_border: Hsla,
}

impl IntoElement for XpSliderTrackElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for XpSliderTrackElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<gpui::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = gpui::relative(1.0).into();
        style.size.height = px(self.row_h).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        *self.bounds.lock().unwrap() = Some(bounds);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        _cx: &mut App,
    ) {
        let track_y = bounds.top() + px((self.row_h - self.track_h) / 2.0);
        let knob_y = bounds.top() + px((self.row_h - self.knob_size) / 2.0);
        let track_w: f32 = bounds.size.width.into();
        // The XP slider thumb is a tall, narrow paddle, not a
        // square knob (~11×21 at the default token size).
        let knob_w = self.knob_size * 0.55;
        let fill_w = px(self.pct * (track_w - knob_w));
        let knob_x = bounds.left() + px(self.pct * (track_w - knob_w));

        // Track — a thin sunken channel: dark top/left edge…
        let track_bounds = Bounds::new(
            point(bounds.left(), track_y),
            size(bounds.size.width, px(self.track_h)),
        );
        window.paint_quad(PaintQuad {
            bounds: track_bounds,
            corner_radii: Corners::all(px(0.0)),
            background: self.track_bg.into(),
            border_color: self.track_border,
            border_widths: GpuiEdges {
                top: px(self.border_w),
                left: px(self.border_w),
                ..Default::default()
            },
            border_style: BorderStyle::Solid,
        });

        // …and a white bottom/right highlight edge, completing
        // the Win32 sunken double border.
        window.paint_quad(PaintQuad {
            bounds: track_bounds,
            corner_radii: Corners::all(px(0.0)),
            background: hsla(0., 0., 0., 0.).into(),
            border_color: self.track_highlight,
            border_widths: GpuiEdges {
                right: px(self.border_w),
                bottom: px(self.border_w),
                ..Default::default()
            },
            border_style: BorderStyle::Solid,
        });

        // Fill — inset inside the etched edges; repaints the
        // well colour (XP trackbars have no filled portion).
        let fill_bounds = Bounds::new(
            point(
                bounds.left() + px(self.border_w),
                track_y + px(self.border_w),
            ),
            size(fill_w, px((self.track_h - self.border_w * 2.0).max(0.0))),
        );
        window.paint_quad(PaintQuad {
            bounds: fill_bounds,
            corner_radii: Corners::all(px(0.0)),
            background: self.fill_bg.into(),
            border_color: hsla(0., 0., 0., 0.),
            border_widths: GpuiEdges::default(),
            border_style: BorderStyle::default(),
        });

        // Thumb — cream-gradient XP button face with a 1px
        // olive edge and 2px corners.
        let knob_bounds = Bounds::new(point(knob_x, knob_y), size(px(knob_w), px(self.knob_size)));
        window.paint_quad(PaintQuad {
            bounds: knob_bounds,
            corner_radii: Corners::all(px(2.0)),
            background: self.knob_bg,
            border_color: self.knob_border,
            border_widths: GpuiEdges::all(px(self.border_w)),
            border_style: BorderStyle::Solid,
        });
    }
}
