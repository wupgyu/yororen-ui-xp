//! `RendererRegistry` — the collection of component renderers wired into
//! a `Theme`. Phase B ships the button entry; Phase C adds the remaining
//! 30+ components one trait per file.

use std::sync::Arc;

use super::avatar::{AvatarRenderer, TokenAvatarRenderer};
use super::badge::{BadgeRenderer, TokenBadgeRenderer};
use super::button::{ButtonRenderer, TokenButtonRenderer};
use super::card::{CardRenderer, TokenCardRenderer};
use super::checkbox::{CheckboxRenderer, TokenCheckboxRenderer};
use super::divider::{DividerRenderer, TokenDividerRenderer};
use super::dropdown_menu::{DropdownMenuRenderer, TokenDropdownMenuRenderer};
use super::focus_ring::{FocusRingRenderer, TokenFocusRingRenderer};
use super::form::{FormRenderer, TokenFormRenderer};
use super::heading::{HeadingRenderer, TokenHeadingRenderer};
use super::icon::{IconRenderer, TokenIconRenderer};
use super::icon_button::{IconButtonRenderer, TokenIconButtonRenderer};
use super::label::{LabelRenderer, TokenLabelRenderer};
use super::list_item::{ListItemRenderer, TokenListItemRenderer};
use super::modal::{ModalRenderer, TokenModalRenderer};
use super::notification::{NotificationRenderer, TokenNotificationRenderer};
use super::popover::{PopoverRenderer, TokenPopoverRenderer};
use super::progress::{ProgressBarRenderer, TokenProgressBarRenderer};
use super::radio::{RadioRenderer, TokenRadioRenderer};
use super::skeleton::{SkeletonRenderer, TokenSkeletonRenderer};
use super::switch::{SwitchRenderer, TokenSwitchRenderer};
use super::tag::{TagRenderer, TokenTagRenderer};
use super::text_input::{TextInputRenderer, TokenTextInputRenderer};
use super::toast::{ToastRenderer, TokenToastRenderer};
use super::toggle_button::{ToggleButtonRenderer, TokenToggleButtonRenderer};
use super::tooltip::{TooltipRenderer, TokenTooltipRenderer};

#[derive(Clone)]
pub struct RendererRegistry {
    pub button: Arc<dyn ButtonRenderer>,
    pub icon_button: Arc<dyn IconButtonRenderer>,
    pub toggle_button: Arc<dyn ToggleButtonRenderer>,
    pub label: Arc<dyn LabelRenderer>,
    pub heading: Arc<dyn HeadingRenderer>,
    pub divider: Arc<dyn DividerRenderer>,
    pub focus_ring: Arc<dyn FocusRingRenderer>,
    pub badge: Arc<dyn BadgeRenderer>,
    pub tag: Arc<dyn TagRenderer>,
    pub progress_bar: Arc<dyn ProgressBarRenderer>,
    pub skeleton: Arc<dyn SkeletonRenderer>,
    pub tooltip: Arc<dyn TooltipRenderer>,
    pub avatar: Arc<dyn AvatarRenderer>,
    pub switch: Arc<dyn SwitchRenderer>,
    pub checkbox: Arc<dyn CheckboxRenderer>,
    pub radio: Arc<dyn RadioRenderer>,
    pub text_input: Arc<dyn TextInputRenderer>,
    pub modal: Arc<dyn ModalRenderer>,
    pub popover: Arc<dyn PopoverRenderer>,
    pub dropdown_menu: Arc<dyn DropdownMenuRenderer>,
    pub toast: Arc<dyn ToastRenderer>,
    pub notification: Arc<dyn NotificationRenderer>,
    pub card: Arc<dyn CardRenderer>,
    pub form: Arc<dyn FormRenderer>,
    pub list_item: Arc<dyn ListItemRenderer>,
    pub icon: Arc<dyn IconRenderer>,
}

impl std::fmt::Debug for RendererRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RendererRegistry").finish_non_exhaustive()
    }
}

impl Default for RendererRegistry {
    fn default() -> Self {
        Self::token_based()
    }
}

impl RendererRegistry {
    /// All renderers set to the default `TokenXxxRenderer` implementations.
    /// This is the v0.3 / v0.4 visual baseline.
    pub fn token_based() -> Self {
        Self {
            button: Arc::new(TokenButtonRenderer),
            icon_button: Arc::new(TokenIconButtonRenderer),
            toggle_button: Arc::new(TokenToggleButtonRenderer),
            label: Arc::new(TokenLabelRenderer),
            heading: Arc::new(TokenHeadingRenderer),
            divider: Arc::new(TokenDividerRenderer),
            focus_ring: Arc::new(TokenFocusRingRenderer),
            badge: Arc::new(TokenBadgeRenderer),
            tag: Arc::new(TokenTagRenderer),
            progress_bar: Arc::new(TokenProgressBarRenderer),
            skeleton: Arc::new(TokenSkeletonRenderer),
            tooltip: Arc::new(TokenTooltipRenderer),
            avatar: Arc::new(TokenAvatarRenderer),
            switch: Arc::new(TokenSwitchRenderer),
            checkbox: Arc::new(TokenCheckboxRenderer),
            radio: Arc::new(TokenRadioRenderer),
            text_input: Arc::new(TokenTextInputRenderer),
            modal: Arc::new(TokenModalRenderer),
            popover: Arc::new(TokenPopoverRenderer),
            dropdown_menu: Arc::new(TokenDropdownMenuRenderer),
            toast: Arc::new(TokenToastRenderer),
            notification: Arc::new(TokenNotificationRenderer),
            card: Arc::new(TokenCardRenderer),
            form: Arc::new(TokenFormRenderer),
            list_item: Arc::new(TokenListItemRenderer),
            icon: Arc::new(TokenIconRenderer),
        }
    }

    pub fn with_button(mut self, r: Arc<dyn ButtonRenderer>) -> Self {
        self.button = r;
        self
    }
    pub fn with_label(mut self, r: Arc<dyn LabelRenderer>) -> Self {
        self.label = r;
        self
    }
    pub fn with_heading(mut self, r: Arc<dyn HeadingRenderer>) -> Self {
        self.heading = r;
        self
    }
    pub fn with_divider(mut self, r: Arc<dyn DividerRenderer>) -> Self {
        self.divider = r;
        self
    }
    pub fn with_focus_ring(mut self, r: Arc<dyn FocusRingRenderer>) -> Self {
        self.focus_ring = r;
        self
    }
    pub fn with_badge(mut self, r: Arc<dyn BadgeRenderer>) -> Self {
        self.badge = r;
        self
    }
    pub fn with_tag(mut self, r: Arc<dyn TagRenderer>) -> Self {
        self.tag = r;
        self
    }
    pub fn with_progress_bar(mut self, r: Arc<dyn ProgressBarRenderer>) -> Self {
        self.progress_bar = r;
        self
    }
    pub fn with_skeleton(mut self, r: Arc<dyn SkeletonRenderer>) -> Self {
        self.skeleton = r;
        self
    }
    pub fn with_tooltip(mut self, r: Arc<dyn TooltipRenderer>) -> Self {
        self.tooltip = r;
        self
    }
    pub fn with_avatar(mut self, r: Arc<dyn AvatarRenderer>) -> Self {
        self.avatar = r;
        self
    }
    pub fn with_switch(mut self, r: Arc<dyn SwitchRenderer>) -> Self {
        self.switch = r;
        self
    }
    pub fn with_checkbox(mut self, r: Arc<dyn CheckboxRenderer>) -> Self {
        self.checkbox = r;
        self
    }
    pub fn with_radio(mut self, r: Arc<dyn RadioRenderer>) -> Self {
        self.radio = r;
        self
    }
    pub fn with_icon_button(mut self, r: Arc<dyn IconButtonRenderer>) -> Self {
        self.icon_button = r;
        self
    }
    pub fn with_toggle_button(mut self, r: Arc<dyn ToggleButtonRenderer>) -> Self {
        self.toggle_button = r;
        self
    }
    pub fn with_text_input(mut self, r: Arc<dyn TextInputRenderer>) -> Self {
        self.text_input = r;
        self
    }
    pub fn with_modal(mut self, r: Arc<dyn ModalRenderer>) -> Self {
        self.modal = r;
        self
    }
    pub fn with_popover(mut self, r: Arc<dyn PopoverRenderer>) -> Self {
        self.popover = r;
        self
    }
    pub fn with_dropdown_menu(mut self, r: Arc<dyn DropdownMenuRenderer>) -> Self {
        self.dropdown_menu = r;
        self
    }
    pub fn with_toast(mut self, r: Arc<dyn ToastRenderer>) -> Self {
        self.toast = r;
        self
    }
    pub fn with_notification(mut self, r: Arc<dyn NotificationRenderer>) -> Self {
        self.notification = r;
        self
    }
    pub fn with_card(mut self, r: Arc<dyn CardRenderer>) -> Self {
        self.card = r;
        self
    }
    pub fn with_form(mut self, r: Arc<dyn FormRenderer>) -> Self {
        self.form = r;
        self
    }
    pub fn with_list_item(mut self, r: Arc<dyn ListItemRenderer>) -> Self {
        self.list_item = r;
        self
    }
    pub fn with_icon(mut self, r: Arc<dyn IconRenderer>) -> Self {
        self.icon = r;
        self
    }
}
