//! Component renderer traits. Phase B spike scope was just `ButtonRenderer`;
//! Phase C generalizes the same shape to all components.

pub mod avatar;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod divider;
pub mod focus_ring;
pub mod heading;
pub mod label;
pub mod progress;
pub mod radio;
pub mod registry;
pub mod skeleton;
pub mod spec;
pub mod switch;
pub mod tag;
pub mod tooltip;

pub use avatar::{AvatarRenderState, AvatarRenderer, TokenAvatarRenderer};
pub use badge::{BadgeRenderState, BadgeRenderer, TokenBadgeRenderer};
pub use button::{ButtonRenderState, ButtonRenderer, TokenButtonRenderer};
pub use checkbox::{CheckboxRenderState, CheckboxRenderer, TokenCheckboxRenderer};
pub use divider::{DividerRenderState, DividerRenderer, TokenDividerRenderer};
pub use focus_ring::{FocusRingRenderState, FocusRingRenderer, TokenFocusRingRenderer};
pub use heading::{HeadingRenderState, HeadingRenderer, TokenHeadingRenderer};
pub use label::{LabelRenderState, LabelRenderer, TokenLabelRenderer};
pub use progress::{ProgressBarRenderState, ProgressBarRenderer, TokenProgressBarRenderer};
pub use radio::{RadioRenderState, RadioRenderer, TokenRadioRenderer};
pub use registry::RendererRegistry;
pub use skeleton::{SkeletonRenderState, SkeletonRenderer, TokenSkeletonRenderer};
pub use spec::{BorderSpec, Edges, IconPosition, ShadowSpec};
pub use switch::{SwitchRenderState, SwitchRenderer, TokenSwitchRenderer};
pub use tag::{TagRenderState, TagRenderer, TokenTagRenderer};
pub use tooltip::{TooltipRenderState, TooltipRenderer, TokenTooltipRenderer};
