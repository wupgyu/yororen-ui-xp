# v0.2 → v0.3 Migration Guide

v0.3 is a **breaking release** that ships the headless-core + preset-theme split.
This guide covers every change that affects existing user code.

> TL;DR — Most users need only one change: depend on the new meta-crate
> `yororen-ui` (which re-exports `yororen-ui-core` + `yororen-ui-theme-system`)
> and call `yororen_ui::theme_system::install(cx, appearance)` once during
> app bootstrap.

---

## 1. Workspace structure change

| Before (v0.2) | After (v0.3) |
|---|---|
| `yororen_ui` (single crate) | `yororen-ui-core` (headless) + `yororen-ui-theme-system` (default theme) + `yororen-ui` (meta-crate) |

If you depend on `yororen_ui = "0.2"` you can keep using the meta-crate
`yororen_ui = "0.3"` for an easy migration. If you want full control over
theming, depend on `yororen-ui-core` and `yororen-ui-theme-system` directly.

```toml
# Cargo.toml — easy migration (recommended)
[dependencies]
yororen-ui = "0.3"
yororen-ui-theme-system = "0.3"   # optional, only if you use the preset theme

# Cargo.toml — full control
[dependencies]
yororen-ui-core = "0.3"
# your-own-theme = "0.1"          # implement the Theme shape from core
```

---

## 2. Install the theme

**Before (v0.2):**
```rust
use yororen_ui::theme::{GlobalTheme, Theme};

cx.set_global(GlobalTheme::new(cx.window_appearance()));
```

**After (v0.3):**
```rust
use yororen_ui::theme_system;

theme_system::install(cx, cx.window_appearance());
```

The default light/dark palette is unchanged, so visuals stay identical.

If you build your own theme, depend on `yororen-ui-core` only and set
`GlobalTheme::new_with_themes(appearance, ThemeSet::new(light).dark(dark))`
yourself.

---

## 3. Removed business icons

`IconName` no longer carries the 5 app-specific variants that shipped in
v0.2. They were tied to a specific app's domain and don't belong in a
general-purpose UI library.

| Removed `IconName` variant | Migration |
|---|---|
| `Microsoft` | Use `IconPath::External("icons/microsoft.svg")` |
| `Minecraft` | Use `IconPath::External("icons/minecraft.svg")` |
| `Modpack` | Use `IconPath::External("icons/modpack.svg")` |
| `Server` | Use `IconPath::External("icons/server.svg")` |
| `PingIndicator(usize)` | Use `IconPath::External("icons/ping-N.svg")` with N = 0..=3 |

The 8 corresponding SVG files were also removed from the embedded
`assets/icons/` folder. The 13 universal icons remain
(`Search, Arrow, Check, Warning, Info, Close, Maximize, Minimize, User,
Pencil, Trash, File, Folder`).

```rust
// Before
icon(IconName::Minecraft)

// After
icon(IconPath::External("icons/minecraft.svg"))
icon(IconPath::External("icons/your-icon.svg"))   // app-specific
```

---

## 4. Placeholder strings are no longer auto-localized

The `i18n::defaults::DefaultPlaceholders` 9-language hardcoded table is
**deprecated** in v0.3 and will be **removed in v0.4**. Components
(`Select`, `ComboBox`, `FilePathInput`, `KeybindingInput`) already expose
a `.placeholder(...)` builder — pass an explicit string (or your own
localized lookup) instead of relying on the auto-magic.

```rust
// Before
select("my-select").localized(true)  // shows "Select…" in en, "请选择…" in zh, ...

// After
select("my-select").placeholder("Select…")  // whatever you want, in any language
// Or, when using a yororen-ui-locale-* package:
select("my-select").placeholder(cx.i18n().t("ui.select.placeholder"))
```

For backward compatibility the auto-magic still works in v0.3 but emits a
`#[deprecated]` warning. Fix the warnings now and you're ready for v0.4.

---

## 5. Cargo workspace paths (informational)

If you use `path = "..."` in your own crates' `Cargo.toml`:

```toml
# v0.2
yororen_ui = { path = "../yororen-ui" }

# v0.3 (same, but the meta-crate moved inside `crates/`)
yororen-ui = { path = "../yororen-ui/crates/yororen-ui" }
yororen-ui-core = { path = "../yororen-ui/crates/yororen-ui-core" }
yororen-ui-theme-system = { path = "../yororen-ui/crates/yororen-ui-theme-system" }
```

For published use just bump the version:

```toml
yororen-ui = "0.3"
```

---

## 6. New design tokens (optional, no action required)

v0.3 added `Theme::tokens: DesignTokens` for the headless components to
read sizing / motion / typography from. **All defaults are pinned to the
v0.2 pixel values**, so existing apps see no visual change. If you want
to override (e.g. for a compact mode), modify `theme.tokens` in your
custom theme's constructor — no component code changes needed.

```rust
let mut theme = theme_system::dark();
theme.tokens.control.switch.thumb_size = px(18.);   // example: larger thumb
cx.set_global(GlobalTheme::new_with_themes(appearance, ThemeSet::new(light).dark(theme)));
```

Run `yororen_ui_core::theme::validate(&theme)` in CI to catch low-contrast
text and out-of-range tokens before shipping your theme.

---

## 7. Summary checklist

- [ ] Bump `yororen-ui` to `0.3` in your `Cargo.toml`
- [ ] Replace `GlobalTheme::new(...)` with `yororen_ui::theme_system::install(cx, appearance)` (or your own custom install)
- [ ] Remove the 5 business `IconName` usages and switch to `IconPath::External`
- [ ] Pass `.placeholder(...)` explicitly on `Select` / `ComboBox` / `FilePathInput` / `KeybindingInput` (clear the `#[deprecated]` warning)
- [ ] (Optional) Drop the `yororen-ui` meta-crate dep and depend on `yororen-ui-core` + your own theme package
- [ ] (Optional) Tweak `theme.tokens` for compact/dense modes
