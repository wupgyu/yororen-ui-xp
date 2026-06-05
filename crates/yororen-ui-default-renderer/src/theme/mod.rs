use std::sync::Arc;

use gpui::{App, Global, Hsla};

use yororen_ui_core::i18n::TextDirection;

pub mod tokens;
pub mod validate;

pub use crate::renderers::RendererRegistry;
pub use tokens::{
    DesignTokens, EasingFn, MotionTokens, RadiiTokens, SizeTokens, SpacingTokens, TypographyTokens,
};
pub use validate::{Issue, IssueKind, validate};

#[derive(Clone, Debug)]
pub struct Theme {
    pub surface: SurfaceTheme,
    pub content: ContentTheme,
    pub border: BorderTheme,
    pub action: ActionTheme,
    pub status: StatusTheme,
    pub shadow: ShadowTheme,
    /// Text direction (LTR or RTL)
    pub text_direction: TextDirection,
    /// Design tokens — single source of truth for component geometry, typography,
    /// spacing, radii, and motion. Themes override these to reshape the UI
    /// without touching component logic.
    pub tokens: DesignTokens,
    /// Per-component renderers. The `ButtonRenderer` entry is the
    /// reference example; the other 37 follow the same pattern.
    pub renderers: RendererRegistry,
}

#[derive(Clone, Debug, Default)]
pub struct SurfaceTheme {
    pub canvas: Hsla,
    pub base: Hsla,
    pub raised: Hsla,
    pub sunken: Hsla,
    pub hover: Hsla,
}

#[derive(Clone, Debug, Default)]
pub struct ContentTheme {
    pub primary: Hsla,
    pub secondary: Hsla,
    pub tertiary: Hsla,
    pub disabled: Hsla,
    pub on_primary: Hsla,
    pub on_status: Hsla,
}

#[derive(Clone, Debug, Default)]
pub struct BorderTheme {
    pub default: Hsla,
    pub muted: Hsla,
    pub focus: Hsla,
    pub divider: Hsla,
}

#[derive(Clone, Debug, Default)]
pub struct ActionTheme {
    pub neutral: ActionVariant,
    pub primary: ActionVariant,
    pub danger: ActionVariant,
}

#[derive(Clone, Debug, Default)]
pub struct ActionVariant {
    pub bg: Hsla,
    pub hover_bg: Hsla,
    pub active_bg: Hsla,
    pub fg: Hsla,
    pub disabled_bg: Hsla,
    pub disabled_fg: Hsla,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum ActionVariantKind {
    #[default]
    Neutral,
    Primary,
    Danger,
}

impl ActionVariantKind {
    /// Canonical lowercase string used in diagnostics and as the
    /// legacy `VariantRegistry` builtin key.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Primary => "primary",
            Self::Danger => "danger",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct StatusTheme {
    pub success: StatusVariant,
    pub warning: StatusVariant,
    pub error: StatusVariant,
    pub info: StatusVariant,
}

#[derive(Clone, Debug, Default)]
pub struct StatusVariant {
    pub bg: Hsla,
    pub fg: Hsla,
}

#[derive(Clone, Debug, Default)]
pub struct ShadowTheme {
    pub elevation_1: Hsla,
    pub elevation_2: Hsla,
}

impl Theme {
    /// Check if RTL mode is enabled.
    pub fn is_rtl(&self) -> bool {
        self.text_direction.is_rtl()
    }

    pub fn action_variant(&self, variant: ActionVariantKind) -> &ActionVariant {
        match variant {
            ActionVariantKind::Neutral => &self.action.neutral,
            ActionVariantKind::Primary => &self.action.primary,
            ActionVariantKind::Danger => &self.action.danger,
        }
    }

    /// Populate a `Theme` from a JSON string in the v0.3
    /// `serde_json::Value`-backed schema. Unknown / missing
    /// fields fall back to `Default::default()` (zeros). The
    /// 38 `XxxRenderer` keep reading the strong-typed fields
    /// below — this constructor is the bridge that lets a
    /// JSON file drive the existing v0.4 code without
    /// rewriting all 38 renderers.
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        let core = yororen_ui_core::theme::Theme::from_json(s)?;
        Ok(Self::from_core(&core))
    }

    /// Populate a `Theme` from the v0.3 core `Theme`
    /// (serde_json::Value wrapper). See [`from_json`].
    pub fn from_core(core: &yororen_ui_core::theme::Theme) -> Self {
        fn get_color(c: &yororen_ui_core::theme::Theme, path: &str) -> Hsla {
            c.get_color(path).unwrap_or_default()
        }
        fn get_num(c: &yororen_ui_core::theme::Theme, path: &str) -> f32 {
            c.get_number(path).unwrap_or(0.0) as f32
        }
        fn get_dur(c: &yororen_ui_core::theme::Theme, path: &str) -> std::time::Duration {
            std::time::Duration::from_millis(get_num(c, path) as u64)
        }
        fn get_font_weight(c: &yororen_ui_core::theme::Theme, path: &str) -> gpui::FontWeight {
            let v = c.get_number(path).unwrap_or(400.0) as f32;
            gpui::FontWeight(v)
        }
        fn get_text_direction(c: &yororen_ui_core::theme::Theme) -> TextDirection {
            if c.get_bool("text_direction_rtl").unwrap_or(false) {
                TextDirection::Rtl
            } else {
                TextDirection::Ltr
            }
        }
        let c = core;
        let surface = SurfaceTheme {
            canvas: get_color(c, "surface.canvas"),
            base: get_color(c, "surface.base"),
            raised: get_color(c, "surface.raised"),
            sunken: get_color(c, "surface.sunken"),
            hover: get_color(c, "surface.hover"),
        };
        let content = ContentTheme {
            primary: get_color(c, "content.primary"),
            secondary: get_color(c, "content.secondary"),
            tertiary: get_color(c, "content.tertiary"),
            disabled: get_color(c, "content.disabled"),
            on_primary: get_color(c, "content.on_primary"),
            on_status: get_color(c, "content.on_status"),
        };
        let border = BorderTheme {
            default: get_color(c, "border.default"),
            muted: get_color(c, "border.muted"),
            focus: get_color(c, "border.focus"),
            divider: get_color(c, "border.divider"),
        };
        let make_variant = |key: &str| ActionVariant {
            bg: get_color(c, &format!("action.{}.bg", key)),
            hover_bg: get_color(c, &format!("action.{}.hover_bg", key)),
            active_bg: get_color(c, &format!("action.{}.active_bg", key)),
            fg: get_color(c, &format!("action.{}.fg", key)),
            disabled_bg: get_color(c, &format!("action.{}.disabled_bg", key)),
            disabled_fg: get_color(c, &format!("action.{}.disabled_fg", key)),
        };
        let action = ActionTheme {
            neutral: make_variant("neutral"),
            primary: make_variant("primary"),
            danger: make_variant("danger"),
        };
        let make_status = |kind: &str| StatusVariant {
            bg: get_color(c, &format!("status.{}.bg", kind)),
            fg: get_color(c, &format!("status.{}.fg", kind)),
        };
        let status = StatusTheme {
            success: make_status("success"),
            warning: make_status("warning"),
            error: make_status("error"),
            info: make_status("info"),
        };
        let shadow = ShadowTheme {
            elevation_1: get_color(c, "shadow.elevation_1"),
            elevation_2: get_color(c, "shadow.elevation_2"),
        };
        let sizes = SizeTokens {
            control_h_xxs: gpui::px(get_num(c, "tokens.sizes.control_h_xxs")),
            control_h_xs: gpui::px(get_num(c, "tokens.sizes.control_h_xs")),
            control_h_sm: gpui::px(get_num(c, "tokens.sizes.control_h_sm")),
            control_h_md: gpui::px(get_num(c, "tokens.sizes.control_h_md")),
            control_h_lg: gpui::px(get_num(c, "tokens.sizes.control_h_lg")),
            control_h_xl: gpui::px(get_num(c, "tokens.sizes.control_h_xl")),
            icon_xxs: gpui::px(get_num(c, "tokens.sizes.icon_xxs")),
            icon_xs: gpui::px(get_num(c, "tokens.sizes.icon_xs")),
            icon_sm: gpui::px(get_num(c, "tokens.sizes.icon_sm")),
            icon_md: gpui::px(get_num(c, "tokens.sizes.icon_md")),
            icon_lg: gpui::px(get_num(c, "tokens.sizes.icon_lg")),
            icon_xl: gpui::px(get_num(c, "tokens.sizes.icon_xl")),
            avatar_xs: gpui::px(get_num(c, "tokens.sizes.avatar_xs")),
            avatar_sm: gpui::px(get_num(c, "tokens.sizes.avatar_sm")),
            avatar_md: gpui::px(get_num(c, "tokens.sizes.avatar_md")),
            avatar_lg: gpui::px(get_num(c, "tokens.sizes.avatar_lg")),
            avatar_xl: gpui::px(get_num(c, "tokens.sizes.avatar_xl")),
            status_dot: gpui::px(get_num(c, "tokens.sizes.status_dot")),
        };
        let radii = RadiiTokens {
            none: gpui::px(get_num(c, "tokens.radii.none")),
            xs: gpui::px(get_num(c, "tokens.radii.xs")),
            sm: gpui::px(get_num(c, "tokens.radii.sm")),
            md: gpui::px(get_num(c, "tokens.radii.md")),
            lg: gpui::px(get_num(c, "tokens.radii.lg")),
            xl: gpui::px(get_num(c, "tokens.radii.xl")),
            pill: gpui::px(get_num(c, "tokens.radii.pill")),
        };
        let spacing = SpacingTokens {
            gap_0: gpui::px(get_num(c, "tokens.spacing.gap_0")),
            gap_1: gpui::px(get_num(c, "tokens.spacing.gap_1")),
            gap_2: gpui::px(get_num(c, "tokens.spacing.gap_2")),
            gap_3: gpui::px(get_num(c, "tokens.spacing.gap_3")),
            gap_4: gpui::px(get_num(c, "tokens.spacing.gap_4")),
            gap_5: gpui::px(get_num(c, "tokens.spacing.gap_5")),
            gap_6: gpui::px(get_num(c, "tokens.spacing.gap_6")),
            gap_7: gpui::px(get_num(c, "tokens.spacing.gap_7")),
            inset_xs: gpui::px(get_num(c, "tokens.spacing.inset_xs")),
            inset_sm: gpui::px(get_num(c, "tokens.spacing.inset_sm")),
            inset_md: gpui::px(get_num(c, "tokens.spacing.inset_md")),
            inset_lg: gpui::px(get_num(c, "tokens.spacing.inset_lg")),
            inset_xl: gpui::px(get_num(c, "tokens.spacing.inset_xl")),
        };
        let typography = TypographyTokens {
            font_size_xxs: gpui::px(get_num(c, "tokens.typography.font_size_xxs")),
            font_size_xs: gpui::px(get_num(c, "tokens.typography.font_size_xs")),
            font_size_sm: gpui::px(get_num(c, "tokens.typography.font_size_sm")),
            font_size_md: gpui::px(get_num(c, "tokens.typography.font_size_md")),
            font_size_lg: gpui::px(get_num(c, "tokens.typography.font_size_lg")),
            font_size_xl: gpui::px(get_num(c, "tokens.typography.font_size_xl")),
            font_size_2xl: gpui::px(get_num(c, "tokens.typography.font_size_2xl")),
            line_height_tight: get_num(c, "tokens.typography.line_height_tight"),
            line_height_normal: get_num(c, "tokens.typography.line_height_normal"),
            line_height_loose: get_num(c, "tokens.typography.line_height_loose"),
            weight_regular: get_font_weight(c, "tokens.typography.weight_regular"),
            weight_medium: get_font_weight(c, "tokens.typography.weight_medium"),
            weight_semibold: get_font_weight(c, "tokens.typography.weight_semibold"),
            weight_bold: get_font_weight(c, "tokens.typography.weight_bold"),
            family_default: c
                .get_string("tokens.typography.family_default")
                .unwrap_or("system-ui")
                .to_string()
                .into(),
            family_mono: c
                .get_string("tokens.typography.family_mono")
                .unwrap_or("ui-monospace")
                .to_string()
                .into(),
        };
        let motion = MotionTokens {
            duration_instant: get_dur(c, "tokens.motion.duration_instant"),
            duration_very_fast: get_dur(c, "tokens.motion.duration_very_fast"),
            duration_fast: get_dur(c, "tokens.motion.duration_fast"),
            duration_normal: get_dur(c, "tokens.motion.duration_normal"),
            duration_slow: get_dur(c, "tokens.motion.duration_slow"),
            duration_very_slow: get_dur(c, "tokens.motion.duration_very_slow"),
            duration_cursor_blink: get_dur(c, "tokens.motion.duration_cursor_blink"),
            duration_skeleton_pulse: get_dur(c, "tokens.motion.duration_skeleton_pulse"),
            duration_progress_spinner: get_dur(c, "tokens.motion.duration_progress_spinner"),
            duration_modal_fade: get_dur(c, "tokens.motion.duration_modal_fade"),
            duration_modal_slide_up: get_dur(c, "tokens.motion.duration_modal_slide_up"),
            duration_menu_open: get_dur(c, "tokens.motion.duration_menu_open"),
            duration_menu_open_fast: get_dur(c, "tokens.motion.duration_menu_open_fast"),
            duration_menu_open_slow: get_dur(c, "tokens.motion.duration_menu_open_slow"),
            duration_tooltip_show: get_dur(c, "tokens.motion.duration_tooltip_show"),
            duration_tooltip_hide: get_dur(c, "tokens.motion.duration_tooltip_hide"),
            duration_navigator_slider: get_dur(c, "tokens.motion.duration_navigator_slider"),
            duration_tab_switch: get_dur(c, "tokens.motion.duration_tab_switch"),
            duration_skeleton_pulse_1: get_dur(c, "tokens.motion.duration_skeleton_pulse_1"),
            duration_skeleton_pulse_2: get_dur(c, "tokens.motion.duration_skeleton_pulse_2"),
            duration_progress_circle: get_dur(c, "tokens.motion.duration_progress_circle"),
            duration_progress_bar: get_dur(c, "tokens.motion.duration_progress_bar"),
            easing_linear: tokens::linear,
            easing_standard: tokens::ease_in_out,
            easing_emphasized: tokens::ease_out_cubic,
            easing_decelerate: tokens::ease_out_quint,
            easing_accelerate: tokens::ease_in,
            pulse_min_opacity: get_num(c, "tokens.motion.pulse_min_opacity"),
            pulse_max_opacity: get_num(c, "tokens.motion.pulse_max_opacity"),
            slide_distance: get_num(c, "tokens.motion.slide_distance"),
            bounce_distance: get_num(c, "tokens.motion.bounce_distance"),
        };
        // Build the 36 ControlTokens sub-structs. This is mechanical
        // because they all follow `get_num` + `gpui::px` from a path.
        let px_num = |path: &str| gpui::px(get_num(c, path));
        let control = tokens::ControlTokens {
            button: tokens::ButtonTokens {
                min_height: px_num("tokens.control.button.min_height"),
                icon_button_min_size: px_num("tokens.control.button.icon_button_min_size"),
                horizontal_padding: px_num("tokens.control.button.horizontal_padding"),
                icon_gap: px_num("tokens.control.button.icon_gap"),
            },
            input: tokens::InputTokens {
                min_height: px_num("tokens.control.input.min_height"),
                horizontal_padding: px_num("tokens.control.input.horizontal_padding"),
                vertical_padding: px_num("tokens.control.input.vertical_padding"),
                cursor_thickness: px_num("tokens.control.input.cursor_thickness"),
                focus_ring_thickness: px_num("tokens.control.input.focus_ring_thickness"),
                icon_gap: px_num("tokens.control.input.icon_gap"),
                spinner_size: px_num("tokens.control.input.spinner_size"),
                text_area_min_h: px_num("tokens.control.input.text_area_min_h"),
            },
            switch: tokens::SwitchTokens {
                track_w: px_num("tokens.control.switch.track_w"),
                track_h: px_num("tokens.control.switch.track_h"),
                knob_size: px_num("tokens.control.switch.knob_size"),
                padding: px_num("tokens.control.switch.padding"),
                border_w: px_num("tokens.control.switch.border_w"),
                focus_border_w: px_num("tokens.control.switch.focus_border_w"),
                duration: get_dur(c, "tokens.control.switch.duration"),
                disabled_opacity: get_num(c, "tokens.control.switch.disabled_opacity"),
            },
            checkbox: tokens::CheckboxTokens {
                box_size: px_num("tokens.control.checkbox.box_size"),
                check_size: px_num("tokens.control.checkbox.check_size"),
                border_w: px_num("tokens.control.checkbox.border_w"),
                focus_border_w: px_num("tokens.control.checkbox.focus_border_w"),
                border_radius: px_num("tokens.control.checkbox.border_radius"),
            },
            radio: tokens::RadioTokens {
                ring_size: px_num("tokens.control.radio.ring_size"),
                dot_size: px_num("tokens.control.radio.dot_size"),
                border_w: px_num("tokens.control.radio.border_w"),
                border_radius: px_num("tokens.control.radio.border_radius"),
            },
            select: tokens::SelectTokens {
                min_height: px_num("tokens.control.select.min_height"),
                horizontal_padding: px_num("tokens.control.select.horizontal_padding"),
                chevron_size: px_num("tokens.control.select.chevron_size"),
                menu_min_width: px_num("tokens.control.select.menu_min_width"),
                menu_width: px_num("tokens.control.select.menu_width"),
                menu_max_height: px_num("tokens.control.select.menu_max_height"),
                item_padding_y: px_num("tokens.control.select.item_padding_y"),
                item_padding_x: px_num("tokens.control.select.item_padding_x"),
            },
            combo_box: tokens::ComboBoxTokens {
                min_height: px_num("tokens.control.combo_box.min_height"),
                horizontal_padding: px_num("tokens.control.combo_box.horizontal_padding"),
                menu_width: px_num("tokens.control.combo_box.menu_width"),
                menu_max_height: px_num("tokens.control.combo_box.menu_max_height"),
                search_gap: px_num("tokens.control.combo_box.search_gap"),
                item_padding_y: px_num("tokens.control.combo_box.item_padding_y"),
                item_padding_x: px_num("tokens.control.combo_box.item_padding_x"),
                icon_size: px_num("tokens.control.combo_box.icon_size"),
            },
            slider: tokens::SliderTokens {
                track_h: px_num("tokens.control.slider.track_h"),
                thumb_size: px_num("tokens.control.slider.thumb_size"),
                hit_padding: px_num("tokens.control.slider.hit_padding"),
                focus_ring: px_num("tokens.control.slider.focus_ring"),
            },
            toast: tokens::ToastTokens {
                min_width: px_num("tokens.control.toast.min_width"),
                max_width: px_num("tokens.control.toast.max_width"),
                horizontal_padding: px_num("tokens.control.toast.horizontal_padding"),
                vertical_padding: px_num("tokens.control.toast.vertical_padding"),
                gap: px_num("tokens.control.toast.gap"),
                close_icon_size: px_num("tokens.control.toast.close_icon_size"),
            },
            modal: tokens::ModalTokens {
                min_width: px_num("tokens.control.modal.min_width"),
                max_width: px_num("tokens.control.modal.max_width"),
                padding: px_num("tokens.control.modal.padding"),
                header_gap: px_num("tokens.control.modal.header_gap"),
                footer_gap: px_num("tokens.control.modal.footer_gap"),
                scrim_blur: px_num("tokens.control.modal.scrim_blur"),
                border_radius: px_num("tokens.control.modal.border_radius"),
            },
            popover: tokens::PopoverTokens {
                padding_x: px_num("tokens.control.popover.padding_x"),
                padding_y: px_num("tokens.control.popover.padding_y"),
                min_width: px_num("tokens.control.popover.min_width"),
                max_width: px_num("tokens.control.popover.max_width"),
                max_height: px_num("tokens.control.popover.max_height"),
                arrow_size: px_num("tokens.control.popover.arrow_size"),
                offset: px_num("tokens.control.popover.offset"),
            },
            dropdown: tokens::DropdownTokens {
                padding_x: px_num("tokens.control.dropdown.padding_x"),
                padding_y: px_num("tokens.control.dropdown.padding_y"),
                min_width: px_num("tokens.control.dropdown.min_width"),
                max_width: px_num("tokens.control.dropdown.max_width"),
                max_height: px_num("tokens.control.dropdown.max_height"),
                item_gap: px_num("tokens.control.dropdown.item_gap"),
                icon_size: px_num("tokens.control.dropdown.icon_size"),
            },
            badge: tokens::BadgeTokens {
                min_height: px_num("tokens.control.badge.min_height"),
                horizontal_padding: px_num("tokens.control.badge.horizontal_padding"),
                gap: px_num("tokens.control.badge.gap"),
                icon_size: px_num("tokens.control.badge.icon_size"),
            },
            tag: tokens::TagTokens {
                min_height: px_num("tokens.control.tag.min_height"),
                horizontal_padding: px_num("tokens.control.tag.horizontal_padding"),
                gap: px_num("tokens.control.tag.gap"),
                close_button_size: px_num("tokens.control.tag.close_button_size"),
                close_icon_size: px_num("tokens.control.tag.close_icon_size"),
            },
            skeleton: tokens::SkeletonTokens {
                line_h: px_num("tokens.control.skeleton.line_h"),
                line_min_w: px_num("tokens.control.skeleton.line_min_w"),
                block_min_h: px_num("tokens.control.skeleton.block_min_h"),
                border_radius: px_num("tokens.control.skeleton.border_radius"),
            },
            progress: tokens::ProgressTokens {
                bar_h_sm: px_num("tokens.control.progress.bar_h_sm"),
                bar_h_md: px_num("tokens.control.progress.bar_h_md"),
                bar_h_lg: px_num("tokens.control.progress.bar_h_lg"),
                bar_default_h: px_num("tokens.control.progress.bar_default_h"),
                spinner_size_sm: px_num("tokens.control.progress.spinner_size_sm"),
                spinner_size_md: px_num("tokens.control.progress.spinner_size_md"),
                spinner_size_lg: px_num("tokens.control.progress.spinner_size_lg"),
                circle_size_sm: px_num("tokens.control.progress.circle_size_sm"),
                circle_size_md: px_num("tokens.control.progress.circle_size_md"),
                circle_size_lg: px_num("tokens.control.progress.circle_size_lg"),
                track_radius: px_num("tokens.control.progress.track_radius"),
                steps_gap: px_num("tokens.control.progress.steps_gap"),
            },
            avatar: tokens::AvatarTokens {
                border_w: px_num("tokens.control.avatar.border_w"),
                status_inset: px_num("tokens.control.avatar.status_inset"),
                status_dot_size: px_num("tokens.control.avatar.status_dot_size"),
                fallback_font_size: px_num("tokens.control.avatar.fallback_font_size"),
            },
            tooltip: tokens::TooltipTokens {
                padding_x: px_num("tokens.control.tooltip.padding_x"),
                padding_y: px_num("tokens.control.tooltip.padding_y"),
                max_width: px_num("tokens.control.tooltip.max_width"),
                arrow_size: px_num("tokens.control.tooltip.arrow_size"),
                offset: px_num("tokens.control.tooltip.offset"),
            },
            disclosure: tokens::DisclosureTokens {
                icon_size: px_num("tokens.control.disclosure.icon_size"),
                chevron_size: px_num("tokens.control.disclosure.chevron_size"),
            },
            keybinding_input: tokens::KeybindingInputTokens {
                kbd_padding_x: px_num("tokens.control.keybinding_input.kbd_padding_x"),
                kbd_padding_y: px_num("tokens.control.keybinding_input.kbd_padding_y"),
                kbd_min_width: px_num("tokens.control.keybinding_input.kbd_min_width"),
                separator_gap: px_num("tokens.control.keybinding_input.separator_gap"),
                icon_size: px_num("tokens.control.keybinding_input.icon_size"),
            },
            split_button: tokens::SplitButtonTokens {
                min_height: px_num("tokens.control.split_button.min_height"),
                chevron_width: px_num("tokens.control.split_button.chevron_width"),
                separator_w: px_num("tokens.control.split_button.separator_w"),
            },
            search_input: tokens::SearchInputTokens {
                min_height: px_num("tokens.control.search_input.min_height"),
                horizontal_padding: px_num("tokens.control.search_input.horizontal_padding"),
                icon_size: px_num("tokens.control.search_input.icon_size"),
                clear_icon_size: px_num("tokens.control.search_input.clear_icon_size"),
                spinner_size: px_num("tokens.control.search_input.spinner_size"),
                input_gap: px_num("tokens.control.search_input.input_gap"),
            },
            number_input: tokens::NumberInputTokens {
                min_height: px_num("tokens.control.number_input.min_height"),
                stepper_button_size: px_num("tokens.control.number_input.stepper_button_size"),
                stepper_icon_size: px_num("tokens.control.number_input.stepper_icon_size"),
                stepper_gap: px_num("tokens.control.number_input.stepper_gap"),
                horizontal_padding: px_num("tokens.control.number_input.horizontal_padding"),
            },
            file_path_input: tokens::FilePathInputTokens {
                min_height: px_num("tokens.control.file_path_input.min_height"),
                horizontal_padding: px_num("tokens.control.file_path_input.horizontal_padding"),
                icon_size: px_num("tokens.control.file_path_input.icon_size"),
                action_button_size: px_num("tokens.control.file_path_input.action_button_size"),
                action_gap: px_num("tokens.control.file_path_input.action_gap"),
            },
            icon_button: tokens::IconButtonTokens {
                min_size: px_num("tokens.control.icon_button.min_size"),
                icon_size: px_num("tokens.control.icon_button.icon_size"),
            },
            toggle_button: tokens::ToggleButtonTokens {
                min_height: px_num("tokens.control.toggle_button.min_height"),
                horizontal_padding: px_num("tokens.control.toggle_button.horizontal_padding"),
                icon_gap: px_num("tokens.control.toggle_button.icon_gap"),
            },
            empty_state: tokens::EmptyStateTokens {
                icon_size: px_num("tokens.control.empty_state.icon_size"),
                title_font_size: px_num("tokens.control.empty_state.title_font_size"),
                description_font_size: px_num("tokens.control.empty_state.description_font_size"),
                action_gap: px_num("tokens.control.empty_state.action_gap"),
                vertical_padding: px_num("tokens.control.empty_state.vertical_padding"),
            },
            list_item: tokens::ListItemTokens {
                min_height: px_num("tokens.control.list_item.min_height"),
                horizontal_padding: px_num("tokens.control.list_item.horizontal_padding"),
                gap: px_num("tokens.control.list_item.gap"),
                icon_size: px_num("tokens.control.list_item.icon_size"),
                chevron_size: px_num("tokens.control.list_item.chevron_size"),
            },
            tree_item: tokens::TreeItemTokens {
                min_height: px_num("tokens.control.tree_item.min_height"),
                horizontal_padding: px_num("tokens.control.tree_item.horizontal_padding"),
                indent: px_num("tokens.control.tree_item.indent"),
                chevron_size: px_num("tokens.control.tree_item.chevron_size"),
                icon_size: px_num("tokens.control.tree_item.icon_size"),
                drag_handle_w: px_num("tokens.control.tree_item.drag_handle_w"),
                drop_indicator_h: px_num("tokens.control.tree_item.drop_indicator_h"),
            },
            card: tokens::CardTokens {
                padding: px_num("tokens.control.card.padding"),
                gap: px_num("tokens.control.card.gap"),
                border_radius: px_num("tokens.control.card.border_radius"),
            },
            divider: tokens::DividerTokens {
                thickness: px_num("tokens.control.divider.thickness"),
                margin_x: px_num("tokens.control.divider.margin_x"),
                margin_y: px_num("tokens.control.divider.margin_y"),
            },
            form: tokens::FormTokens {
                field_gap: px_num("tokens.control.form.field_gap"),
                label_gap: px_num("tokens.control.form.label_gap"),
                helper_gap: px_num("tokens.control.form.helper_gap"),
                error_gap: px_num("tokens.control.form.error_gap"),
                group_gap: px_num("tokens.control.form.group_gap"),
                horizontal_field_gap: px_num("tokens.control.form.horizontal_field_gap"),
                horizontal_label_width: px_num("tokens.control.form.horizontal_label_width"),
            },
            notification: tokens::NotificationTokens {
                min_width: px_num("tokens.control.notification.min_width"),
                max_width: px_num("tokens.control.notification.max_width"),
                horizontal_padding: px_num("tokens.control.notification.horizontal_padding"),
                vertical_padding: px_num("tokens.control.notification.vertical_padding"),
                gap: px_num("tokens.control.notification.gap"),
                icon_size: px_num("tokens.control.notification.icon_size"),
                close_icon_size: px_num("tokens.control.notification.close_icon_size"),
                host_padding: px_num("tokens.control.notification.host_padding"),
                host_gap: px_num("tokens.control.notification.host_gap"),
            },
            focus_ring: tokens::FocusRingTokens {
                thickness: px_num("tokens.control.focus_ring.thickness"),
                offset: px_num("tokens.control.focus_ring.offset"),
            },
        };
        Self {
            surface,
            content,
            border,
            action,
            status,
            shadow,
            text_direction: get_text_direction(c),
            tokens: DesignTokens {
                sizes,
                radii,
                spacing,
                typography,
                motion,
                control,
            },
            renderers: crate::renderers::registry::RendererRegistry::token_based(),
        }
    }
}

// Compile-time proof that `Theme` is `Send + Sync`.
//
// `Theme` is stored inside `GlobalTheme` and shared across gpui worker
// threads. The unsoundness risk is that a future field could introduce
// interior mutability that is *not* `Send + Sync` (e.g. `RefCell<…>`)
// and break that assumption silently. This assertion makes any such
// regression a hard compile error.
//
// Manually verified (2026-06-04): all fields are `Send + Sync`:
//   - palette fields: `Hsla` (`Copy`, trivially `Send + Sync`)
//   - `text_direction: TextDirection` (`enum`)
//   - `tokens: DesignTokens` — all leaf fields are `Pixels` / `Duration`
//   - `renderers: RendererRegistry` — 40+ `Arc<dyn …Renderer>` where
//     every `*Renderer` trait is declared `: Send + Sync`.
//
// If you add a new field, re-verify and update this comment.
const _: fn() = || {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Theme>();
    assert_send_sync::<GlobalTheme>();
};

pub struct GlobalTheme {
    theme: Arc<Theme>,
}

impl Global for GlobalTheme {}

impl GlobalTheme {
    /// Install `theme` as the single process-global theme.
    ///
    /// `core` is headless: it does not ship a default palette. Use a
    /// theme package (e.g. `yororen_ui_theme_system::install`) to
    /// obtain a `Theme` and pass it here.
    ///
    /// As of the headless-core cutover, this is the only identity.
    /// The previous `ThemeSet` (light/dark factory) and
    /// `new_with_themes(appearance, …)` were removed because three
    /// parallel identity systems caused boundary confusion. The
    /// model now is: the app picks the right `Theme` for the OS
    /// appearance, then sets it once.
    pub fn new(theme: impl Into<Arc<Theme>>) -> Self {
        Self {
            theme: theme.into(),
        }
    }

    fn theme(cx: &App) -> &Arc<Theme> {
        &cx.global::<Self>().theme
    }

    /// Read-only accessor for the active `Arc<Theme>`. Useful when an
    /// app needs to clone / mutate the theme (e.g. to flip
    /// `text_direction` for an RTL locale) without going through
    /// `cx.global::<GlobalTheme>()`.
    pub fn current(&self) -> &Arc<Theme> {
        &self.theme
    }

    /// Consume the wrapper and return the underlying `Arc<Theme>`.
    /// Useful when re-wrapping the same theme with different
    /// `WindowAppearance` selection.
    pub fn into_arc(self) -> Arc<Theme> {
        self.theme
    }
}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

#[derive(Clone, Copy)]
pub struct InteractiveColors {
    pub bg: Hsla,
    pub hover_bg: Hsla,
    pub active_bg: Hsla,
    pub fg: Hsla,
    pub disabled_bg: Hsla,
    pub disabled_fg: Hsla,
}

pub fn interactive_colors(theme: &Theme) -> InteractiveColors {
    let neutral = &theme.action.neutral;
    InteractiveColors {
        bg: neutral.bg,
        hover_bg: neutral.hover_bg,
        active_bg: neutral.active_bg,
        fg: neutral.fg,
        disabled_bg: neutral.disabled_bg,
        disabled_fg: neutral.disabled_fg,
    }
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        GlobalTheme::theme(self)
    }
}

// Performance note:
//
// `cx.theme()` is hot in every render. It returns `&Arc<Theme>`,
// not `&Theme`, so:
//   - `let theme = cx.theme();` is a single reference copy (no
//     atomic increment).
//   - `let theme = cx.theme().clone();` is one `Arc::clone` (one
//     atomic increment). It does NOT recursively clone
//     `Theme.renderers` — `Arc::clone` only touches the outer
//     `Arc<Theme>`'s refcount.
//
// The earlier concern that "1000 nodes = 40k Arc clones" was based
// on the misreading that `Theme::clone` was deep. It is not:
// `Theme` is wrapped in `Arc<Theme>` inside `GlobalTheme`, and
// that is what `cx.theme()` returns. Renderer methods that take
// `&Theme` are reached by a single deref.
//
// Where to watch out: `cx.global::<GlobalTheme>().current()` (or
// any code path that explicitly clones the inner `Theme`) will
// invoke `Theme::clone`, which is `#[derive(Clone)]` and DOES
// recursively clone every field — including the 40+ `Arc<dyn …>`
// in `RendererRegistry`. So:
//   - Prefer `cx.theme()` (or `cx.theme().clone()`) for read-only
//     access.
//   - Never call `cx.global::<GlobalTheme>().current().as_ref().clone()`
//     inside a render loop; that allocates 40+ Arc::clone per
//     frame.

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Rgba, hsla, rgb};

    /// Internal test fixtures for theme-contrast tests. `core` is headless
    /// and does not export a default palette; these fixtures are used only
    /// to exercise the contrast helper. The real, exposed default themes
    /// live in `yororen_ui_theme_system`.
    fn fixture_themes() -> [(&'static str, Theme); 2] {
        [("dark", fixture_dark()), ("light", fixture_light())]
    }

    fn fixture_dark() -> Theme {
        let content = ContentTheme {
            primary: rgb(0xF2F2F3).into(),
            secondary: rgb(0xC8C8CC).into(),
            tertiary: rgb(0x9B9BA1).into(),
            disabled: rgb(0x6F6F76).into(),
            on_primary: rgb(0x0B0B0D).into(),
            on_status: rgb(0x0B0B0D).into(),
        };
        Theme {
            surface: SurfaceTheme {
                canvas: rgb(0x0F0F11).into(),
                base: rgb(0x151518).into(),
                raised: rgb(0x1D1D21).into(),
                sunken: rgb(0x111113).into(),
                hover: rgb(0x232327).into(),
            },
            content: content.clone(),
            border: BorderTheme {
                default: rgb(0x2A2A2F).into(),
                muted: rgb(0x1E1E22).into(),
                focus: rgb(0x8BB0FF).into(),
                divider: rgb(0x1E1E22).into(),
            },
            action: ActionTheme {
                neutral: ActionVariant {
                    bg: rgb(0x1D1D21).into(),
                    hover_bg: rgb(0x24242A).into(),
                    active_bg: rgb(0x2A2A31).into(),
                    fg: content.primary,
                    disabled_bg: rgb(0x1A1A1D).into(),
                    disabled_fg: content.disabled,
                },
                primary: ActionVariant {
                    bg: rgb(0xF4F4F6).into(),
                    hover_bg: rgb(0xFFFFFF).into(),
                    active_bg: rgb(0xE9E9EC).into(),
                    fg: content.on_primary,
                    disabled_bg: rgb(0xE0E0E4).into(),
                    disabled_fg: rgb(0x5B5B61).into(),
                },
                danger: ActionVariant {
                    bg: rgb(0xFFB4AE).into(),
                    hover_bg: rgb(0xFFA099).into(),
                    active_bg: rgb(0xFF8A82).into(),
                    fg: content.on_status,
                    disabled_bg: rgb(0xE0B3AF).into(),
                    disabled_fg: rgb(0x5B5B61).into(),
                },
            },
            status: StatusTheme {
                success: StatusVariant {
                    bg: rgb(0xB9F5C9).into(),
                    fg: content.on_status,
                },
                warning: StatusVariant {
                    bg: rgb(0xFFE1A6).into(),
                    fg: content.on_status,
                },
                error: StatusVariant {
                    bg: rgb(0xFFB4AE).into(),
                    fg: content.on_status,
                },
                info: StatusVariant {
                    bg: rgb(0xB6D9FF).into(),
                    fg: content.on_status,
                },
            },
            shadow: ShadowTheme {
                elevation_1: hsla(0.0, 0.0, 0.0, 0.3),
                elevation_2: hsla(0.0, 0.0, 0.0, 0.45),
            },
            text_direction: TextDirection::Ltr,
            tokens: DesignTokens::default(),
            renderers: RendererRegistry::token_based(),
        }
    }

    fn fixture_light() -> Theme {
        let content = ContentTheme {
            primary: rgb(0x141416).into(),
            secondary: rgb(0x3E3E45).into(),
            tertiary: rgb(0x6B6B73).into(),
            disabled: rgb(0x9A9AA2).into(),
            on_primary: rgb(0xFFFFFF).into(),
            on_status: rgb(0x0B0B0D).into(),
        };
        Theme {
            surface: SurfaceTheme {
                canvas: rgb(0xF4F4F6).into(),
                base: rgb(0xFFFFFF).into(),
                raised: rgb(0xFBFBFD).into(),
                sunken: rgb(0xEFEFF2).into(),
                hover: rgb(0xE6E6EA).into(),
            },
            content: content.clone(),
            border: BorderTheme {
                default: rgb(0xD8D8DD).into(),
                muted: rgb(0xE3E3E8).into(),
                focus: rgb(0x2F63FF).into(),
                divider: rgb(0xE3E3E8).into(),
            },
            action: ActionTheme {
                neutral: ActionVariant {
                    bg: rgb(0xF1F1F3).into(),
                    hover_bg: rgb(0xE6E6EA).into(),
                    active_bg: rgb(0xDADADF).into(),
                    fg: content.primary,
                    disabled_bg: rgb(0xE7E7EA).into(),
                    disabled_fg: content.disabled,
                },
                primary: ActionVariant {
                    bg: rgb(0x121214).into(),
                    hover_bg: rgb(0x0C0C0D).into(),
                    active_bg: rgb(0x000000).into(),
                    fg: content.on_primary,
                    disabled_bg: rgb(0x2A2A2E).into(),
                    disabled_fg: rgb(0xD0D0D6).into(),
                },
                danger: ActionVariant {
                    bg: rgb(0xFFB4AE).into(),
                    hover_bg: rgb(0xFFA099).into(),
                    active_bg: rgb(0xFF8A82).into(),
                    fg: content.on_status,
                    disabled_bg: rgb(0xF0CBC7).into(),
                    disabled_fg: content.disabled,
                },
            },
            status: StatusTheme {
                success: StatusVariant {
                    bg: rgb(0xB9F5C9).into(),
                    fg: content.on_status,
                },
                warning: StatusVariant {
                    bg: rgb(0xFFE1A6).into(),
                    fg: content.on_status,
                },
                error: StatusVariant {
                    bg: rgb(0xFFB4AE).into(),
                    fg: content.on_status,
                },
                info: StatusVariant {
                    bg: rgb(0xB6D9FF).into(),
                    fg: content.on_status,
                },
            },
            shadow: ShadowTheme {
                elevation_1: hsla(0.0, 0.0, 0.0, 0.18),
                elevation_2: hsla(0.0, 0.0, 0.0, 0.3),
            },
            text_direction: TextDirection::Ltr,
            tokens: DesignTokens::default(),
            renderers: RendererRegistry::token_based(),
        }
    }

    fn relative_luminance(color: Hsla) -> f32 {
        let rgb = Rgba::from(color);
        let linear = |c: f32| {
            if c <= 0.03928 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        let r = linear(rgb.r);
        let g = linear(rgb.g);
        let b = linear(rgb.b);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    fn contrast_ratio(a: Hsla, b: Hsla) -> f32 {
        let l1 = relative_luminance(a);
        let l2 = relative_luminance(b);
        let (lighter, darker) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }

    fn assert_contrast_at_least(label: &str, a: Hsla, b: Hsla, min: f32) {
        let ratio = contrast_ratio(a, b);
        assert!(ratio >= min, "{label} contrast {ratio:.2} below {min:.2}");
    }

    #[test]
    fn theme_contrast_requirements() {
        for (name, theme) in fixture_themes() {
            assert_contrast_at_least(
                &format!("{name}: surface.base/content.primary"),
                theme.surface.base,
                theme.content.primary,
                4.5,
            );
            assert_contrast_at_least(
                &format!("{name}: action.neutral"),
                theme.action.neutral.bg,
                theme.action.neutral.fg,
                4.5,
            );
            assert_contrast_at_least(
                &format!("{name}: action.primary"),
                theme.action.primary.bg,
                theme.action.primary.fg,
                4.5,
            );
            assert_contrast_at_least(
                &format!("{name}: status.success"),
                theme.status.success.bg,
                theme.status.success.fg,
                4.5,
            );
            assert_contrast_at_least(
                &format!("{name}: status.warning"),
                theme.status.warning.bg,
                theme.status.warning.fg,
                4.5,
            );
            assert_contrast_at_least(
                &format!("{name}: status.error"),
                theme.status.error.bg,
                theme.status.error.fg,
                4.5,
            );
            assert_contrast_at_least(
                &format!("{name}: status.info"),
                theme.status.info.bg,
                theme.status.info.fg,
                4.5,
            );
            assert_contrast_at_least(
                &format!("{name}: border.focus"),
                theme.surface.base,
                theme.border.focus,
                3.0,
            );
        }
    }
}
