//! `SwitchRenderer` — the visual side of `Switch`.

use std::any::Any;
use std::sync::Arc;

use gpui::{Hsla, Pixels};

use yororen_ui_core::theme::Theme;

#[derive(Clone, Copy, Debug, Default)]
pub struct SwitchRenderState {
    pub checked: bool,
    pub disabled: bool,
    pub has_custom_tone: bool,
}

pub trait SwitchRenderer: Any + Send + Sync {
    fn track_w(&self, state: &SwitchRenderState, theme: &Theme) -> Pixels;
    fn track_h(&self, state: &SwitchRenderState, theme: &Theme) -> Pixels;
    fn knob_size(&self, state: &SwitchRenderState, theme: &Theme) -> Pixels;
    fn padding(&self, state: &SwitchRenderState, theme: &Theme) -> Pixels;
    fn track_bg(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla;
    fn track_border(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla;
    fn track_hover_bg(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla;
    fn knob_bg(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla;
    fn focus_color(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla;
    fn disabled_opacity(&self, state: &SwitchRenderState, theme: &Theme) -> f32;
}

pub struct TokenSwitchRenderer;

impl SwitchRenderer for TokenSwitchRenderer {
    fn track_w(&self, _state: &SwitchRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.switch.track_w").unwrap_or(0.0) as f32)
    }
    fn track_h(&self, _state: &SwitchRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.switch.track_h").unwrap_or(0.0) as f32)
    }
    fn knob_size(&self, _state: &SwitchRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.switch.knob_size").unwrap_or(0.0) as f32)
    }
    fn padding(&self, _state: &SwitchRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.control.switch.padding").unwrap_or(0.0) as f32)
    }

    fn track_bg(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("surface.sunken").unwrap_or_default()
        } else if state.checked {
            theme.get_color("action.primary.bg").unwrap_or_default()
        } else {
            theme.get_color("surface.hover").unwrap_or_default()
        }
    }
    fn track_border(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla {
        if state.checked {
            theme.get_color("border.muted").unwrap_or_default()
        } else {
            theme.get_color("border.default").unwrap_or_default()
        }
    }
    fn track_hover_bg(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla {
        if state.checked {
            theme.get_color("action.primary.hover_bg").unwrap_or_default()
        } else {
            theme.get_color("surface.base").unwrap_or_default()
        }
    }
    fn knob_bg(&self, state: &SwitchRenderState, theme: &Theme) -> Hsla {
        if state.disabled {
            theme.get_color("content.disabled").unwrap_or_default()
        } else if state.checked {
            theme.get_color("action.primary.fg").unwrap_or_default()
        } else {
            theme.get_color("content.primary").unwrap_or_default()
        }
    }
    fn focus_color(&self, _state: &SwitchRenderState, theme: &Theme) -> Hsla {
        theme.get_color("border.focus").unwrap_or_default()
    }
    fn disabled_opacity(&self, _state: &SwitchRenderState, _theme: &Theme) -> f32 {
        0.5
    }
}

pub fn arc_switch<T: SwitchRenderer + 'static>(r: T) -> Arc<dyn SwitchRenderer> {
    Arc::new(r)
}

// =====================================================================
// `DefaultSwitch` — `headless::SwitchProps` sugar.
// =====================================================================

use gpui::{prelude::FluentBuilder, div, App, ParentElement, Stateful, Styled};
use yororen_ui_core::headless::switch::SwitchProps;
use yororen_ui_core::renderer::{markers, RendererContext};
use yororen_ui_core::theme::ActiveTheme;

pub trait DefaultSwitch: Sized {
    fn default_render(self, cx: &App) -> Stateful<gpui::Div>;
}

impl DefaultSwitch for SwitchProps {
    fn default_render(self, cx: &App) -> Stateful<gpui::Div> {
        let theme = cx.theme();
        let r: &Arc<dyn SwitchRenderer> = cx
            .renderer_arc::<markers::Switch, dyn SwitchRenderer>()
            .expect("SwitchRenderer registered");
        let state = SwitchRenderState {
            checked: self.checked,
            disabled: self.disabled,
            has_custom_tone: false,
        };
        let track = r.track_bg(&state, theme);
        let knob = r.knob_bg(&state, theme);
        let w = r.track_w(&state, theme);
        let h = r.track_h(&state, theme);
        let knob_size = r.knob_size(&state, theme);
        let pad = r.padding(&state, theme);
        let pill_radius = gpui::px(theme.get_number("tokens.radii.pill").unwrap_or(0.0) as f32);
        let mut el = div()
            .bg(track)
            .w(w)
            .h(h)
            .rounded(pill_radius)
            .p(pad)
            .flex()
            .items_center();
        if self.checked {
            el = el.justify_end();
        } else {
            el = el.justify_start();
        }
        el = el.child(div().bg(knob).size(knob_size).rounded(pill_radius));
        self.apply(el)
    }
}
