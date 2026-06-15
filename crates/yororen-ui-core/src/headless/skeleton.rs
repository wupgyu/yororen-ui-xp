//! Headless `skeleton` — placeholder shape. No state.

use std::sync::Arc;

use gpui::{Div, ElementId, InteractiveElement, Pixels, Stateful};

#[derive(Clone, Debug)]
pub struct SkeletonProps {
    pub id: ElementId,
    /// `true` → block (filled rect); `false` → single line.
    pub block: bool,
    /// Only meaningful when `block == true`. `true` → square
    /// corners; `false` → rounded.
    pub block_sharp: bool,
    /// Explicit width. When `None`, the skeleton sizes to its
    /// parent or content as usual.
    pub w: Option<Pixels>,
    /// Explicit height. For block skeletons this replaces the
    /// default `h_full()`; for line skeletons it replaces the
    /// default `min_h()`.
    pub h: Option<Pixels>,
}

pub fn skeleton(id: impl Into<ElementId>, _cx: &mut gpui::App) -> SkeletonProps {
    SkeletonProps {
        id: id.into(),
        block: false,
        block_sharp: false,
        w: None,
        h: None,
    }
}

impl SkeletonProps {
    pub fn block(mut self, v: bool) -> Self {
        self.block = v;
        self
    }
    pub fn block_sharp(mut self, v: bool) -> Self {
        self.block_sharp = v;
        self
    }
    pub fn w(mut self, w: impl Into<Pixels>) -> Self {
        self.w = Some(w.into());
        self
    }
    pub fn h(mut self, h: impl Into<Pixels>) -> Self {
        self.h = Some(h.into());
        self
    }
    pub fn apply(self, el: Div) -> Stateful<Div> {
        // `apply()` is the headless path: the caller owns the
        // visual `Div`, so we only attach the element id. Width /
        // height live in the prop bag for renderers to consume; they
        // are *not* applied here, keeping headless free of styling.
        el.id(self.id)
    }

    /// Render the skeleton using the registered `SkeletonRenderer`.
    /// Returns a `Div`; caller chains `.w(...)` / `.h(...)` for
    /// explicit sizing.
    pub fn render(self, cx: &gpui::App) -> Div {
        use crate::renderer::RendererContext;
        use crate::renderer::markers::Skeleton as SkeletonMarker;
        use crate::renderer::skeleton::SkeletonRenderer;

        let r: &Arc<dyn SkeletonRenderer> = cx
            .renderer_arc::<SkeletonMarker, dyn SkeletonRenderer>()
            .expect("SkeletonRenderer registered");
        r.compose(&self, cx)
    }
}
