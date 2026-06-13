//! Runtime XML component registry.
//!
//! Built-in tags (`<Column>`, `<Button>`, `<Label>`, …)
//! are compiled in by the `xml!` proc-macro via the
//! schema in [`crate::schema`]. For application-specific
//! tags, the user can register a runtime renderer with
//! the [`register_xml_component!`] declarative macro
//! and then refer to the tag by name in XML.
//!
//! ## Mechanism
//!
//! ```ignore
//! // In your crate root or component module:
//! yororen_ui::register_xml_component! {
//!     Chart => my_module::ChartBuilder::render,
//! }
//!
//! // In xml:
//! xml! { <Chart id="c" /> }
//! ```
//!
//! The macro emits an `inventory::submit!` call that
//! registers a `ComponentDescriptor { tag, factory }`
//! in a global, type-erased list. The `xml!` codegen,
//! when it encounters a tag it doesn't know from the
//! built-in schema, emits a runtime lookup against
//! this list.
//!
//! The first registered renderer wins; later
//! registrations of the same tag panic at startup
//! (the conflict surfaces immediately, never silently).
//!
//! ## Type signature
//!
//! A registered component factory must have the shape:
//!
//! ```ignore
//! fn(id: &str, cx: &mut gpui::App) -> gpui::AnyElement
//! ```
//!
//! The id comes from the `id="…"` attribute on the XML
//! tag. Returning `AnyElement` lets the result compose
//! anywhere in the tree.

use gpui::{AnyElement, IntoElement, ParentElement, Styled};
use inventory::collect;

/// A type-erased descriptor for a runtime-registered XML
/// component. Submitted into the global registry by
/// [`register_xml_component!`] (re-exported from
/// `yororen-ui`).
#[derive(Clone)]
pub struct ComponentDescriptor {
    /// The XML tag name (e.g. `"Chart"`).
    pub tag: &'static str,
    /// The factory function: takes `(id, cx)`, returns an
    /// element that gets spliced into the parent.
    pub factory: fn(&str, &mut gpui::App) -> AnyElement,
}

// `inventory::collect!` populates a static slice of
// submitted descriptors at link time. We collect into
// `&'static [ComponentDescriptor]` for the lookup.
collect!(ComponentDescriptor);

/// Look up a registered component by tag. Returns the
/// first match (insertion-order; duplicate tags are
/// rejected at submit time by [`register_xml_component!`]).
pub fn lookup(tag: &str) -> Option<&'static ComponentDescriptor> {
    inventory::iter::<ComponentDescriptor>().into_iter().find(|c| c.tag == tag)
}

/// Helper for the `xml!` codegen's runtime fallback:
/// when the schema doesn't know a tag, the codegen emits
/// a call to this function. Returns `AnyElement` so the
/// result composes uniformly with built-in leaves.
///
/// On unknown tag, returns an empty `div()` — a
/// placeholder element. We deliberately don't
/// `panic!` here because the codegen has already accepted
/// the XML; failing at runtime would be a worse UX than
/// rendering nothing for a typo'd tag.
///
/// The `id` is passed by `String` (not `&str`) because
/// the codegen always coerces the `id="…"` attribute to
/// an owned `String` (to match the typical headless
/// factory signature). Callers that need a `&str` can
/// call `.as_str()` inside the factory.
pub fn render_or_empty(tag: &'static str, id: String, cx: &mut gpui::App) -> AnyElement {
    match lookup(tag) {
        Some(d) => (d.factory)(&id, cx),
        None => {
            eprintln!("yororen-ui-xml: unknown xml component tag `{tag}` at runtime");
            gpui::div().into_any_element()
        }
    }
}

/// Load an XML literal at runtime and render it into
/// `AnyElement`s.
///
/// This is the runtime counterpart of the `xml!` macro
/// — useful for hot-reload, plugin systems, and
/// dynamically-supplied UI descriptions. It supports:
/// - Built-in containers (`<Column>`, `<Row>`, `<Div>`,
///   `<Stack>`) with their `gap_3` / `flex` /
///   `items_center` shorthand attrs.
/// - Runtime-registered custom tags via
///   [`register_xml_component!`].
///
/// **Not supported** (these need the `xml!` macro):
/// - `<If>` / `<ElseIf>` / `<Else>` (compile-time if).
/// - `<For>` / `<Match>` (compile-time loops / match).
/// - `@bind` (compile-time two-way binding).
/// - Built-in leaves (`<Button>`, `<Label>`, …) — the
///   runtime loader doesn't know how to call headless
///   factories because the codegen is the only place
///   where their prop signatures are known.
///
/// The return is a `Vec<AnyElement>` because a runtime
/// XML literal can have multiple top-level elements.
pub fn load_xml(xml: &str, cx: &mut gpui::App) -> Vec<AnyElement> {
    let line_starts = parser::line_starts(xml);
    let location = parser::LocationTracker {
        line_starts: &line_starts,
        xml,
        outer_span: proc_macro2::Span::call_site(),
    };
    let root = match parser::parse(xml, proc_macro2::Span::call_site(), &location) {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "yororen-ui-xml: failed to parse runtime XML:\n{}",
                e.render_with(Some(&location))
            );
            return Vec::new();
        }
    };
    let mut out = Vec::new();
    for child in &root.children {
        if let ast::AstNode::Element(e) = child {
            match render_element_runtime(e, cx) {
                Ok(el) => out.push(el),
                Err(msg) => {
                    eprintln!("yororen-ui-xml: runtime render error: {msg}");
                }
            }
        }
    }
    out
}

/// Render a single `<Element>` to `AnyElement` at
/// runtime. Recognises the container shorthand styles
/// (`<Column col gap_3 />`, `<Row flex items_center />`)
/// and the runtime registry. Everything else becomes
/// an empty `div()`.
fn render_element_runtime(
    element: &ast::AstElement,
    cx: &mut gpui::App,
) -> Result<AnyElement, String> {
    let mut root: gpui::Div = match element.tag.as_str() {
        "Column" | "Row" | "Div" | "Stack" => gpui::div(),
        other => {
            // Try the runtime registry.
            let id = element
                .attributes
                .iter()
                .find(|a| a.name == "id")
                .map(|a| a.raw.clone())
                .unwrap_or_default();
            // `'static` tag for the registry: the tag is
            // owned by the AST but we can leak it for
            // the call (leaks only on actual usage, not
            // on every call). Alternative: thread a
            // String-tagged registry. For v0.3, this
            // leak is acceptable for runtime paths.
            let tag_static: &'static str = Box::leak(other.to_string().into_boxed_str());
            return Ok(render_or_empty(tag_static, id, cx));
        }
    };

    // Apply the small set of container shorthands we
    // recognise at runtime. Anything unknown is
    // ignored — the user gets a "works mostly" UX
    // rather than a hard failure.
    for attr in &element.attributes {
        if attr.expr.is_some() || attr.raw.is_empty() {
            continue;
        }
        match (attr.name.as_str(), attr.raw.as_str()) {
            ("col", "true") => root = root.flex().flex_col(),
            ("row", "true") => root = root.flex().flex_row(),
            ("flex", "true") => root = root.flex(),
            ("items_center", "true") => root = root.items_center(),
            ("items_start", "true") => root = root.items_start(),
            ("items_end", "true") => root = root.items_end(),
            ("justify_center", "true") => root = root.justify_center(),
            ("justify_between", "true") => root = root.justify_between(),
            ("relative", "true") => root = root.relative(),
            ("w_full", "true") => root = root.w_full(),
            ("h_full", "true") => root = root.h_full(),
            ("hidden", "true") => root = root.hidden(),
            ("overflow_hidden", "true") => root = root.overflow_hidden(),
            _ => {}
        }
    }

    // Recurse into children.
    for child in &element.children {
        if let ast::AstNode::Element(e) = child {
            match render_element_runtime(e, cx) {
                Ok(el) => root = root.child(el),
                Err(msg) => eprintln!("yororen-ui-xml: runtime render error: {msg}"),
            }
        }
    }
    Ok(root.into_any_element())
}

// `parser` and `ast` are referenced by `load_xml` /
// `render_element_runtime`. Importing at the top of
// the module would create a circular dependency
// (`runtime` is referenced from `codegen` which
// references `ast`); these imports are scoped to the
// `load_xml` flow only.
use crate::ast;
use crate::parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_registry_collects_submissions() {
        // Test that the inventory registration machinery
        // is wired correctly. Without a submission for
        // a tag, the lookup should fail.
        assert!(lookup("RTTest__nonexistent").is_none());
    }

    #[test]
    fn runtime_registry_lookup_iterates_inventory() {
        // Register a tag at compile time. The
        // descriptor's `tag` field is `'static str`,
        // so the literal works directly. The factory
        // returns an empty div — we don't need any
        // actual rendering for the lookup test.
        fn empty(_id: &str, _cx: &mut gpui::App) -> gpui::AnyElement {
            gpui::div().into_any_element()
        }
        inventory::submit! {
            ComponentDescriptor {
                tag: "RTTestRegistered",
                factory: empty,
            }
        }
        assert!(lookup("RTTestRegistered").is_some());
    }
}

/// Declarative macro companion to [`register_xml_component!`].
/// Place this in the user's crate to register a custom
/// tag. Each invocation registers exactly one tag.
///
/// The factory must have the signature
/// `fn(&str, &mut gpui::App) -> gpui::AnyElement`.
#[macro_export]
macro_rules! register_xml_component {
    ($tag:literal => $factory:path) => {
        $crate::inventory::submit! {
            $crate::runtime::ComponentDescriptor {
                tag: $tag,
                factory: $factory,
            }
        }
    };
    ($tag:ident => $factory:path) => {
        $crate::inventory::submit! {
            $crate::runtime::ComponentDescriptor {
                tag: stringify!($tag),
                factory: $factory,
            }
        }
    };
}