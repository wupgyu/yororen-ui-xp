//! Windows XP (Luna) style renderer for yororen-ui.
//!
//! Implements all 55 `XxxRenderer` traits with the classic XP look:
//! white → cream gradient buttons with 3px rounded corners, Luna
//! blue title bars, beveled wells, green segmented progress chunks,
//! pale-yellow tooltips, and Tahoma typography.
//!
//! One bundled theme: `xp-luna.json`. XP has no dark mode, so
//! [`install`] always uses the Luna theme regardless of system
//! appearance.
//!
//! ```ignore
//! use yororen_ui_xp_renderer as xp;
//! xp::install(cx);
//! // Full-window app chrome (Luna title bar + three-side blue frame):
//! let options = xp::XpAppWindow::window_options(cx, "My App", size(px(800.), px(600.)));
//! ```

// The crate is built up across several commits; until every
// `XxxRenderer` is wired in, individual helpers and constants
// will appear unused.
#![allow(dead_code)]

mod style;

pub mod renderers;
pub mod window;

pub use window::XpAppWindow;

use std::sync::Arc;

use gpui::App;
use yororen_ui_core::renderer::{RendererContext, markers as m};
use yororen_ui_core::theme::{Theme, install as install_theme};

use yororen_ui_core::renderer::button_group::ButtonGroupRenderer;
use yororen_ui_core::renderer::form_field::FormFieldRenderer;
use yororen_ui_core::renderer::icon::IconRenderer as CoreIconRenderer;
use yororen_ui_core::renderer::image::ImageRenderer;
use yororen_ui_core::renderer::keybinding_display::KeybindingDisplayRenderer;
use yororen_ui_core::renderer::listbox::ListboxRenderer;
use yororen_ui_core::renderer::menu::MenuRenderer;
use yororen_ui_core::renderer::overlay::OverlayRenderer;
use yororen_ui_core::renderer::radio_group::RadioGroupRenderer;
use yororen_ui_core::renderer::shortcut_hint::ShortcutHintRenderer;
use yororen_ui_core::renderer::slider::SliderRenderer;
use yororen_ui_core::renderer::spacer::SpacerRenderer;
use yororen_ui_core::renderer::table::TableRenderer;
use yororen_ui_core::renderer::text::TextRenderer;
use yororen_ui_core::renderer::tree::TreeRenderer;
use yororen_ui_default_renderer::renderers::*;

use crate::renderers::{
    actions::{
        XpButtonGroupRenderer, XpButtonRenderer, XpIconButtonRenderer, XpSplitButtonRenderer,
        XpToggleButtonRenderer,
    },
    controls::{
        XpCheckboxRenderer, XpRadioGroupRenderer, XpRadioRenderer, XpSliderRenderer,
        XpSwitchRenderer,
    },
    display::{
        XpBadgeRenderer, XpDividerRenderer, XpEmptyStateRenderer, XpFocusRingRenderer,
        XpHeadingRenderer, XpIconRenderer, XpKeybindingDisplayRenderer, XpLabelRenderer,
        XpProgressBarRenderer, XpShortcutHintRenderer, XpSkeletonRenderer, XpSpacerRenderer,
        XpTagRenderer, XpTextRenderer,
    },
    inputs::{
        XpComboBoxRenderer, XpFilePathInputRenderer, XpKeybindingInputRenderer,
        XpNumberInputRenderer, XpPasswordInputRenderer, XpSearchInputRenderer, XpSelectRenderer,
        XpTextAreaRenderer, XpTextInputRenderer,
    },
    lists::{
        XpFormFieldRenderer, XpFormRenderer, XpListItemRenderer, XpListboxRenderer,
        XpTableRenderer, XpTreeItemRenderer, XpTreeRenderer, XpUniformVirtualListRenderer,
        XpVirtualListRenderer,
    },
    notifications::{XpNotificationRenderer, XpToastRenderer},
    overlays::{
        XpDisclosureRenderer, XpDropdownMenuRenderer, XpMenuRenderer, XpModalRenderer,
        XpOverlayRenderer, XpPopoverRenderer,
    },
    surfaces::{
        XpAvatarRenderer, XpCardRenderer, XpImageRenderer, XpPanelRenderer, XpTooltipRenderer,
    },
};

const XP_LUNA: &str = include_str!("../themes/xp-luna.json");

/// Install the XP renderer with the bundled Luna theme.
///
/// XP has no dark mode — this always installs the same theme
/// regardless of system appearance.
pub fn install(cx: &mut App) {
    install_with_default_theme(cx);
}

/// Install the XP renderer with the bundled Luna theme.
pub fn install_with_default_theme(cx: &mut App) {
    let theme = Theme::from_json(XP_LUNA).expect("xp-luna.json is valid");
    install_with(cx, theme);
}

/// Install the XP renderer with a custom theme.
pub fn install_with(cx: &mut App, theme: Theme) {
    install_theme(cx, theme);
    register_xp_renderers(cx);
}

/// Register all 55 XP `XxxRenderer` impls against the core
/// `RendererRegistry`. Public so a caller who already installed
/// the theme (e.g. for tests) can still wire up the XP look
/// without re-installing the theme.
pub fn register_xp_renderers(cx: &mut App) {
    // Actions (5)
    cx.register_renderer_arc::<m::Button, dyn ButtonRenderer>(Arc::new(XpButtonRenderer));
    cx.register_renderer_arc::<m::IconButton, dyn IconButtonRenderer>(Arc::new(
        XpIconButtonRenderer,
    ));
    cx.register_renderer_arc::<m::ToggleButton, dyn ToggleButtonRenderer>(Arc::new(
        XpToggleButtonRenderer,
    ));
    cx.register_renderer_arc::<m::SplitButton, dyn SplitButtonRenderer>(Arc::new(
        XpSplitButtonRenderer,
    ));
    cx.register_renderer_arc::<m::ButtonGroup, dyn ButtonGroupRenderer>(Arc::new(
        XpButtonGroupRenderer,
    ));

    // Display (14)
    cx.register_renderer_arc::<m::Label, dyn LabelRenderer>(Arc::new(XpLabelRenderer));
    cx.register_renderer_arc::<m::Heading, dyn HeadingRenderer>(Arc::new(XpHeadingRenderer));
    cx.register_renderer_arc::<m::Divider, dyn DividerRenderer>(Arc::new(XpDividerRenderer));
    cx.register_renderer_arc::<m::FocusRing, dyn FocusRingRenderer>(Arc::new(XpFocusRingRenderer));
    cx.register_renderer_arc::<m::Badge, dyn BadgeRenderer>(Arc::new(XpBadgeRenderer));
    cx.register_renderer_arc::<m::Tag, dyn TagRenderer>(Arc::new(XpTagRenderer));
    cx.register_renderer_arc::<m::Skeleton, dyn SkeletonRenderer>(Arc::new(XpSkeletonRenderer));
    cx.register_renderer_arc::<m::ProgressBar, dyn ProgressBarRenderer>(Arc::new(
        XpProgressBarRenderer,
    ));
    cx.register_renderer_arc::<m::EmptyState, dyn EmptyStateRenderer>(Arc::new(
        XpEmptyStateRenderer,
    ));
    cx.register_renderer_arc::<m::KeybindingDisplay, dyn KeybindingDisplayRenderer>(Arc::new(
        XpKeybindingDisplayRenderer,
    ));
    cx.register_renderer_arc::<m::ShortcutHint, dyn ShortcutHintRenderer>(Arc::new(
        XpShortcutHintRenderer,
    ));
    cx.register_renderer_arc::<m::Icon, dyn CoreIconRenderer>(Arc::new(XpIconRenderer));
    cx.register_renderer_arc::<m::Text, dyn TextRenderer>(Arc::new(XpTextRenderer));
    cx.register_renderer_arc::<m::Spacer, dyn SpacerRenderer>(Arc::new(XpSpacerRenderer));

    // Surfaces (5)
    cx.register_renderer_arc::<m::Tooltip, dyn TooltipRenderer>(Arc::new(XpTooltipRenderer));
    cx.register_renderer_arc::<m::Avatar, dyn AvatarRenderer>(Arc::new(XpAvatarRenderer));
    cx.register_renderer_arc::<m::Panel, dyn PanelRenderer>(Arc::new(XpPanelRenderer));
    cx.register_renderer_arc::<m::Card, dyn CardRenderer>(Arc::new(XpCardRenderer));
    cx.register_renderer_arc::<m::Image, dyn ImageRenderer>(Arc::new(XpImageRenderer));

    // Inputs (9)
    cx.register_renderer_arc::<m::TextInput, dyn TextInputRenderer>(Arc::new(XpTextInputRenderer));
    cx.register_renderer_arc::<m::TextArea, dyn TextAreaRenderer>(Arc::new(XpTextAreaRenderer));
    cx.register_renderer_arc::<m::PasswordInput, dyn PasswordInputRenderer>(Arc::new(
        XpPasswordInputRenderer,
    ));
    cx.register_renderer_arc::<m::NumberInput, dyn NumberInputRenderer>(Arc::new(
        XpNumberInputRenderer,
    ));
    cx.register_renderer_arc::<m::FilePathInput, dyn FilePathInputRenderer>(Arc::new(
        XpFilePathInputRenderer,
    ));
    cx.register_renderer_arc::<m::SearchInput, dyn SearchInputRenderer>(Arc::new(
        XpSearchInputRenderer,
    ));
    cx.register_renderer_arc::<m::Select, dyn SelectRenderer>(Arc::new(XpSelectRenderer));
    cx.register_renderer_arc::<m::ComboBox, dyn ComboBoxRenderer>(Arc::new(XpComboBoxRenderer));
    cx.register_renderer_arc::<m::KeybindingInput, dyn KeybindingInputRenderer>(Arc::new(
        XpKeybindingInputRenderer,
    ));

    // Controls (5)
    cx.register_renderer_arc::<m::Switch, dyn SwitchRenderer>(Arc::new(XpSwitchRenderer));
    cx.register_renderer_arc::<m::Checkbox, dyn CheckboxRenderer>(Arc::new(XpCheckboxRenderer));
    cx.register_renderer_arc::<m::Radio, dyn RadioRenderer>(Arc::new(XpRadioRenderer));
    cx.register_renderer_arc::<m::RadioGroup, dyn RadioGroupRenderer>(Arc::new(
        XpRadioGroupRenderer,
    ));
    cx.register_renderer_arc::<m::Slider, dyn SliderRenderer>(Arc::new(XpSliderRenderer));

    // Overlays (6)
    cx.register_renderer_arc::<m::Modal, dyn ModalRenderer>(Arc::new(XpModalRenderer));
    cx.register_renderer_arc::<m::Popover, dyn PopoverRenderer>(Arc::new(XpPopoverRenderer));
    cx.register_renderer_arc::<m::DropdownMenu, dyn DropdownMenuRenderer>(Arc::new(
        XpDropdownMenuRenderer,
    ));
    cx.register_renderer_arc::<m::Disclosure, dyn DisclosureRenderer>(Arc::new(
        XpDisclosureRenderer,
    ));
    cx.register_renderer_arc::<m::Overlay, dyn OverlayRenderer>(Arc::new(XpOverlayRenderer));
    cx.register_renderer_arc::<m::Menu, dyn MenuRenderer>(Arc::new(XpMenuRenderer));

    // Notifications (2)
    cx.register_renderer_arc::<m::Toast, dyn ToastRenderer>(Arc::new(XpToastRenderer));
    cx.register_renderer_arc::<m::Notification, dyn NotificationRenderer>(Arc::new(
        XpNotificationRenderer,
    ));

    // Lists (9)
    cx.register_renderer_arc::<m::ListItem, dyn ListItemRenderer>(Arc::new(XpListItemRenderer));
    cx.register_renderer_arc::<m::Listbox, dyn ListboxRenderer>(Arc::new(XpListboxRenderer));
    cx.register_renderer_arc::<m::TreeItem, dyn TreeItemRenderer>(Arc::new(XpTreeItemRenderer));
    cx.register_renderer_arc::<m::Tree, dyn TreeRenderer>(Arc::new(XpTreeRenderer));
    cx.register_renderer_arc::<m::Form, dyn FormRenderer>(Arc::new(XpFormRenderer));
    cx.register_renderer_arc::<m::FormField, dyn FormFieldRenderer>(Arc::new(XpFormFieldRenderer));
    cx.register_renderer_arc::<m::Table, dyn TableRenderer>(Arc::new(XpTableRenderer));
    cx.register_renderer_arc::<m::VirtualList, dyn VirtualListRenderer>(Arc::new(
        XpVirtualListRenderer,
    ));
    cx.register_renderer_arc::<m::UniformVirtualList, dyn UniformVirtualListRenderer>(Arc::new(
        XpUniformVirtualListRenderer,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use yororen_ui_core::renderer::RendererRegistry;

    /// Every one of the 55 `XxxRenderer` slots must resolve after
    /// `register_xp_renderers` — a missing registration panics at
    /// render time, so catch it here instead.
    #[gpui::test]
    fn registers_all_55_renderers(cx: &mut gpui::TestAppContext) {
        cx.update(register_xp_renderers);
        cx.update(|cx| {
            // Actions (5)
            assert!(cx.has_renderer::<m::Button>(), "Button");
            assert!(cx.has_renderer::<m::IconButton>(), "IconButton");
            assert!(cx.has_renderer::<m::ToggleButton>(), "ToggleButton");
            assert!(cx.has_renderer::<m::SplitButton>(), "SplitButton");
            assert!(cx.has_renderer::<m::ButtonGroup>(), "ButtonGroup");
            // Display (14)
            assert!(cx.has_renderer::<m::Label>(), "Label");
            assert!(cx.has_renderer::<m::Heading>(), "Heading");
            assert!(cx.has_renderer::<m::Divider>(), "Divider");
            assert!(cx.has_renderer::<m::FocusRing>(), "FocusRing");
            assert!(cx.has_renderer::<m::Badge>(), "Badge");
            assert!(cx.has_renderer::<m::Tag>(), "Tag");
            assert!(cx.has_renderer::<m::Skeleton>(), "Skeleton");
            assert!(cx.has_renderer::<m::ProgressBar>(), "ProgressBar");
            assert!(cx.has_renderer::<m::EmptyState>(), "EmptyState");
            assert!(
                cx.has_renderer::<m::KeybindingDisplay>(),
                "KeybindingDisplay"
            );
            assert!(cx.has_renderer::<m::ShortcutHint>(), "ShortcutHint");
            assert!(cx.has_renderer::<m::Icon>(), "Icon");
            assert!(cx.has_renderer::<m::Text>(), "Text");
            assert!(cx.has_renderer::<m::Spacer>(), "Spacer");
            // Surfaces (5)
            assert!(cx.has_renderer::<m::Tooltip>(), "Tooltip");
            assert!(cx.has_renderer::<m::Avatar>(), "Avatar");
            assert!(cx.has_renderer::<m::Panel>(), "Panel");
            assert!(cx.has_renderer::<m::Card>(), "Card");
            assert!(cx.has_renderer::<m::Image>(), "Image");
            // Inputs (9)
            assert!(cx.has_renderer::<m::TextInput>(), "TextInput");
            assert!(cx.has_renderer::<m::TextArea>(), "TextArea");
            assert!(cx.has_renderer::<m::PasswordInput>(), "PasswordInput");
            assert!(cx.has_renderer::<m::NumberInput>(), "NumberInput");
            assert!(cx.has_renderer::<m::FilePathInput>(), "FilePathInput");
            assert!(cx.has_renderer::<m::SearchInput>(), "SearchInput");
            assert!(cx.has_renderer::<m::Select>(), "Select");
            assert!(cx.has_renderer::<m::ComboBox>(), "ComboBox");
            assert!(cx.has_renderer::<m::KeybindingInput>(), "KeybindingInput");
            // Controls (5)
            assert!(cx.has_renderer::<m::Switch>(), "Switch");
            assert!(cx.has_renderer::<m::Checkbox>(), "Checkbox");
            assert!(cx.has_renderer::<m::Radio>(), "Radio");
            assert!(cx.has_renderer::<m::RadioGroup>(), "RadioGroup");
            assert!(cx.has_renderer::<m::Slider>(), "Slider");
            // Overlays (6)
            assert!(cx.has_renderer::<m::Modal>(), "Modal");
            assert!(cx.has_renderer::<m::Popover>(), "Popover");
            assert!(cx.has_renderer::<m::DropdownMenu>(), "DropdownMenu");
            assert!(cx.has_renderer::<m::Disclosure>(), "Disclosure");
            assert!(cx.has_renderer::<m::Overlay>(), "Overlay");
            assert!(cx.has_renderer::<m::Menu>(), "Menu");
            // Notifications (2)
            assert!(cx.has_renderer::<m::Toast>(), "Toast");
            assert!(cx.has_renderer::<m::Notification>(), "Notification");
            // Lists (9)
            assert!(cx.has_renderer::<m::ListItem>(), "ListItem");
            assert!(cx.has_renderer::<m::Listbox>(), "Listbox");
            assert!(cx.has_renderer::<m::TreeItem>(), "TreeItem");
            assert!(cx.has_renderer::<m::Tree>(), "Tree");
            assert!(cx.has_renderer::<m::Form>(), "Form");
            assert!(cx.has_renderer::<m::FormField>(), "FormField");
            assert!(cx.has_renderer::<m::Table>(), "Table");
            assert!(cx.has_renderer::<m::VirtualList>(), "VirtualList");
            assert!(
                cx.has_renderer::<m::UniformVirtualList>(),
                "UniformVirtualList"
            );

            assert_eq!(cx.global::<RendererRegistry>().len(), 55);
        });
    }

    /// The bundled Luna theme must parse and carry the key paths
    /// the renderers read.
    #[test]
    fn xp_luna_theme_parses_with_key_paths() {
        let theme = Theme::from_json(XP_LUNA).expect("xp-luna.json parses");

        // Standard palette paths (shared schema with the other
        // renderers).
        for path in [
            "surface.base",
            "content.primary",
            "border.default",
            "action.primary.bg",
            "action.primary.fg",
            "status.success.bg",
        ] {
            assert!(theme.get_color(path).is_some(), "missing color {path}");
        }

        // XP extension palette.
        for path in [
            "xp.titlebar.from",
            "xp.titlebar.mid_1",
            "xp.titlebar.mid_2",
            "xp.titlebar.mid_3",
            "xp.titlebar.to",
            "xp.titlebar.inactive_from",
            "xp.titlebar.inactive_to",
            "xp.button.face_from",
            "xp.button.face_mid",
            "xp.button.face_to",
            "xp.button.hover_ring",
            "xp.progress.track_border",
            "xp.progress.chunk_from",
            "xp.progress.chunk_to",
            "xp.input.border",
            "xp.input.focus_border",
            "xp.selection.bg",
            "xp.selection.hover_bg",
            "xp.menu.hover_bg",
            "xp.menu.hover_fg",
            "xp.toast.bg",
            "xp.toast.border",
            "xp.window.border_active",
            "xp.window.border_inactive",
            "xp.caption.from",
            "xp.caption.close_from",
            "xp.caption.border",
            "xp.check.glyph",
            "xp.explorer.task_pane_bg_from",
            "xp.explorer.task_card_header_from",
            "xp.explorer.task_card_body_from",
            "xp.explorer.task_card_title",
            "xp.explorer.content_bg",
            "xp.explorer.link",
            "xp.explorer.toolbar_border",
            "xp.explorer.group_rule_from",
        ] {
            assert!(theme.get_color(path).is_some(), "missing color {path}");
        }

        // Geometry tokens the renderers fall back on.
        for path in [
            "tokens.control.button.radius",
            "tokens.control.button.min_height",
            "tokens.control.progress.height",
            "tokens.control.checkbox.size",
            "tokens.typography.family_default",
            "xp.progress.segment_width",
            "xp.progress.segment_gap",
        ] {
            assert!(theme.get(path).is_some(), "missing token {path}");
        }
    }
}
