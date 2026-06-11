//! `TokenTreeItemRenderer` — default `TreeItemRenderer` impl.

use std::sync::Arc;

use gpui::{App, Div, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement, Pixels, SharedString, StatefulInteractiveElement, Styled, Stateful, div, prelude::FluentBuilder};

use yororen_ui_core::headless::tree_item::TreeItemProps;
use yororen_ui_core::renderer::spec::Edges;
use yororen_ui_core::theme::Theme;

pub use yororen_ui_core::renderer::tree_item::{TreeItemRenderState, TreeItemRenderer};

pub struct TokenTreeItemRenderer;

// Inherent helpers — *not* part of the trait surface.
impl TokenTreeItemRenderer {
    pub fn bg(&self, _state: &TreeItemRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.base").unwrap_or_default()
    }
    pub fn hover_bg(&self, _state: &TreeItemRenderState, theme: &Theme) -> Hsla {
        theme.get_color("surface.hover").unwrap_or_default()
    }
    pub fn selected_bg(&self, _state: &TreeItemRenderState, theme: &Theme) -> Hsla {
        theme.get_color("action.primary.bg").unwrap_or_default()
    }
    pub fn fg(&self, state: &TreeItemRenderState, theme: &Theme) -> Hsla {
        if state.selected {
            theme.get_color("action.primary.fg").unwrap_or_default()
        } else {
            theme.get_color("content.primary").unwrap_or_default()
        }
    }
    pub fn indent(&self, state: &TreeItemRenderState, theme: &Theme) -> Pixels {
        let step = theme
            .get_number("tokens.control.tree_item.indent")
            .unwrap_or_else(|| theme.get_number("tokens.spacing.inset_md").unwrap_or(0.0))
            as f32;
        let step = step.max(12.0);
        gpui::px(state.depth as f32 * step)
    }
    pub fn padding(&self, _state: &TreeItemRenderState, theme: &Theme) -> Edges<Pixels> {
        Edges::symmetric(
            gpui::px(
                theme
                    .get_number("tokens.control.tree_item.horizontal_padding")
                    .unwrap_or_else(|| theme.get_number("tokens.spacing.inset_sm").unwrap_or(8.0))
                    as f32,
            ),
            gpui::px(theme.get_number("tokens.spacing.inset_xs").unwrap_or(4.0) as f32),
        )
    }
    pub fn min_height(&self, _state: &TreeItemRenderState, theme: &Theme) -> Pixels {
        gpui::px(
            theme
                .get_number("tokens.control.tree_item.min_height")
                .unwrap_or(0.0) as f32,
        )
    }
    pub fn chevron_size(&self, _state: &TreeItemRenderState, theme: &Theme) -> Pixels {
        gpui::px(
            theme
                .get_number("tokens.control.tree_item.chevron_size")
                .unwrap_or_else(|| {
                    theme
                        .get_number("tokens.control.list_item.chevron_size")
                        .unwrap_or(14.0)
                }) as f32,
        )
    }
    pub fn chevron_gap(&self, _state: &TreeItemRenderState, theme: &Theme) -> Pixels {
        gpui::px(theme.get_number("tokens.spacing.gap_1").unwrap_or(4.0) as f32)
    }
}

impl TreeItemRenderer for TokenTreeItemRenderer {
    fn compose(&self, props: &TreeItemProps, cx: &App) -> Div {
        use yororen_ui_core::theme::ActiveTheme;
        let theme = cx.theme();
        let state = TreeItemRenderState {
            selected: props.selected,
            expanded: props.expanded,
            depth: props.depth.min(u8::MAX as usize) as u8,
            is_leaf: !props.has_children,
        };
        let bg = if state.selected {
            self.selected_bg(&state, theme)
        } else {
            self.bg(&state, theme)
        };
        let fg = self.fg(&state, theme);
        let pad = self.padding(&state, theme);
        let h = self.min_height(&state, theme);
        let indent = self.indent(&state, theme);
        let chevron_size = self.chevron_size(&state, theme);
        let gap = self.chevron_gap(&state, theme);

        // Chevron slot — always reserves space so labels at the
        // same depth align whether or not a row has children.
        // When `has_children` is true the slot is a clickable
        // stateful div that fires `props.on_toggle` and uses
        // `.occlude()` so the click doesn't pass through to the
        // row body underneath. Leaves get an empty placeholder of
        // the same width.
        let chevron_slot = if props.has_children {
            let glyph: SharedString = if props.expanded { "▼".into() } else { "▶".into() };
            let chevron_id: ElementId =
                format!("{:?}-chevron", props.id).into();
            let toggle_cb = props.on_toggle.clone();
            let disabled = props.disabled;
            div()
                .id(chevron_id)
                .w(chevron_size)
                .h(chevron_size)
                .flex()
                .items_center()
                .justify_center()
                .text_color(fg)
                .when(!disabled, |s: Stateful<Div>| s.cursor_pointer())
                .occlude()
                .child(glyph)
                .on_click(move |ev, window, cx| {
                    if disabled {
                        return;
                    }
                    if let Some(cb) = toggle_cb.as_ref() {
                        cb(ev, window, cx);
                    }
                })
                .into_any_element()
        } else {
            div().w(chevron_size).h(chevron_size).into_any_element()
        };

        div()
            .flex()
            .items_center()
            .gap(gap)
            .bg(bg)
            .text_color(fg)
            .pl(indent + pad.left)
            .pr(pad.right)
            .py(pad.top)
            .min_h(h)
            .child(chevron_slot)
            .child(props.label.clone())
    }
}

pub fn arc_tree_item<T: TreeItemRenderer + 'static>(r: T) -> Arc<dyn TreeItemRenderer> {
    Arc::new(r)
}
