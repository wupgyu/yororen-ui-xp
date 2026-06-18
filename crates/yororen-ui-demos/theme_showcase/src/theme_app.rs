//! yororen-ui Theme Showcase Demo
//!
//! One `headless::button` + two `headless::label`s, all rendered
//! by the same `TokenButtonRenderer` / `TokenLabelRenderer` but
//! reading from one of four different JSON themes. Each
//! `Render::render` installs the *current* theme into the global
//! slot, so a click on "Next theme" both increments the index
//! and triggers a re-render — and the whole window re-themes
//! because the renderers pull colors from the live global.
//!
//! The four themes:
//! - `themes/system-light.json` (default light — neutral palette)
//! - `themes/system-dark.json`  (default dark — neutral palette)
//! - `CATPPUCCIN` (inline — user-defined catppuccin mocha)
//! - `MATERIAL`   (inline — user-defined material rose)
//!
//! Note: the per-render `theme_mod::install` is the *whole point*
//! of the demo (live JSON swap). It is not the recommended
//! pattern for production code — apps should install their theme
//! once at boot and never touch it again.

use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window};
use yororen_ui::ActionVariantKind;
use yororen_ui::Theme;
use yororen_ui::headless::button::button;
use yororen_ui::headless::label::label;
use yororen_ui::headless::layout::{Inset, Spacing, column};
use yororen_ui::theme as theme_mod;

pub struct ThemeApp {
    themes: Vec<(&'static str, &'static str)>,
    current: usize,
    // Cache the last `Theme` we installed, keyed by index.
    // `Theme::from_json` parses + validates the full JSON on
    // every call; doing that 60 times per second for a
    // 4-theme cycle is wasteful. The cache lives on the
    // entity so it survives across re-renders.
    last_installed: Option<usize>,
    last_theme: Option<Theme>,
}

impl ThemeApp {
    pub fn new() -> Self {
        // Names are used for the label. JSON is loaded in
        // `current_theme()`.
        Self {
            themes: vec![
                ("system-light", "Default light — neutral palette"),
                ("system-dark", "Default dark — neutral palette"),
                ("catppuccin", "User-defined — catppuccin mocha"),
                ("material", "User-defined — material rose"),
            ],
            current: 0,
            last_installed: None,
            last_theme: None,
        }
    }

    fn current_theme(&self) -> Theme {
        let json = match self.current {
            0 => include_str!("../themes/system-light.json"),
            1 => include_str!("../themes/system-dark.json"),
            2 => CATPPUCCIN,
            _ => MATERIAL,
        };
        Theme::from_json(json).expect("valid JSON")
    }
}

/// `catppuccin-mocha`-flavoured user-defined theme. Inline
/// because this demo is the only place it lives; other apps
/// would drop it into `themes/`.
///
/// All 6 `action.primary.*` fields are present so the
/// `TokenButtonRenderer`'s `bg` / `fg` / `hover_bg` /
/// `active_bg` / `disabled_bg` / `disabled_fg` reads
/// resolve to real colors (otherwise `unwrap_or_default()`
/// returns a fully transparent `Hsla` and hover / active
/// states make the button disappear).
const CATPPUCCIN: &str = r##"{
  "action": {
    "primary": {
      "bg": "#89b4fa",
      "hover_bg": "#a3c2fb",
      "active_bg": "#74a0e0",
      "fg": "#1e1e2e",
      "disabled_bg": "#b8c8e3",
      "disabled_fg": "#5d5d6e"
    }
  },
  "surface": { "base": "#1e1e2e" },
  "content": { "primary": "#cdd6f4" }
}"##;

/// `material-rose`-flavoured user-defined theme. Inline for
/// the same reason as `CATPPUCCIN`. Same `action.primary.*`
/// completeness note as above.
const MATERIAL: &str = r##"{
  "action": {
    "primary": {
      "bg": "#c2185b",
      "hover_bg": "#d6296e",
      "active_bg": "#a81549",
      "fg": "#ffffff",
      "disabled_bg": "#d593aa",
      "disabled_fg": "#5e3a45"
    }
  },
  "surface": { "base": "#fffbfe" },
  "content": { "primary": "#3e001f" }
}"##;

impl Render for ThemeApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Install the current theme so every `default_render`
        // call below picks it up. This is the *core mechanism*
        // of the demo: clicking "Next theme" both advances the
        // index and triggers a re-render, so the whole window
        // re-themes.
        //
        // The full `Theme::from_json` is only called when
        // `self.current` actually changes — subsequent
        // re-renders reuse the cached `Theme` value.
        if self.last_installed != Some(self.current) {
            let theme = self.current_theme();
            theme_mod::install(cx, theme.clone());
            self.last_installed = Some(self.current);
            self.last_theme = Some(theme);
        }
        let theme = self
            .last_theme
            .as_ref()
            .expect("last_theme is set whenever last_installed is Some");

        // Read the theme's surface color so the root div paints
        // the intended background. Without this, the window
        // shows whatever the OS window manager decides to put
        // behind the div (usually the system window chrome
        // background), which often clashes with the theme.
        //
        // `surface.base` is required by every bundled theme;
        // the fallback is a near-white safety net.
        let surface = theme
            .get_color("surface.base")
            .unwrap_or_else(|| gpui::hsla(0.0, 0.0, 0.98, 1.0));

        let (name, blurb) = self.themes[self.current];
        let current = self.current;
        let total = self.themes.len();

        // Capture the `Entity<ThemeApp>` so the click handler
        // can mutate `self.current` and call `cx.notify()` to
        // trigger a re-render. `cx.entity()` is on
        // `Context<Self>` and returns an owned `Entity<Self>`
        // (cloning the internal `Arc` is cheap).
        let entity = cx.entity().clone();

        let next_btn = button("next-theme", cx)
            .variant(ActionVariantKind::Primary)
            .on_click(move |_, _, cx| {
                entity.update(cx, |app, cx| {
                    app.current = (app.current + 1) % app.themes.len();
                    cx.notify();
                });
            })
            .render(cx)
            .child(format!("Next theme → ({}/{})", current + 1, total));

        column("theme-root", cx)
            .w_full()
            .h_full()
            .p(Inset::Lg)
            .gap(Spacing::Md)
            .child(
                label(
                    "title",
                    format!("Theme showcase ({}/{})", current + 1, total),
                    cx,
                )
                .render(cx),
            )
            .child(label("blurb", format!("Currently: {} — {}", name, blurb), cx).render(cx))
            .child(next_btn)
            .render(cx)
            .bg(surface)
    }
}
