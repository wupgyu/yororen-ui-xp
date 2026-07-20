//! XP (Luna) surface renderers: `Tooltip`, `Avatar`, `Panel`,
//! `Card`, `Image`.
//!
//! Tooltips are the classic pale-yellow Win32 balloon (square
//! corners, thin black edge); panels and cards are flat surfaces
//! with a 1px bevel border; avatars are nearly square with a thin
//! bevel frame. Nothing in this group casts a shadow — XP only
//! shadows overlays.

use gpui::{
    App, AppContext, Context, CursorStyle, Div, FontWeight, Hsla, InteractiveElement, IntoElement,
    ParentElement, Pixels, Render, StatefulInteractiveElement, Styled, Window, div, px,
};
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::{ActiveTheme, Theme};

use crate::style::{
    self, XP_BORDER_WIDTH, XP_RADIUS, bevel_inner_dark, bevel_outer_light, dialog_bg, hgrad,
    hsl_fallback, tooltip_bg, xp_color,
};

// =====================================================================
// Tooltip
// =====================================================================

pub use yororen_ui_core::renderer::tooltip::{TooltipRenderState, TooltipRenderer};

pub struct XpTooltipRenderer;

/// View rendered by gpui's `hoverable_tooltip` builder for the
/// XP tooltip panel: pale yellow, square corners, 1px black
/// edge — the classic Win32 balloon tip.
struct XpTooltipView {
    text: String,
    bg: Hsla,
    fg: Hsla,
    pad_top: Pixels,
    font_size: Pixels,
    border_radius: Pixels,
    border_w: Pixels,
    border_color: Hsla,
    max_width: Pixels,
}

impl Render for XpTooltipView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
            .bg(self.bg)
            .text_color(self.fg)
            .p(self.pad_top)
            .text_size(self.font_size)
            .rounded(self.border_radius)
            .border(self.border_w)
            .border_color(self.border_color)
            .max_w(self.max_width)
            .child(self.text.clone())
    }
}

// Inherent helpers — *not* part of the trait surface.
impl XpTooltipRenderer {
    pub fn bg(&self, _: &TooltipRenderState, theme: &Theme) -> Hsla {
        // XP tooltips are pale yellow (`#FFFFE1`), not action blue.
        xp_color(theme, "xp.tooltip.bg", tooltip_bg())
    }
    pub fn fg(&self, _: &TooltipRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.primary")
            .unwrap_or(style::hsl_fallback(0x000000))
    }
    pub fn padding(&self, _: &TooltipRenderState, theme: &Theme) -> Edges<Pixels> {
        let h = theme
            .get_number("tokens.control.tooltip.padding_x")
            .unwrap_or(8.0) as f32;
        let v = theme
            .get_number("tokens.control.tooltip.padding_y")
            .unwrap_or(4.0) as f32;
        Edges::symmetric(px(h), px(v))
    }
    pub fn font_size(&self, _: &TooltipRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.tooltip.font_size")
            .unwrap_or(11.0) as f32)
    }
    pub fn border_radius(&self, _: &TooltipRenderState, theme: &Theme) -> Pixels {
        // Win32 balloon tips have square corners.
        px(theme
            .get_number("tokens.control.tooltip.radius")
            .unwrap_or(0.0) as f32)
    }
    pub fn border_w(&self, _: &TooltipRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.tooltip.border_width")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32)
    }
    pub fn border_color(&self, _: &TooltipRenderState, theme: &Theme) -> Hsla {
        // The classic XP tooltip edge is solid black.
        xp_color(theme, "xp.tooltip.border", style::hsl_fallback(0x000000))
    }
}

impl TooltipRenderer for XpTooltipRenderer {
    fn compose(
        &self,
        props: &mut yororen_ui_core::headless::tooltip::TooltipProps,
        cx: &App,
    ) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = TooltipRenderState {
            has_custom_bg: props.has_custom_bg,
            has_custom_fg: props.has_custom_fg,
        };
        let bg = self.bg(&state, theme);
        let fg = self.fg(&state, theme);
        let pad = self.padding(&state, theme);
        let fs = self.font_size(&state, theme);
        let r = self.border_radius(&state, theme);
        let bw = self.border_w(&state, theme);
        let bc = self.border_color(&state, theme);
        let max_w = px(theme
            .get_number("tokens.control.tooltip.max_width")
            .unwrap_or(240.0) as f32);

        // The trigger is wrapped in a `Stateful<Div>` so we can attach
        // gpui's `hoverable_tooltip`. The floating panel is created by
        // gpui on hover and styled with XP tokens.
        let mut outer = gpui::div().flex().flex_col().items_start();

        if let Some(t) = props.trigger.take() {
            let text = props.text.clone();
            let trigger_id = format!("{}-trigger", props.id);
            outer = outer.child(gpui::div().id(trigger_id).child(t).hoverable_tooltip(
                move |_window, cx| {
                    cx.new(|_cx| XpTooltipView {
                        text: text.clone(),
                        bg,
                        fg,
                        pad_top: pad.top,
                        font_size: fs,
                        border_radius: r,
                        border_w: bw,
                        border_color: bc,
                        max_width: max_w,
                    })
                    .into()
                },
            ));
        }

        outer
    }
}

// =====================================================================
// Avatar
// =====================================================================

pub use yororen_ui_core::renderer::avatar::{AvatarRenderState, AvatarRenderer};

pub struct XpAvatarRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpAvatarRenderer {
    pub fn default_bg(&self, _: &AvatarRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("surface.hover")
            .unwrap_or(style::hsl_fallback(0xC1D2EE))
    }

    pub fn border_radius(&self, _: &AvatarRenderState, theme: &Theme) -> Pixels {
        // XP avatars are nearly square (rounded 2), never pills.
        px(theme
            .get_number("tokens.control.avatar.radius")
            .unwrap_or(2.0) as f32)
    }

    pub fn border_w(&self, _: &AvatarRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.avatar.border_width")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32)
    }

    pub fn border_color(&self, _: &AvatarRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
    }

    pub fn status_dot_size(&self, _: &AvatarRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.avatar.status_dot_size")
            .unwrap_or(12.0) as f32)
    }

    pub fn status_inset(&self, _: &AvatarRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.avatar.status_inset")
            .unwrap_or(2.0) as f32)
    }

    pub fn status_border_w(&self, _: &AvatarRenderState, theme: &Theme) -> Pixels {
        px(theme
            .get_number("tokens.control.avatar.border_w")
            .unwrap_or(XP_BORDER_WIDTH as f64) as f32)
    }

    pub fn status_border_color(&self, _: &AvatarRenderState, theme: &Theme) -> Hsla {
        // White ring so the status dot reads against the frame.
        xp_color(theme, "xp.bevel.outer_light", bevel_outer_light())
    }

    /// Initials / label colour. Reads `content.primary` from
    /// the theme so the text contrasts with the avatar
    /// background. Without this helper the `div().child(text)`
    /// would inherit gpui's default (`#000000`).
    pub fn label_color(&self, _: &AvatarRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.primary")
            .unwrap_or(style::hsl_fallback(0x000000))
    }
}

impl AvatarRenderer for XpAvatarRenderer {
    fn compose(&self, props: &yororen_ui_core::headless::avatar::AvatarProps, cx: &App) -> Div {
        let theme = cx.theme();
        let state = AvatarRenderState {
            has_custom_bg: props.has_custom_bg,
            has_status: props.has_status,
            is_circle: props.circle,
        };
        let bg = self.default_bg(&state, theme);
        let r = self.border_radius(&state, theme);
        let bw = self.border_w(&state, theme);
        let bc = self.border_color(&state, theme);
        let label_color = self.label_color(&state, theme);
        let size = props.size.unwrap_or(px(36.0));
        // Initials font sized at ~40% of avatar height so 2-letter
        // initials always fit inside the box.
        let font_size = size * 0.4;
        let label_text: Option<String> = if let Some(initials) = &props.initials {
            Some(initials.clone())
        } else {
            props.name.as_ref().map(|n| initials_from_name(n.as_ref()))
        };
        let content = if let Some(text) = label_text {
            div()
                .text_size(font_size)
                .text_color(label_color)
                .child(text)
        } else {
            div()
        };
        let mut el = div()
            .flex()
            .items_center()
            .justify_center()
            .bg(bg)
            .rounded(r)
            .border(bw)
            .border_color(bc)
            .size(size)
            .child(content);
        if props.has_status {
            let dot = self.status_dot_size(&state, theme);
            let inset = self.status_inset(&state, theme);
            let bw = self.status_border_w(&state, theme);
            let bc = self.status_border_color(&state, theme);
            el = el.child(
                div()
                    .absolute()
                    .right(inset)
                    .bottom(inset)
                    .size(dot)
                    .rounded(dot / 2.)
                    .border(bw)
                    .border_color(bc)
                    .bg(theme
                        .get_color("status.success.bg")
                        .unwrap_or(style::hsl_fallback(0x8CC63F))),
            );
        }
        el
    }
}

/// Extract up to 2 uppercase initials from a person's name. For
/// Latin / Cyrillic / Greek alphabets, takes the first letter of
/// the first and last whitespace-separated tokens (`"Jane Doe"` →
/// `"JD"`). For CJK names returns only the first character
/// (`"张三"` → `"张"`).
fn initials_from_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if let Some(first) = trimmed.chars().next()
        && is_cjk_char(first)
    {
        return first.to_string();
    }
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    if let Some(first) = parts.first().and_then(|w| w.chars().next()) {
        for c in first.to_uppercase() {
            out.push(c);
        }
    }
    if parts.len() > 1
        && let Some(last) = parts.last().and_then(|w| w.chars().next())
    {
        for c in last.to_uppercase() {
            out.push(c);
        }
    }
    out
}

fn is_cjk_char(c: char) -> bool {
    matches!(
        c as u32,
        0x3040..=0x309F
        | 0x30A0..=0x30FF
        | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF
        | 0xAC00..=0xD7AF
        | 0xF900..=0xFAFF
    )
}

// =====================================================================
// Panel
// =====================================================================

pub use yororen_ui_core::renderer::panel::{PanelRenderState, PanelRenderer};

pub struct XpPanelRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpPanelRenderer {
    pub fn bg(&self, _: &PanelRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("surface.raised")
            .unwrap_or(style::hsl_fallback(0xFFFFFF))
    }

    pub fn border(&self, _: &PanelRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
    }

    pub fn padding(&self, _: &PanelRenderState, theme: &Theme) -> Edges<Pixels> {
        let p = theme
            .get_number("tokens.control.panel.padding")
            .unwrap_or(12.0) as f32;
        Edges::all(px(p))
    }

    pub fn border_radius(&self, _: &PanelRenderState, _: &Theme) -> Pixels {
        px(XP_RADIUS)
    }

    pub fn shadow_alpha(&self, _: &PanelRenderState, _: &Theme) -> f32 {
        1.0
    }

    pub fn title_color(&self, _: &PanelRenderState, theme: &Theme) -> Hsla {
        theme
            .get_color("content.primary")
            .unwrap_or(style::hsl_fallback(0x000000))
    }
}

impl PanelRenderer for XpPanelRenderer {
    fn compose(&self, props: &yororen_ui_core::headless::panel::PanelProps, cx: &App) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = PanelRenderState {
            has_custom_bg: false,
            has_custom_border: false,
            has_custom_padding: false,
        };
        let bg = self.bg(&state, theme);
        let border = self.border(&state, theme);
        let pad = self.padding(&state, theme);
        let r = self.border_radius(&state, theme);
        let title_fg = self.title_color(&state, theme);
        let mut el = div()
            .flex()
            .flex_col()
            .bg(bg)
            .border(px(XP_BORDER_WIDTH))
            .border_color(border)
            .p(pad.top)
            .rounded(r);
        if let Some(title) = &props.title {
            el = el.child(div().text_color(title_fg).pb(px(6.)).child(title.clone()));
        }
        el
    }
}

// =====================================================================
// Card
// =====================================================================

pub use yororen_ui_core::renderer::card::{CardRenderState, CardRenderer};

pub struct XpCardRenderer;

// Inherent helpers — *not* part of the trait surface.
impl XpCardRenderer {
    pub fn bg(&self, _: &CardRenderState, theme: &Theme) -> Hsla {
        // Cards sit on the dialog beige (`#ECE9D8`) base surface.
        theme.get_color("surface.base").unwrap_or(dialog_bg())
    }

    pub fn border(&self, _: &CardRenderState, theme: &Theme) -> Hsla {
        xp_color(theme, "xp.bevel.inner_dark", bevel_inner_dark())
    }

    pub fn padding(&self, _: &CardRenderState, theme: &Theme) -> Edges<Pixels> {
        let p = theme
            .get_number("tokens.control.card.padding")
            .unwrap_or(12.0) as f32;
        Edges::all(px(p))
    }

    pub fn border_radius(&self, _: &CardRenderState, _: &Theme) -> Pixels {
        px(XP_RADIUS)
    }

    pub fn shadow_alpha(&self, _: &CardRenderState, _: &Theme) -> f32 {
        1.0
    }
}

impl CardRenderer for XpCardRenderer {
    fn compose(&self, props: &mut yororen_ui_core::headless::card::CardProps, cx: &App) -> Div {
        use yororen_ui_core::headless::card::CardAppearance;
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = CardRenderState {
            has_custom_bg: props.has_custom_bg,
        };
        let title = props.title.clone();
        let trailing = props.header_trailing.take();
        let interactive = props.interactive;

        match props.appearance {
            CardAppearance::ExplorerTask => {
                let header_from = xp_color(
                    theme,
                    "xp.explorer.task_card_header_from",
                    hsl_fallback(0xF0F0FF),
                );
                let header_to = xp_color(
                    theme,
                    "xp.explorer.task_card_header_to",
                    hsl_fallback(0xA8BCFF),
                );
                let body_from = xp_color(
                    theme,
                    "xp.explorer.task_card_body_from",
                    hsl_fallback(0xB4C8FB),
                );
                let body_mid = xp_color(
                    theme,
                    "xp.explorer.task_card_body_mid",
                    hsl_fallback(0xA4B9FB),
                );
                let body_to = xp_color(
                    theme,
                    "xp.explorer.task_card_body_to",
                    hsl_fallback(0xB4C8FB),
                );
                let title_fg =
                    xp_color(theme, "xp.explorer.task_card_title", hsl_fallback(0x0C327D));

                let mut header = gpui::div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .h(px(23.))
                    .pl(px(11.))
                    .pr(px(2.))
                    .bg(hgrad(header_from, header_to));
                if let Some(t) = title {
                    header = header.child(
                        gpui::div()
                            .flex_1()
                            .font_weight(FontWeight::BOLD)
                            .text_color(title_fg)
                            .text_size(px(11.))
                            .child(t),
                    );
                }
                if let Some(tr) = trailing {
                    header = header.child(tr);
                }

                // Body rows are caller-supplied children chained after
                // `.render(cx)`. Outer paint uses the body gradient so
                // content sits on the classic Explorer task-card fill.
                let _ = body_mid;
                gpui::div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .overflow_hidden()
                    .rounded_tl(px(3.))
                    .rounded_tr(px(3.))
                    .bg(hgrad(body_from, body_to))
                    .child(header)
                    .cursor(if interactive {
                        CursorStyle::PointingHand
                    } else {
                        CursorStyle::Arrow
                    })
            }
            CardAppearance::Default => {
                let bg = self.bg(&state, theme);
                let border = self.border(&state, theme);
                let pad = self.padding(&state, theme);
                let r = self.border_radius(&state, theme);
                let mut el = div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .bg(bg)
                    .border(px(XP_BORDER_WIDTH))
                    .border_color(border)
                    .p(pad.top)
                    .rounded(r)
                    .cursor(if interactive {
                        CursorStyle::PointingHand
                    } else {
                        CursorStyle::Arrow
                    });
                if title.is_some() || trailing.is_some() {
                    let mut header = div().flex().flex_row().items_center().gap(px(6.));
                    if let Some(t) = title {
                        header = header.child(
                            div()
                                .flex_1()
                                .text_color(
                                    theme
                                        .get_color("content.primary")
                                        .unwrap_or(hsl_fallback(0x000000)),
                                )
                                .child(t),
                        );
                    }
                    if let Some(tr) = trailing {
                        header = header.child(tr);
                    }
                    el = el.child(header);
                }
                el
            }
        }
    }
}

// End of card impl.

// =====================================================================
// Image
// =====================================================================

pub use yororen_ui_core::renderer::image::{ImageRenderState, ImageRenderer};

use gpui::Stateful;
use std::sync::Arc;
use yororen_ui_core::headless::image::{ImageProps, ImageSource};

pub struct XpImageRenderer;

// Inherent helpers — *not* part of the trait surface.
impl ImageRenderer for XpImageRenderer {
    fn compose(&self, props: &ImageProps, _cx: &App) -> Stateful<Div> {
        // Icons and bitmaps in XP Explorer are painted without a
        // bevel frame. Callers that need a framed image should wrap
        // the element themselves.
        let img = match &props.source {
            ImageSource::Resource(path) => gpui::img(path.to_string()),
            ImageSource::Handle(handle) => {
                gpui::img(gpui::ImageSource::Image(Arc::new(handle.clone())))
            }
        };
        gpui::div()
            .id(props.id.clone())
            .overflow_hidden()
            .child(img.size_full())
    }
}

// End of image impl.
