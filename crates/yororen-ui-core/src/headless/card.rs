//! Headless `card` — generic content surface. No state.

use std::sync::Arc;

use gpui::{AnyElement, Div, ElementId, InteractiveElement, IntoElement, Stateful};

/// Visual appearance of a card surface.
///
/// Renderers may map variants to different paints. Unknown variants
/// should fall back to the default surface style.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CardAppearance {
    /// Standard raised / dialog card.
    #[default]
    Default,
    /// Windows Explorer left-pane task card (header + body bands).
    ExplorerTask,
}

pub struct CardProps {
    pub id: ElementId,
    pub interactive: bool,
    /// `true` if the caller supplied a custom background color
    /// (consumed by `CardRenderer.has_custom_bg`).
    pub has_custom_bg: bool,
    /// Optional title painted by the renderer (e.g. Explorer task
    /// card header). When `None`, the renderer paints only the shell.
    pub title: Option<String>,
    /// Optional trailing header element (icon / chevron). Consumed
    /// by the renderer when present.
    pub header_trailing: Option<AnyElement>,
    /// Appearance variant. Defaults to [`CardAppearance::Default`].
    pub appearance: CardAppearance,
}

pub fn card(id: impl Into<ElementId>, _cx: &mut gpui::App) -> CardProps {
    CardProps {
        id: id.into(),
        interactive: false,
        has_custom_bg: false,
        title: None,
        header_trailing: None,
        appearance: CardAppearance::Default,
    }
}

impl CardProps {
    pub fn interactive(mut self, v: bool) -> Self {
        self.interactive = v;
        self
    }
    pub fn has_custom_bg(mut self, v: bool) -> Self {
        self.has_custom_bg = v;
        self
    }
    /// Set an optional title. Renderers that paint a header (e.g.
    /// Explorer task cards) consume this; default cards may ignore it.
    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }
    /// Optional trailing content in the card header (icon, chevron).
    pub fn header_trailing(mut self, el: impl IntoElement) -> Self {
        self.header_trailing = Some(el.into_element().into_any_element());
        self
    }
    /// Select a card appearance variant.
    pub fn appearance(mut self, appearance: CardAppearance) -> Self {
        self.appearance = appearance;
        self
    }
    pub fn apply(self, el: Div) -> Stateful<Div> {
        el.id(self.id)
    }

    /// Render the card using the registered `CardRenderer`. Returns
    /// a `Stateful<Div>` with the element id and the renderer-built
    /// bg / border / padding / radius. The caller still chains
    /// `.child(...)` to add content.
    pub fn render(mut self, cx: &gpui::App) -> Stateful<Div> {
        use crate::renderer::RendererContext;
        use crate::renderer::card::CardRenderer;
        use crate::renderer::markers::Card as CardMarker;

        let r: &Arc<dyn CardRenderer> = cx
            .renderer_arc::<CardMarker, dyn CardRenderer>()
            .expect("CardRenderer registered");
        let div = r.compose(&mut self, cx);
        self.apply(div)
    }
}
