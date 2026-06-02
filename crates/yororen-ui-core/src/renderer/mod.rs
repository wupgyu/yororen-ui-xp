//! Component renderer traits. Phase B spike scope was just `ButtonRenderer`;
//! Phase C generalizes the same shape to all components.

pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod divider;
pub mod dropdown_menu;
pub mod focus_ring;
pub mod form;
pub mod heading;
pub mod icon;
pub mod icon_button;
pub mod label;
pub mod list_item;
pub mod modal;
pub mod notification;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod registry;
pub mod skeleton;
pub mod spec;
pub mod switch;
pub mod tag;
pub mod text_input;
pub mod toast;
pub mod toggle_button;
pub mod tooltip;

pub use avatar::{AvatarRenderState, AvatarRenderer, TokenAvatarRenderer};
pub use badge::{BadgeRenderState, BadgeRenderer, TokenBadgeRenderer};
pub use button::{ButtonRenderState, ButtonRenderer, TokenButtonRenderer};
pub use card::{CardRenderState, CardRenderer, TokenCardRenderer};
pub use checkbox::{CheckboxRenderState, CheckboxRenderer, TokenCheckboxRenderer};
pub use divider::{DividerRenderState, DividerRenderer, TokenDividerRenderer};
pub use dropdown_menu::{
    DropdownMenuRenderState, DropdownMenuRenderer, TokenDropdownMenuRenderer,
};
pub use focus_ring::{FocusRingRenderState, FocusRingRenderer, TokenFocusRingRenderer};
pub use form::{FormRenderState, FormRenderer, TokenFormRenderer};
pub use heading::{HeadingRenderState, HeadingRenderer, TokenHeadingRenderer};
pub use icon::{IconRenderState, IconRenderer, IconSizePreset, TokenIconRenderer};
pub use icon_button::{IconButtonRenderState, IconButtonRenderer, TokenIconButtonRenderer};
pub use label::{LabelRenderState, LabelRenderer, TokenLabelRenderer};
pub use list_item::{ListItemRenderState, ListItemRenderer, TokenListItemRenderer};
pub use modal::{ModalRenderState, ModalRenderer, TokenModalRenderer};
pub use notification::{
    NotificationRenderState, NotificationRenderer, TokenNotificationRenderer,
};
pub use popover::{PopoverRenderState, PopoverRenderer, TokenPopoverRenderer};
pub use progress::{ProgressBarRenderState, ProgressBarRenderer, TokenProgressBarRenderer};
pub use radio::{RadioRenderState, RadioRenderer, TokenRadioRenderer};
pub use registry::RendererRegistry;
pub use skeleton::{SkeletonRenderState, SkeletonRenderer, TokenSkeletonRenderer};
pub use spec::{BorderSpec, Edges, IconPosition, ShadowSpec};
pub use switch::{SwitchRenderState, SwitchRenderer, TokenSwitchRenderer};
pub use tag::{TagRenderState, TagRenderer, TokenTagRenderer};
pub use text_input::{TextInputRenderState, TextInputRenderer, TokenTextInputRenderer};
pub use toast::{ToastRenderState, ToastRenderer, TokenToastRenderer};
pub use toggle_button::{
    ToggleButtonRenderState, ToggleButtonRenderer, TokenToggleButtonRenderer,
};
pub use tooltip::{TooltipRenderState, TooltipRenderer, TokenTooltipRenderer};
