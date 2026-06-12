# Input patterns — v0.3

Yororen UI inputs are **uncontrolled by design**. The renderer owns
the interaction state (caret, selection, scroll, blink, IME) and your
app owns the business state. The `on_change` callback is the only
bridge between them. This is what eliminates the render-driven
feedback loop that controlled inputs (`.content(state_value)` +
`on_change` writing back) used to cause.

This file covers the v0.3 patterns for the few situations where the
"the input owns its state" model meets a real app need.

## 1. The mental model in one diagram

```
   ┌─────────────────────────────────────────────┐
   │  text_input("email")                        │
   │    └── Entity<TextInputState> (keyed state) │
   │         ├── value: String                   │   owned by the
   │         ├── caret, selection, scroll        │   framework, lives
   │         ├── last_layout, last_bounds        │   across renders,
   │         └── cursor_blink_epoch              │   shared by all
   │                                             │   inputs that
   └─────────────────────────────────────────────┘   use the same id
                  │
                  │  on_change(new, w, cx)
                  ▼
   ┌─────────────────────────────────────────────┐
   │  your app's state                           │
   │    └── Entity<MyForm>                       │   owned by you,
   │         ├── email: String                   │   notified via
   │         ├── email_error: Option<String>     │   cx.notify()
   │         └── submit_count: u32               │
   └─────────────────────────────────────────────┘
```

The contract: **the input is the only place the user can change the
text** (keypress, paste, IME). It calls your `on_change` exactly once
per change. You store the new value. The next render reads your value
for display purposes (e.g. a status line "email: alice@…") — never to
re-seed the input.

## 2. "Open edit modal" — the fresh-id pattern

The classic need: open a modal pre-populated with the row's current
value, let the user edit, submit. In a controlled model you'd call
`set_content(value)` once and trust the flag. In v0.3 the input is
encapsulated, so the pattern is to mount a **fresh input** with a
**fresh id** that encodes "this is a new edit session":

```rust
// In your app state:
pub struct MyApp {
    pub editing: Option<Entity<EditingSession>>,
    // ...
}

pub struct EditingSession {
    pub edit_id: u64,                  // bumped per "Open edit" click
    pub draft_email: String,
    pub draft_email_error: Option<String>,
}

// Click handler for "Edit row":
app.update(cx, |s, cx| {
    let row = s.selected_row();
    let session = cx.new(|_| EditingSession {
        edit_id: s.next_edit_id,        // unique per open
        draft_email: row.email.clone(),
        draft_email_error: None,
    });
    s.editing = Some(session);
    cx.notify();
});

// Render the modal:
if let Some(session_entity) = app.editing.clone() {
    let session = session_entity.read(cx);
    let modal = modal("edit", app.edit_modal_state.clone())
        .child(form_field("edit-email", "email", cx)
            .label("Email")
            .input({
                let entity = session_entity.clone();
                let id = format!("edit-email-{}", session.edit_id);   // ← unique id
                text_input(id)
                    .placeholder("you@example.com")
                    .on_change(move |new, _w, cx| {
                        entity.update(cx, |s, _cx| s.draft_email = new.to_string());
                    })
                    .render(cx, window)
            })
            .render(cx))
        .render(cx);
}
```

Why this works:

- `window.use_keyed_state` keys the input's `Entity<TextInputState>` by
  its `id`. A new `id` ⇒ a brand new state entity ⇒ empty input.
- The id encodes the "session" — close the modal, bump the id, reopen
  the modal ⇒ fresh input. Close + reopen with the same id ⇒ old
  text reappears (useful for "discard changes" behavior).
- The `on_change` closure still uses `cx.entity().clone()` to write
  into the session. The session and the input are decoupled; the
  input can be unmounted and remounted freely.

## 3. Why not a `.set_value(value)` method?

The input does not expose one, deliberately. The reasons are encoded
in the v0.3 architecture:

- The renderer's `use_keyed_state` makes the input a "self-contained
  island" of state. Adding a `set_value` would force the renderer to
  thread a writer handle into the input's keyed state, which then has
  to play nicely with the user's caret position and selection. The
  "fresh id" pattern sidesteps all of that.
- Two callers writing the value from two sources (your `on_change`
  and a programmatic reset) would race. The `on_change`-only model
  has a single writer.
- Most "I need to set the value" requests are actually "I need to
  start a new edit session" — which the fresh-id pattern handles.

If you genuinely need a reset button (a search input with a clear-X
button), there are two options:

- **Use the search_input's built-in `on_clear` hook.** The renderer
  already clears the value on Escape and on click of the built-in
  clear button; `on_clear` lets you run extra work (e.g. log, close
  a sibling dropdown). You don't need a public reset method because
  the input already does it.
- **For a custom reset**, change the id. E.g. an "Add row" button
  increments `next_input_id` and remounts the input with
  `format!("new-row-{next_input_id}")` — the previous value is
  discarded by the new keyed state.

## 4. Common input pitfalls

| Symptom | Cause | Fix |
|---|---|---|
| First keystroke is dropped | `text_input::init(cx)` not called | Call it once at boot, before opening windows with text inputs |
| Text input doesn't appear in the layout | Calling `text_input(id).render(cx, window)` from a context that returns `Div`, not `AnyElement` | Inputs return `AnyElement`; the wrapping `div().child(...)` accepts that |
| `on_change` panics with "entity not found" | The `move` closure captured `cx` itself, not `cx.entity()` | Capture `let entity = cx.entity();` first, then `move |…, cx| { entity.update(cx, …) }` |
| `cx.entity()` is `None` | Calling from a `&mut App` context, not a `Context<MyApp>` | Inputs are only used inside `Render::render`, which gives you `Context<MyApp>` |
| Caret blinks but text doesn't appear | Theme is missing `content.primary` | Verify `cx.theme().get_color("content.primary")` is `Some(_)` |
| IME doesn't show on macOS / Windows | `text_input::init(cx)` not called | Same fix as the first row |
| Modal opens but Escape doesn't close it | The modal is rendered inside a scrollable container that swallows the key | Render the modal at the **scroll-root** level as a sibling, wrapped in `gpui::deferred(...).with_priority(2)` |
| `pick(value, window, cx)` won't compile on `cx` | Used `&mut Context<MyApp>` instead of `&mut App` | Coerce inline: `state.update(cx, \|s, cx_inner\| s.pick(value, window, &mut *cx_inner))` |
| Composite state resets every render | `cx.new(\|_\| SelectState::default())` is called inside the render closure | Mint the entity once in `MyApp::new(cx)` and store it in the app state |

## 5. Validation pattern

`on_change` is the place to do live validation, but be cheap:

```rust
text_input("email")
    .on_change({
        let entity = cx.entity();
        move |new: &str, _w, cx| {
            entity.update(cx, |s, _cx| {
                s.email = new.to_string();
                // Cheap synchronous validation only.
                s.email_error = if new.contains('@') { None } else { Some("must contain @".into()) };
            });
        }
    })
    .render(cx, window)
```

Expensive validation (network calls, file system) belongs in `on_submit`,
not `on_change`. The input fires `on_change` for every keystroke; the
form fires `on_submit` once per user submission.

## 6. The shared `TextInputCore` — what it means for you

All seven text inputs (and `combo_box`'s embedded input) share
`TextInputCore` from `yororen-ui-core/src/headless/text_input_core.rs`.
This is why a fix to "backspace across line boundaries" in
`text_input` also fixes it in `combo_box`, why a `keybinding_input`
clears via Escape, etc.

You do not interact with `TextInputCore` directly — it's an internal
struct that the renderer's painter reads. The only thing you need to
know is that **the 15 keymap actions bound by `init(cx)` apply
uniformly to every text-bearing input in your app**. If you want a
different keymap, don't call `init(cx)` and wire your own actions.

## 7. Related references

- `SKILL.md §3` — the seven text inputs and their builder extras
- `SKILL.md §4` — stateful composites (select, modal, popover, etc.)
- `SKILL.md §5` — form + form_field
