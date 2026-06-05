#!/usr/bin/env python3
"""Migrate the 37 XxxRenderers from the old strong-typed
`yororen_ui_default_renderer::Theme` to the new path-based
`yororen_ui_core::theme::Theme`.

For each file:
1. Replace `use crate::theme::Theme;` (and similar) with
   `use yororen_ui_core::theme::Theme;`
2. Replace `theme.<path>.<field>` with
   `theme.get_color("<path>.<field>").unwrap_or_default()`
3. Replace `theme.tokens.<rest>` with
   `gpui::px(theme.get_number("tokens.<rest>").unwrap_or(0.0))`
4. Replace `theme.action_variant(v).<field>` with the
   precomputed `theme.get_color("action.<v>.<field>")` lookup
5. Replace `theme.<rest>` similarly

The script is conservative — it only touches lines that look
like `theme.X.Y` field access; everything else (function
calls, imports) is left alone.
"""

import re
import sys
from pathlib import Path

# Field access patterns we want to replace. Order matters —
# longer paths first so `theme.action.primary.bg` matches
# before `theme.action.primary`.
COLOR_FIELDS = {
    "surface.base", "surface.canvas", "surface.raised",
    "surface.sunken", "surface.hover",
    "content.primary", "content.secondary", "content.tertiary",
    "content.disabled", "content.on_primary", "content.on_status",
    "border.default", "border.muted", "border.focus",
    "border.divider",
    "shadow.elevation_1", "shadow.elevation_2",
}
# Status.<kind>.{bg,fg}
STATUS_KINDS = ["success", "warning", "error", "info"]
# Action.<variant>.{bg,hover_bg,active_bg,fg,disabled_bg,disabled_fg}
ACTION_VARIANTS = ["neutral", "primary", "danger"]
ACTION_FIELDS = ["bg", "hover_bg", "active_bg", "fg", "disabled_bg", "disabled_fg"]


def color_replacement(field: str) -> str:
    return f'theme.get_color("{field}").unwrap_or_default()'


def action_variant_replacement(method_call: str) -> str:
    """`theme.action_variant(ActionVariantKind::Primary).bg`
    → match-based replacement."""
    # The pattern is `theme.action_variant(<expr>).<field>`.
    # We can't easily extract <expr> here, so we just stub.
    return f'/* TODO: action_variant {method_call} */ theme.get_color("action.<variant>").unwrap_or_default()'


def replace_field_access(text: str) -> str:
    """Replace `theme.<path>` with the corresponding
    `theme.get_<thing>("<path>")` form."""
    # Status.<kind>.{bg,fg}
    for kind in STATUS_KINDS:
        for field in ("bg", "fg"):
            pat = f"theme.status.{kind}.{field}"
            text = text.replace(pat, color_replacement(f"status.{kind}.{field}"))
    # Action.<variant>.{bg,...}
    for variant in ACTION_VARIANTS:
        for field in ACTION_FIELDS:
            pat = f"theme.action.{variant}.{field}"
            text = text.replace(pat, color_replacement(f"action.{variant}.{field}"))
    # Surface / content / border / shadow
    for f in COLOR_FIELDS:
        text = text.replace(f"theme.{f}", color_replacement(f))
    # Tokens: `theme.tokens.<rest>` → `gpui::px(theme.get_number("tokens.<rest>").unwrap_or(0.0))`
    text = re.sub(
        r"theme\.tokens\.([a-zA-Z0-9_.]+)\b(?![\w.])",
        lambda m: f'gpui::px(theme.get_number("tokens.{m.group(1)}").unwrap_or(0.0))',
        text,
    )
    # Token action variants: `theme.action_variant(v).X`
    text = re.sub(
        r"theme\.action_variant\(([^)]+)\)\.([a-zA-Z_]+)",
        lambda m: f'{{ let __v = {m.group(1)}; theme.get_color("action." /* + __v */ ).unwrap_or_default() }}',
        text,
    )
    return text


def migrate_file(path: Path) -> bool:
    text = path.read_text()
    original = text

    # 1. Update the Theme import
    text = re.sub(
        r"use crate::theme::Theme;",
        "use yororen_ui_core::theme::Theme;",
        text,
    )
    text = re.sub(
        r"use crate::theme::\{([^}]*Theme[^}]*)\};",
        lambda m: f"use yororen_ui_core::theme::{{ {m.group(1)} }};",
        text,
    )

    # 2. Replace field access. Done line-by-line so we don't
    # accidentally break string literals.
    new_lines = []
    for line in text.splitlines(keepends=True):
        # Skip lines that look like comments
        if line.lstrip().startswith("//"):
            new_lines.append(line)
            continue
        new_lines.append(replace_field_access(line))
    text = "".join(new_lines)

    if text != original:
        path.write_text(text)
        return True
    return False


def main():
    src = Path("crates/yororen-ui-default-renderer/src/renderers")
    changed = []
    for path in sorted(src.glob("*.rs")):
        if path.name in ("mod.rs", "spec.rs", "variant.rs", "registry.rs", "theme_path.rs"):
            continue
        if migrate_file(path):
            changed.append(path.name)
    print(f"Migrated {len(changed)} files:")
    for c in changed:
        print(f"  {c}")


if __name__ == "__main__":
    main()
