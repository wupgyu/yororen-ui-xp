# Overlay z-order — paint priority

Overlays (popovers, dropdowns, modals, toasts) do not auto-float on
the z-axis in gpui. `.absolute()` only takes an element out of the
layout flow; paint order is still DOM order. The framework's solution
is **`gpui::deferred(child).with_priority(N)`** — the element is
painted in priority order, with higher priority painted last (on top).

## The priority table

| Priority | Layer | Examples |
|---|---|---|
| 0 | scroll content | buttons, inputs, list rows, anything in the main scroll |
| 1 | floating UI attached to a trigger | popover, dropdown_menu, combo_box menu, tooltip, context menu, select dropdown |
| 2 | modal scrim + content | modal |
| 3 | notification host | toast stack |

This table is what the gallery demo uses. Follow it unless you have
a specific reason to deviate.

## Pattern: modal scrim + content

The modal is rendered at the scroll-root level (sibling to the main
content), wrapped in a full-viewport scrim, and prioritized above
popovers:

```rust
let modal_panel = modal("ov-modal", app.modal_state.clone())
    .child(/* title */)
    .child(/* body */)
    .child(/* footer with close button */)
    .render(cx)
    .w(px(360.));

let scrim_with_modal = div()
    .absolute().inset_0()
    .flex().items_center().justify_center()
    .bg(hsla(0.0, 0.0, 0.0, 0.55))    // 55% black scrim
    .child(modal_panel);

let modal_deferred = gpui::deferred(scrim_with_modal).with_priority(2);
```

Then in the scroll root:

```rust
div().id("scroll-root").relative().size_full()
    .child(/* main content */)
    .child(modal_deferred)
    .child(notifications_deferred)        // priority 3
```

## Pattern: popover / dropdown

Popovers, dropdown menus, select/combo box panels, and tooltips all
share the same pattern: render them as a child of the trigger's
container, deferred with priority 1.

```rust
let popover_el = popover("user-menu", app.popover_state.clone())
    .trigger(button("user-btn", cx).on_click(...).render(cx))
    .content(menu_el)
    .render(cx);

// Wrap so it paints above the page content but below the modal:
let popover_deferred = gpui::deferred(popover_el).with_priority(1);

div().relative()                              // .relative() gives the popover a containing block
    .child(button_el)
    .child(popover_deferred)
```

Two things to verify:

1. The trigger's container has `.relative()` — the popover positions
   itself relative to the trigger's bounds.
2. The popover element is a child of the trigger's container, not of
   some other scroll content area. If the popover's container is
   clipped (`overflow: hidden`), the popover will be visually clipped
   too.

## Pattern: notification host

The notification host is a full-viewport overlay that anchors
toasts in one corner (top-right by default) and is priority 3 so it
floats above modals.

```rust
use yororen_ui::notification::center::NotificationCenter;

fn render_toast_host(cx: &mut Context<MyApp>) -> impl IntoElement {
    let center = cx.global::<NotificationCenter>();
    let items = center.items();
    let mut stack = div().absolute().top_0().right_0().flex().flex_col().gap(px(8.));
    for n in items {
        let card = /* paint one card with kind-colored bg, close button, etc. */;
        // Optional: animate the enter with .with_animation
        stack = stack.child(card);
    }
    stack
}

let toast_deferred = gpui::deferred(render_toast_host(cx)).with_priority(3);
```

The `NotificationCenter` itself does not render anything. Your app
owns the host element, registers the window with the center (so
auto-dismiss timers can target it), and inserts toasts via
`center.notify(Notification::new("..."), cx)`.

## Why `deferred`, not just `.absolute()`

`gpui::deferred(child)` defers the child's prepaint and paint until
the end of the parent's prepaint, after all non-deferred children
have been laid out. Combined with `.with_priority(N)`, this gives you
deterministic paint order regardless of how nested the overlay is.

If you skip the `deferred` wrap and just use `.absolute()`, the
overlay paints in DOM order — usually fine for the bottom-most
overlay (page content has no overlays below it), but immediately
broken for any case where two overlays could overlap.

## What about nested modals?

Two modals open at once (a settings modal opening a "are you sure?"
confirmation) need to:

1. Each be its own `priority(2)` overlay (modals don't need
   priority 4 — only toasts do).
2. Acquire a `ScrollLockGuard` (see `$yororen-ui-app-core` § 8) so
   the underlying page can't scroll while the user is interacting
   with the inner modal.
3. Set `modal_state.set_initial_focus(handle)` so the inner modal's
   first focusable element is the right one.

## When to skip the priority wrap

Single-overlay pages (a dropdown menu in a settings screen, no
modal, no toasts) don't need the wrap. The dropdown menu is the
only overlay; it sits on top of page content because it's the only
absolute-positioned element. Only add the wrap when you have two or
more overlays that could overlap.

## Reference implementations

- `crates/yororen-ui-demos/gallery_demo/src/gallery_app.rs` —
  scroll root with section list, modal (priority 2), and toast
  host (priority 3) all as siblings.
- `crates/yororen-ui-demos/gallery_demo/src/notifications_host.rs` —
  the `deferred_host` function that wraps the toast host.
- `crates/yororen-ui-demos/gallery_demo/src/sections/overlays.rs` —
  popover + menu + dropdown inside a section, with their own
  `priority(1)` wrap.
