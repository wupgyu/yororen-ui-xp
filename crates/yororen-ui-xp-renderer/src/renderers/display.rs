//! XP (Luna) display renderers: `Label`, `Heading`, `Divider`,
//! `FocusRing`, `Badge`, `Tag`, `Skeleton`, `ProgressBar`,
//! `EmptyState`, `KeybindingDisplay`, `ShortcutHint`, `Icon`,
//! `Text`, `Spacer`.
//!
//! The signature piece is the segmented `ProgressBar`: a sunken
//! white well filled with Luna-green chunks. Badges, tags and
//! key-cap chips are small raised button-face elements; the
//! skeleton pulses with the Win32 bevel colors.

use gpui::{
    AbsoluteLength, AlignSelf, App, Background, BorderStyle, Bounds, BoxShadow, Corners,
    CursorStyle, DefiniteLength, Div, Edges, Element, ElementId, FontWeight, GlobalElementId, Hsla,
    InspectorElementId, InteractiveElement, IntoElement, LayoutId, Length, PaintQuad, Pixels,
    Position, SharedString, Stateful, StatefulInteractiveElement, Style, Styled, Window, hsla,
    point, px, size,
};
use std::sync::OnceLock;
use std::time::Instant;

use yororen_ui_core::animation::ease_in_out;
use yororen_ui_core::headless::badge::BadgeVariant;
use yororen_ui_core::headless::icon::IconSource;
use yororen_ui_core::headless::label::LabelProps;
use yororen_ui_core::renderer::spec::Edges as SpecEdges;
use yororen_ui_core::theme::ActiveTheme;
use yororen_ui_core::theme::Theme;

use gpui::ParentElement;

use crate::style::{
    XP_BORDER_WIDTH, XP_FONT_FAMILY, XP_PROGRESS_SEGMENT_GAP, XP_PROGRESS_SEGMENT_W, XP_RADIUS,
    bevel_inner_dark, bevel_inner_light, bevel_outer_light, button_border, button_face, darken,
    dialog_bg, hsl_fallback, input_focus_border, lighten, progress_chunk_border,
    progress_chunk_gradient, progress_track_bg, progress_track_border, selection_bg, selection_fg,
    selection_hover_bg, vgrad, xp_color, xp_number,
};

// =====================================================================
// Label
// =====================================================================

pub use yororen_ui_core::renderer::label::{LabelRenderState, LabelRenderer};

pub struct XpLabelRenderer;

// Inherent helpers — *not* part of the `LabelRenderer` trait
// surface.
impl XpLabelRenderer {
    pub fn color(&self, state: &LabelRenderState, theme: &Theme) -> Hsla {
        if state.muted {
            theme
                .get_color("content.secondary")
                .unwrap_or(hsl_fallback(0x2B2B23))
        } else {
            theme
                .get_color("content.primary")
                .unwrap_or(hsl_fallback(0x000000))
        }
    }

    pub fn strong_weight(&self, _: &LabelRenderState, theme: &Theme) -> FontWeight {
        FontWeight(
            theme
                .get_number("tokens.control.label.weight")
                .or_else(|| theme.get_number("tokens.typography.weight_medium"))
                .unwrap_or(700.0) as f32,
        )
    }

    pub fn family_mono(&self, _: &LabelRenderState, theme: &Theme) -> SharedString {
        theme
            .get_string("tokens.typography.family_mono")
            .unwrap_or(XP_FONT_FAMILY)
            .to_string()
            .into()
    }
}

impl LabelRenderer for XpLabelRenderer {
    fn compose(&self, props: &LabelProps, cx: &App) -> Div {
        let theme = cx.theme();
        let state = LabelRenderState {
            muted: props.muted,
            strong: props.strong,
            mono: props.mono,
            inherit_color: props.inherit_color,
            ellipsis: props.ellipsis,
            wrap: props.wrap,
            max_lines: props.max_lines,
        };
        let color = self.color(&state, theme);
        let weight = self.strong_weight(&state, theme);
        let family = self.family_mono(&state, theme);
        let mut el: Div = gpui::div();
        if !props.inherit_color {
            el = el.text_color(color);
        }
        if props.strong {
            el = el.font_weight(weight);
        }
        if props.mono {
            el = el.font_family(family);
        }
        if props.ellipsis {
            el = el.overflow_hidden().text_ellipsis().whitespace_nowrap();
        }
        if props.wrap {
            el = el.whitespace_normal();
        }
        if let Some(n) = props.max_lines {
            el = el.line_clamp(n).overflow_hidden();
        }
        el.child(props.text.clone())
    }
}

// =====================================================================
// Heading
// =====================================================================

pub use yororen_ui_core::renderer::heading::{HeadingRenderState, HeadingRenderer};

pub struct XpHeadingRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpHeadingRenderer {
    pub fn size(&self, state: &HeadingRenderState, theme: &Theme) -> Pixels {
        let (path, fallback) = match state.level {
            yororen_ui_core::headless::heading::HeadingLevel::H1 => {
                ("tokens.control.heading.font_size_lg", 28.0)
            }
            yororen_ui_core::headless::heading::HeadingLevel::H2 => {
                ("tokens.control.heading.font_size_md", 22.0)
            }
            _ => ("tokens.control.heading.font_size_sm", 17.0),
        };
        px(theme.get_number(path).unwrap_or(fallback) as f32)
    }

    pub fn weight(&self, state: &HeadingRenderState, theme: &Theme) -> FontWeight {
        let default = match state.level {
            yororen_ui_core::headless::heading::HeadingLevel::H1 => 700.0,
            _ => 700.0,
        };
        FontWeight(
            theme
                .get_number("tokens.control.heading.weight")
                .or_else(|| theme.get_number("tokens.typography.weight_bold"))
                .unwrap_or(default) as f32,
        )
    }

    pub fn color(&self, _: &HeadingRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.primary")
            .unwrap_or(hsl_fallback(0x000000))
    }
}

impl HeadingRenderer for XpHeadingRenderer {
    fn compose(&self, props: &yororen_ui_core::headless::heading::HeadingProps, cx: &App) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = HeadingRenderState { level: props.level };
        let size = self.size(&state, theme);
        let weight = self.weight(&state, theme);
        let color = self.color(&state, theme);
        gpui::div()
            .text_color(color)
            .text_size(size)
            .font_weight(weight)
            .child(props.text.clone())
    }
}

// =====================================================================
// Divider
// =====================================================================

pub use yororen_ui_core::renderer::divider::{DividerRenderState, DividerRenderer};

pub struct XpDividerRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpDividerRenderer {
    pub fn color(&self, _: &DividerRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("border.divider")
            .unwrap_or(bevel_inner_dark())
    }
    pub fn thickness(&self, _: &DividerRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.divider.thickness")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32)
    }
}

impl DividerRenderer for XpDividerRenderer {
    fn compose(&self, props: &yororen_ui_core::headless::divider::DividerProps, cx: &App) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = DividerRenderState {
            horizontal: props.horizontal,
        };
        let groove = self.color(&state, theme);
        let highlight = xp_color(theme, "xp.bevel.outer_light", bevel_outer_light());
        let thickness = self.thickness(&state, theme);
        // Etched double line: a dark groove next to a white
        // highlight — the classic Win32 sunken separator.
        let mut el = gpui::div().flex_shrink_0();
        if props.horizontal {
            el.style().align_self = Some(AlignSelf::Stretch);
            el.w_full()
                .flex()
                .flex_col()
                .child(
                    gpui::div()
                        .w_full()
                        .h(thickness)
                        .min_h(thickness)
                        .bg(groove),
                )
                .child(
                    gpui::div()
                        .w_full()
                        .h(thickness)
                        .min_h(thickness)
                        .bg(highlight),
                )
        } else {
            el.style().align_self = Some(AlignSelf::Stretch);
            el.h_full()
                .flex()
                .flex_row()
                .child(
                    gpui::div()
                        .h_full()
                        .w(thickness)
                        .min_w(thickness)
                        .bg(groove),
                )
                .child(
                    gpui::div()
                        .h_full()
                        .w(thickness)
                        .min_w(thickness)
                        .bg(highlight),
                )
        }
    }
}

// =====================================================================
// FocusRing
// =====================================================================

pub use yororen_ui_core::renderer::focus_ring::{FocusRingRenderState, FocusRingRenderer};

pub struct XpFocusRingRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpFocusRingRenderer {
    pub fn color(&self, _: &FocusRingRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("border.focus")
            .unwrap_or(input_focus_border())
    }

    pub fn width(&self, _: &FocusRingRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.focus_ring.width")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32)
    }

    /// XP controls round their corners at `XP_RADIUS` (3), so the
    /// focus ring follows suit. The wrapper's `rounded(...)` is
    /// what the `box-shadow` follows; the 1px solid ring
    /// approximates the classic dotted focus rectangle.
    pub fn border_radius(&self, _: &FocusRingRenderState, _: &Theme) -> Pixels {
        px(XP_RADIUS)
    }
}

impl FocusRingRenderer for XpFocusRingRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::focus_ring::FocusRingProps,
        cx: &App,
    ) -> Stateful<Div> {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = FocusRingRenderState {
            has_custom_color: props.has_custom_color,
        };
        let color = self.color(&state, theme);
        let width = self.width(&state, theme);
        let radius = self.border_radius(&state, theme);
        gpui::div()
            .id(props.id.clone())
            .track_focus(&props.focus_handle)
            .rounded(radius)
            .shadow(vec![BoxShadow {
                color,
                offset: point(px(0.), px(0.)),
                blur_radius: px(0.),
                spread_radius: width,
            }])
    }
}

// =====================================================================
// Badge
// =====================================================================

pub use yororen_ui_core::renderer::badge::{BadgeRenderState, BadgeRenderer};

fn badge_variant_key(variant: BadgeVariant) -> &'static str {
    match variant {
        BadgeVariant::Neutral => "neutral",
        BadgeVariant::Success => "success",
        BadgeVariant::Warning => "warning",
        BadgeVariant::Danger => "danger",
        BadgeVariant::Info => "info",
    }
}

pub struct XpBadgeRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpBadgeRenderer {
    /// Neutral badges are small raised chips in the button face;
    /// status variants keep their status hue as a light → base
    /// vertical gradient, like a tiny XP button.
    pub fn bg(&self, state: &BadgeRenderState, theme: &Theme) -> Background {
        if matches!(state.variant, BadgeVariant::Neutral) {
            return button_face(theme);
        }
        let base = theme
            .get_color(&format!("status.{}.bg", badge_variant_key(state.variant)))
            .unwrap_or(dialog_bg());
        vgrad(lighten(base, 0.12), base)
    }

    pub fn fg(&self, state: &BadgeRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color(&format!("status.{}.fg", badge_variant_key(state.variant)))
            .unwrap_or(hsl_fallback(0x000000))
    }

    pub fn border_color(&self, state: &BadgeRenderState, theme: &Theme) -> Hsla {
        if matches!(state.variant, BadgeVariant::Neutral) {
            return xp_color(theme, "xp.button.border", button_border());
        }
        let base = theme
            .get_color(&format!("status.{}.bg", badge_variant_key(state.variant)))
            .unwrap_or(dialog_bg());
        darken(base, 0.18)
    }

    pub fn padding_x(&self, _: &BadgeRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.badge.horizontal_padding")
            .unwrap_or(6.0) as f32)
    }

    pub fn height(&self, _: &BadgeRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.badge.min_height")
            .unwrap_or(18.0) as f32)
    }

    pub fn font_size(&self, _: &BadgeRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.badge.font_size")
            .unwrap_or(11.0) as f32)
    }

    pub fn font_weight(&self, _: &BadgeRenderState, theme: &Theme) -> FontWeight {
        FontWeight(
            theme
                .get_number("tokens.control.badge.weight")
                .or_else(|| theme.get_number("tokens.typography.weight_bold"))
                .unwrap_or(400.0) as f32,
        )
    }

    pub fn border_radius(&self, _: &BadgeRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.badge.radius")
            .unwrap_or(XP_RADIUS as f64) as f32)
    }
}

impl BadgeRenderer for XpBadgeRenderer {
    fn compose(&self, props: &yororen_ui_core::headless::badge::BadgeProps, cx: &App) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = BadgeRenderState {
            variant: props.variant,
            has_custom_tone: false,
        };
        let bg = self.bg(&state, theme);
        let fg = self.fg(&state, theme);
        let border = self.border_color(&state, theme);
        let px_v = self.padding_x(&state, theme);
        let h = self.height(&state, theme);
        let fs = self.font_size(&state, theme);
        let fw = self.font_weight(&state, theme);
        let r = self.border_radius(&state, theme);
        gpui::div()
            .flex()
            .items_center()
            .justify_center()
            .bg(bg)
            .text_color(fg)
            .px(px_v)
            .h(h)
            .text_size(fs)
            .font_weight(fw)
            .rounded(r)
            .border(px(XP_BORDER_WIDTH))
            .border_color(border)
            .child(props.text.clone())
    }
}

// =====================================================================
// Tag
// =====================================================================

pub use yororen_ui_core::renderer::tag::{TagRenderState, TagRenderer};

pub struct XpTagRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpTagRenderer {
    pub fn bg(&self, state: &TagRenderState, theme: &Theme) -> Background {
        if state.selected {
            xp_color(theme, "xp.selection.bg", selection_bg()).into()
        } else if state.has_custom_tone {
            theme
                .get_color("content.on_status")
                .unwrap_or(hsl_fallback(0x000000))
                .into()
        } else {
            button_face(theme)
        }
    }

    pub fn fg(&self, state: &TagRenderState, theme: &Theme) -> Hsla {
        if state.selected {
            xp_color(theme, "xp.selection.fg", selection_fg())
        } else if state.has_custom_tone {
            theme
                .get_color("content.on_status")
                .unwrap_or(hsl_fallback(0x000000))
        } else {
            theme
                .get_color("action.neutral.fg")
                .unwrap_or(hsl_fallback(0x000000))
        }
    }

    pub fn border_color(&self, state: &TagRenderState, theme: &Theme) -> Hsla {
        if state.selected {
            xp_color(theme, "xp.selection.bg", selection_bg())
        } else {
            xp_color(theme, "xp.button.border", button_border())
        }
    }

    pub fn min_height(&self, _: &TagRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.tag.min_height")
            .unwrap_or(22.0) as f32)
    }

    pub fn padding_x(&self, _: &TagRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.tag.horizontal_padding")
            .unwrap_or(8.0) as f32)
    }

    pub fn font_size(&self, _: &TagRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.typography.font_size_xs")
            .unwrap_or(12.0) as f32)
    }

    pub fn font_weight(&self, _: &TagRenderState, theme: &Theme) -> FontWeight {
        FontWeight(
            theme
                .get_number("tokens.typography.weight_bold")
                .unwrap_or(700.0) as f32,
        )
    }

    pub fn border_radius(&self, _: &TagRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.tag.radius")
            .unwrap_or(XP_RADIUS as f64) as f32)
    }

    pub fn close_size(&self, _: &TagRenderState, _: &Theme) -> Pixels {
        px(16.0)
    }

    pub fn close_hover_bg(&self, _: &TagRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("action.neutral.hover_bg")
            .unwrap_or(selection_hover_bg())
    }
}

impl TagRenderer for XpTagRenderer {
    fn compose(&self, props: &yororen_ui_core::headless::tag::TagProps, cx: &App) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = TagRenderState {
            selected: props.selected,
            has_custom_tone: false,
            closable: props.closable,
        };
        let bg = self.bg(&state, theme);
        let fg = self.fg(&state, theme);
        let border = self.border_color(&state, theme);
        let h = self.min_height(&state, theme);
        let p = self.padding_x(&state, theme);
        let fs = self.font_size(&state, theme);
        let fw = self.font_weight(&state, theme);
        let r = self.border_radius(&state, theme);
        let mut el = gpui::div()
            .flex()
            .items_center()
            .bg(bg)
            .text_color(fg)
            .min_h(h)
            .px(p)
            .text_size(fs)
            .font_weight(fw)
            .rounded(r)
            .gap(p / 2.)
            .border(px(XP_BORDER_WIDTH))
            .border_color(border)
            .cursor(if props.disabled {
                CursorStyle::OperationNotAllowed
            } else if props.on_click.is_some() {
                CursorStyle::PointingHand
            } else {
                CursorStyle::Arrow
            })
            .child(props.label.clone());
        if props.closable {
            let close_size = self.close_size(&state, theme);
            // `on_click` lives on `StatefulInteractiveElement`,
            // which requires an id. Derive a stable, unique id
            // from the tag's own id so the close button gets a
            // distinct identity.
            let close_id: gpui::ElementId = match &props.id {
                gpui::ElementId::Name(name) => {
                    let mut s = name.to_string();
                    s.push_str("__close");
                    s.into()
                }
                _ => "xp_tag_close".into(),
            };
            let mut close_btn = gpui::div()
                .id(close_id)
                .flex()
                .items_center()
                .justify_center()
                .size(close_size)
                .rounded(close_size / 2.)
                .cursor(gpui::CursorStyle::PointingHand)
                .child("×");
            if !props.disabled
                && let Some(f) = props.on_close.clone()
            {
                close_btn = close_btn.on_click(move |ev, window, cx: &mut gpui::App| {
                    cx.stop_propagation();
                    f(ev, window, cx);
                });
            }
            el = el.child(close_btn);
        }
        el
    }
}

// =====================================================================
// Skeleton
// =====================================================================

pub use yororen_ui_core::renderer::skeleton::{SkeletonRenderState, SkeletonRenderer};

pub struct XpSkeletonRenderer;

impl XpSkeletonRenderer {
    pub fn bg(&self, _: &SkeletonRenderState, theme: &Theme) -> Hsla {
        // The pulse rides on the light bevel edge so placeholders
        // read as raised paper on the XP dialog beige.
        xp_color(theme, "xp.bevel.inner_light", bevel_inner_light())
    }
    pub fn min_height(&self, state: &SkeletonRenderState, theme: &Theme) -> Pixels {
        if state.block {
            px(theme
                .get_number("tokens.control.skeleton.block_min_h")
                .unwrap_or(48.0) as f32)
        } else {
            px(theme
                .get_number("tokens.control.skeleton.line_h")
                .unwrap_or(16.0) as f32)
        }
    }
    pub fn border_radius(&self, _: &SkeletonRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.skeleton.radius")
            .unwrap_or(2.0) as f32)
    }
}

/// Pulse opacity range (matches `yororen_ui_core::animation::preset::defaults`).
const XP_SKELETON_PULSE_MIN: f32 = 0.55;
const XP_SKELETON_PULSE_MAX: f32 = 0.95;

/// Animation epoch — all skeletons in the app pulse in sync,
/// which is the standard loading-animation behavior. Captured
/// once on first paint via `OnceLock`.
static XP_SKELETON_PULSE_EPOCH: OnceLock<Instant> = OnceLock::new();

/// A `Length` of zero pixels for `Edges::all` — pins the
/// absolutely-positioned overlay to all four sides of its parent
/// Div (the "fill the parent" idiom).
const XP_ZERO_LENGTH: Length = Length::Definite(DefiniteLength::Absolute(AbsoluteLength::Pixels(
    gpui::px(0.),
)));

/// Custom `gpui::Element` that paints a single rounded quad with
/// a time-varying alpha, producing the skeleton pulse animation.
struct XpSkeletonPulseElement {
    bg: Hsla,
    radius: Pixels,
    duration_ms: u64,
}

impl IntoElement for XpSkeletonPulseElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for XpSkeletonPulseElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let style = Style {
            position: Position::Absolute,
            inset: Edges::all(XP_ZERO_LENGTH),
            ..Default::default()
        };
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let _ = XP_SKELETON_PULSE_EPOCH.get_or_init(Instant::now);
        window.request_animation_frame();
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        _cx: &mut App,
    ) {
        let epoch = XP_SKELETON_PULSE_EPOCH.get_or_init(Instant::now);
        let elapsed_ms = epoch.elapsed().as_millis() as u64;
        let progress = if self.duration_ms == 0 {
            0.0
        } else {
            (elapsed_ms % self.duration_ms) as f32 / self.duration_ms as f32
        };
        // Triangle wave 0 → 1 → 0 so the alpha ramps UP for
        // the first half, then BACK DOWN — a true "breath"
        // instead of a sawtooth that snaps from MAX back to MIN
        // at the cycle boundary.
        let tri = if progress < 0.5 {
            progress * 2.0
        } else {
            2.0 - progress * 2.0
        };
        let eased = ease_in_out(tri);
        let alpha_mult =
            XP_SKELETON_PULSE_MIN + (XP_SKELETON_PULSE_MAX - XP_SKELETON_PULSE_MIN) * eased;
        let color = hsla(self.bg.h, self.bg.s, self.bg.l, self.bg.a * alpha_mult);

        window.paint_quad(gpui::PaintQuad {
            bounds,
            corner_radii: Corners::all(self.radius).clamp_radii_for_quad_size(bounds.size),
            background: color.into(),
            border_color: hsla(0., 0., 0., 0.),
            border_widths: Edges::default(),
            border_style: BorderStyle::default(),
        });
    }
}

impl SkeletonRenderer for XpSkeletonRenderer {
    fn compose(&self, props: &yororen_ui_core::headless::skeleton::SkeletonProps, cx: &App) -> Div {
        let theme = cx.theme();
        let state = SkeletonRenderState {
            block: props.block,
            block_sharp: props.block_sharp,
        };
        let bg = self.bg(&state, theme);
        let min_h = self.min_height(&state, theme);
        let radius = self.border_radius(&state, theme);
        let duration_ms = theme
            .get_number("motion.duration_skeleton_pulse")
            .unwrap_or(1100.0) as u64;

        let mut el = gpui::div().rounded(radius);
        if let Some(w) = props.w {
            el = el.w(w);
        }
        if let Some(h) = props.h {
            el = el.h(h);
        } else {
            el = el.min_h(min_h);
        }
        el.child(XpSkeletonPulseElement {
            bg,
            radius,
            duration_ms,
        })
    }
}

// =====================================================================
// ProgressBar
// =====================================================================

pub use yororen_ui_core::renderer::progress::{ProgressBarRenderState, ProgressBarRenderer};

pub struct XpProgressBarRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpProgressBarRenderer {
    pub fn track_bg(&self, _: &ProgressBarRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.progress.track_bg", progress_track_bg())
    }

    pub fn track_border(&self, _: &ProgressBarRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.progress.track_border", progress_track_border())
    }

    pub fn chunk_bg(&self, _: &ProgressBarRenderState, theme: &Theme) -> Background {
        progress_chunk_gradient(theme)
    }

    pub fn chunk_border(&self, _: &ProgressBarRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.progress.chunk_border", progress_chunk_border())
    }

    pub fn height(&self, _: &ProgressBarRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.progress.height")
            .unwrap_or(12.0) as f32)
    }

    pub fn border_radius(&self, _: &ProgressBarRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.progress.radius")
            .unwrap_or(2.0) as f32)
    }

    pub fn segment_width(&self, _: &ProgressBarRenderState, theme: &Theme) -> Pixels {
        px(xp_number(
            theme,
            "xp.progress.segment_width",
            XP_PROGRESS_SEGMENT_W as f64,
        ))
    }

    pub fn segment_gap(&self, _: &ProgressBarRenderState, theme: &Theme) -> Pixels {
        px(xp_number(
            theme,
            "xp.progress.segment_gap",
            XP_PROGRESS_SEGMENT_GAP as f64,
        ))
    }

    pub fn segment_radius(&self, _: &ProgressBarRenderState, theme: &Theme) -> Pixels {
        px(xp_number(theme, "xp.progress.segment_radius", 1.0))
    }
}

impl ProgressBarRenderer for XpProgressBarRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::progress::ProgressBarProps,
        cx: &App,
    ) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = ProgressBarRenderState {
            indeterminate: props.indeterminate,
            has_custom_height: props.has_custom_height,
        };
        let track_bg = self.track_bg(&state, theme);
        let track_border = self.track_border(&state, theme);
        let chunk_bg = self.chunk_bg(&state, theme);
        let chunk_border = self.chunk_border(&state, theme);
        let h = self.height(&state, theme);
        let r = self.border_radius(&state, theme);
        let seg_w = self.segment_width(&state, theme);
        let seg_gap = self.segment_gap(&state, theme);
        let seg_r = self.segment_radius(&state, theme);
        let ratio = if props.indeterminate || props.max <= 0.0 {
            0.0
        } else {
            (props.value / props.max).clamp(0.0, 1.0)
        };
        // The chunks are painted by a custom element so they keep
        // their fixed pixel width and cover exactly `ratio` of the
        // track — divs can't measure the track at compose time.
        gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(h)
            .bg(track_bg)
            .rounded(r)
            .border(px(XP_BORDER_WIDTH))
            .border_color(track_border)
            // 1px white inset between the sunken border and the
            // chunks, like the real Luna progress well.
            .p(px(1.))
            .child(XpProgressChunksElement {
                ratio,
                seg_w,
                seg_gap,
                seg_r,
                chunk_bg,
                chunk_border,
            })
    }
}

/// Internal `Element` painting the Luna-green chunks across
/// `ratio` of the track width. Same pattern as
/// `XpSliderTrackElement` in `controls.rs`.
struct XpProgressChunksElement {
    ratio: f32,
    seg_w: Pixels,
    seg_gap: Pixels,
    seg_r: Pixels,
    chunk_bg: Background,
    chunk_border: Hsla,
}

impl IntoElement for XpProgressChunksElement {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for XpProgressChunksElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = gpui::relative(1.0).into();
        style.size.height = gpui::relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        _cx: &mut App,
    ) {
        let track_w: f32 = bounds.size.width.into();
        let fill_w = self.ratio * track_w;
        let sw: f32 = self.seg_w.into();
        let gap: f32 = self.seg_gap.into();
        let mut x: f32 = 0.0;
        // Paint whole chunks until the next one would cross the
        // fill edge; a partial trailing chunk is clipped away,
        // which matches the Luna block-step look.
        while x + sw <= fill_w + f32::EPSILON {
            window.paint_quad(PaintQuad {
                bounds: Bounds::new(
                    point(bounds.left() + px(x), bounds.top()),
                    size(px(sw), bounds.size.height),
                ),
                corner_radii: Corners::all(self.seg_r),
                background: self.chunk_bg,
                border_color: self.chunk_border,
                border_widths: Edges::all(px(XP_BORDER_WIDTH)),
                border_style: BorderStyle::Solid,
            });
            x += sw + gap;
        }
    }
}

// =====================================================================
// EmptyState
// =====================================================================

pub use yororen_ui_core::renderer::empty_state::{EmptyStateRenderState, EmptyStateRenderer};

pub struct XpEmptyStateRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpEmptyStateRenderer {
    pub fn icon_color(&self, _: &EmptyStateRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.tertiary")
            .unwrap_or(hsl_fallback(0x56554A))
    }
    pub fn title_color(&self, _: &EmptyStateRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.primary")
            .unwrap_or(hsl_fallback(0x000000))
    }
    pub fn body_color(&self, _: &EmptyStateRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.secondary")
            .unwrap_or(hsl_fallback(0x2B2B23))
    }
    pub fn padding(&self, _: &EmptyStateRenderState, theme: &Theme) -> SpecEdges<Pixels> {
        let p = theme
            .get_number("tokens.control.empty_state.padding")
            .unwrap_or(24.0) as f32;
        SpecEdges::all(px(p))
    }
    pub fn icon_size(&self, _: &EmptyStateRenderState, _: &Theme) -> Pixels {
        px(48.0)
    }
    pub fn gap(&self, _: &EmptyStateRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.spacing.loose").unwrap_or(16.0) as f32)
    }
}

impl EmptyStateRenderer for XpEmptyStateRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::empty_state::EmptyStateProps,
        cx: &App,
    ) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = EmptyStateRenderState {};
        let ic = self.icon_color(&state, theme);
        let tc = self.title_color(&state, theme);
        let bc = self.body_color(&state, theme);
        let pad = self.padding(&state, theme);
        let is = self.icon_size(&state, theme);
        let g = self.gap(&state, theme);
        let mut el = gpui::div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p(pad.top)
            .gap(g);
        if let Some(icon) = &props.icon {
            // Resolve the icon source the same way `IconProps::render`
            // does: builtin names map to `icons/<name>.svg`; resource
            // paths pass through.
            let path: SharedString = match icon {
                IconSource::Builtin(name) => format!("icons/{name}.svg").into(),
                IconSource::Resource(p) => p.clone(),
            };
            el = el.child(
                gpui::svg()
                    .path(path)
                    .size(is)
                    .text_color(ic)
                    .into_any_element(),
            );
        }
        if let Some(title) = &props.title {
            el = el.child(gpui::div().text_color(tc).child(title.clone()));
        }
        if let Some(desc) = &props.description {
            el = el.child(gpui::div().text_color(bc).child(desc.clone()));
        }
        el
    }
}

// End of empty-state impl.

// =====================================================================
// KeybindingDisplay
// =====================================================================

pub use yororen_ui_core::renderer::keybinding_display::{
    KeybindingDisplayRenderState, KeybindingDisplayRenderer,
};

pub struct XpKeybindingDisplayRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpKeybindingDisplayRenderer {
    /// Each key cap is a miniature raised XP button.
    pub fn kbd_bg(&self, _state: &KeybindingDisplayRenderState, theme: &Theme) -> Background {
        button_face(theme)
    }
    pub fn kbd_fg(&self, _state: &KeybindingDisplayRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.primary")
            .unwrap_or(hsl_fallback(0x000000))
    }
    pub fn border(&self, _state: &KeybindingDisplayRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.button.border", button_border())
    }
    pub fn padding_x(&self, _state: &KeybindingDisplayRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.keybinding_input.kbd_padding_x")
            .unwrap_or(6.0) as f32)
    }
    pub fn padding_y(&self, _state: &KeybindingDisplayRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.keybinding_input.kbd_padding_y")
            .unwrap_or(2.0) as f32)
    }
    pub fn gap(&self, _state: &KeybindingDisplayRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.keybinding_input.separator_gap")
            .unwrap_or(4.0) as f32)
    }
    pub fn font_family(
        &self,
        _state: &KeybindingDisplayRenderState,
        theme: &Theme,
    ) -> SharedString {
        theme
            .get_string("tokens.typography.family_mono")
            .unwrap_or(XP_FONT_FAMILY)
            .to_string()
            .into()
    }
}

impl KeybindingDisplayRenderer for XpKeybindingDisplayRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::keybinding_display::KeybindingDisplayProps,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = KeybindingDisplayRenderState {};
        let kbd_bg = self.kbd_bg(&state, theme);
        let kbd_fg = self.kbd_fg(&state, theme);
        let bd = self.border(&state, theme);
        let px_h = self.padding_x(&state, theme);
        let px_v = self.padding_y(&state, theme);
        let g = self.gap(&state, theme);
        let family = self.font_family(&state, theme);
        let mut row = gpui::div()
            .id(props.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .gap(g);
        for key in &props.keys {
            row = row.child(
                gpui::div()
                    .bg(kbd_bg)
                    .text_color(kbd_fg)
                    .border(px(XP_BORDER_WIDTH))
                    .border_color(bd)
                    .rounded(px(XP_RADIUS))
                    .px(px_h)
                    .py(px_v)
                    .font_family(family.clone())
                    .text_size(px(12.))
                    .child(key.clone()),
            );
        }
        row
    }
}

// End of keybinding-display impl.

// =====================================================================
// ShortcutHint
// =====================================================================

pub use yororen_ui_core::renderer::shortcut_hint::{ShortcutHintRenderState, ShortcutHintRenderer};

pub struct XpShortcutHintRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpShortcutHintRenderer {
    pub fn label_fg(&self, _state: &ShortcutHintRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.secondary")
            .unwrap_or(hsl_fallback(0x2B2B23))
    }
    pub fn kbd_bg(&self, _state: &ShortcutHintRenderState, theme: &Theme) -> Background {
        button_face(theme)
    }
    pub fn kbd_fg(&self, _state: &ShortcutHintRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.primary")
            .unwrap_or(hsl_fallback(0x000000))
    }
    pub fn border(&self, _state: &ShortcutHintRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.button.border", button_border())
    }
    pub fn padding_x(&self, _state: &ShortcutHintRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.keybinding_input.kbd_padding_x")
            .unwrap_or(6.0) as f32)
    }
    pub fn padding_y(&self, _state: &ShortcutHintRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.keybinding_input.kbd_padding_y")
            .unwrap_or(2.0) as f32)
    }
    pub fn key_gap(&self, _state: &ShortcutHintRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.keybinding_input.separator_gap")
            .unwrap_or(4.0) as f32)
    }
    pub fn label_gap(&self, _state: &ShortcutHintRenderState, theme: &Theme) -> Pixels {
        px(theme.get_number("tokens.spacing.inset_sm").unwrap_or(8.0) as f32)
    }
    pub fn font_family(&self, _state: &ShortcutHintRenderState, theme: &Theme) -> SharedString {
        theme
            .get_string("tokens.typography.family_mono")
            .unwrap_or(XP_FONT_FAMILY)
            .to_string()
            .into()
    }
}

impl ShortcutHintRenderer for XpShortcutHintRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::shortcut_hint::ShortcutHintProps,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = ShortcutHintRenderState {};
        let label_fg = self.label_fg(&state, theme);
        let kbd_bg = self.kbd_bg(&state, theme);
        let kbd_fg = self.kbd_fg(&state, theme);
        let bd = self.border(&state, theme);
        let px_h = self.padding_x(&state, theme);
        let px_v = self.padding_y(&state, theme);
        let key_g = self.key_gap(&state, theme);
        let label_g = self.label_gap(&state, theme);
        let family = self.font_family(&state, theme);

        let mut keys_row = gpui::div().flex().flex_row().items_center().gap(key_g);
        for key in &props.keys {
            keys_row = keys_row.child(
                gpui::div()
                    .bg(kbd_bg)
                    .text_color(kbd_fg)
                    .border(px(XP_BORDER_WIDTH))
                    .border_color(bd)
                    .rounded(px(XP_RADIUS))
                    .px(px_h)
                    .py(px_v)
                    .font_family(family.clone())
                    .text_size(px(12.))
                    .child(key.clone()),
            );
        }

        gpui::div()
            .id(props.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .gap(label_g)
            .child(
                gpui::div()
                    .text_color(label_fg)
                    .font_family(family.clone())
                    .text_size(px(12.))
                    .child(props.label.clone()),
            )
            .child(keys_row)
    }
}

// =====================================================================
// Icon
// =====================================================================

pub use yororen_ui_core::renderer::icon::{IconRenderState, IconRenderer};

pub struct XpIconRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpIconRenderer {
    pub fn size(&self, state: &IconRenderState, theme: &Theme) -> Pixels {
        if state.has_custom_size {
            return px(0.0);
        }
        // XP toolbars and dialogs standardize on 16px icons, which
        // is also the system theme's `icon_md`.
        px(theme
            .get_number("tokens.control.icon.default_size")
            .or_else(|| theme.get_number("tokens.sizes.icon_md"))
            .unwrap_or(16.0) as f32)
    }

    pub fn color(&self, state: &IconRenderState, theme: &Theme) -> Hsla {
        if state.has_custom_color {
            return hsl_fallback(0x000000);
        }
        theme
            .get_color("content.primary")
            .unwrap_or(hsl_fallback(0x000000))
    }
}

impl IconRenderer for XpIconRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::icon::IconProps,
        cx: &App,
    ) -> gpui::AnyElement {
        let theme = cx.theme();
        let state = IconRenderState {
            has_custom_color: props.color.is_some(),
            has_custom_size: props.size.is_some(),
        };
        let path: SharedString = match &props.source {
            IconSource::Builtin(name) => format!("icons/{name}.svg").into(),
            IconSource::Resource(path) => path.clone(),
        };
        let size = props.size.unwrap_or_else(|| self.size(&state, theme));
        let color = props.color.unwrap_or_else(|| self.color(&state, theme));
        gpui::svg()
            .path(path)
            .size(size)
            .id(props.id.clone())
            .text_color(color)
            .into_any_element()
    }
}

// =====================================================================
// Text
// =====================================================================

pub use yororen_ui_core::renderer::text::{TextRenderState, TextRenderer};

pub struct XpTextRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpTextRenderer {
    pub fn size(&self, state: &TextRenderState, theme: &Theme) -> Pixels {
        if state.has_custom_size {
            return px(0.0);
        }
        px(theme
            .get_number("tokens.control.text.default_size")
            .or_else(|| theme.get_number("tokens.typography.font_size_md"))
            .unwrap_or(13.0) as f32)
    }

    pub fn color(&self, state: &TextRenderState, theme: &Theme) -> Hsla {
        if state.has_custom_color {
            return hsl_fallback(0x000000);
        }
        theme
            .get_color("content.primary")
            .unwrap_or(hsl_fallback(0x000000))
    }

    pub fn family(&self, _state: &TextRenderState, theme: &Theme) -> SharedString {
        // XP body text is Tahoma. Fall back to the XP font stack
        // if the theme omits the typography family path.
        theme
            .get_string("tokens.typography.family_default")
            .unwrap_or(XP_FONT_FAMILY)
            .to_string()
            .into()
    }
}

impl TextRenderer for XpTextRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::text::TextProps,
        cx: &App,
    ) -> Stateful<Div> {
        let theme = cx.theme();
        let state = TextRenderState {
            has_custom_size: props.size.is_some(),
            has_custom_color: false,
        };
        gpui::div()
            .id(props.id.clone())
            .text_size(props.size.unwrap_or_else(|| self.size(&state, theme)))
            .text_color(self.color(&state, theme))
            .font_family(self.family(&state, theme))
            .child(props.text.clone())
    }
}

// =====================================================================
// Spacer
// =====================================================================

pub use yororen_ui_core::renderer::spacer::{SpacerRenderState, SpacerRenderer};

pub struct XpSpacerRenderer;

impl SpacerRenderer for XpSpacerRenderer {
    fn compose(
        &self,
        props: &yororen_ui_core::headless::spacer::SpacerProps,
        _cx: &App,
    ) -> Stateful<Div> {
        // A spacer is invisible by definition — XP has nothing
        // extra to add, so we mirror the default renderer's
        // `flex_1()` behaviour. The caller can layer explicit
        // width / height on top via the returned `Stateful<Div>`.
        gpui::div().id(props.id.clone()).flex_1()
    }
}

// End of shortcut-hint impl.
