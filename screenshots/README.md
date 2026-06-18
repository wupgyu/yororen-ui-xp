# Screenshots

These PNGs are referenced from the top-level `README.md` and `README_zh_CN.md`
under each demo section and the "Built with Yororen UI" section.

Source apps live under `crates/yororen-ui-demos/`:

- `counter.png` ← `crates/yororen-ui-demos/counter`
- `layers-demo.png` ← `crates/yororen-ui-demos/layers_demo`
- `inputs-demo.png` ← `crates/yororen-ui-demos/inputs_demo`
- `gallery-demo.png` ← `crates/yororen-ui-demos/gallery_demo`
- `theme-showcase.png` ← `crates/yororen-ui-demos/theme_showcase`
- `accelerator-{1,2,3,4}.png` ← Yororen Accelerator (external project)

To regenerate, run each demo and capture a screenshot into this folder with
the matching filename.

> Historical note: this directory was previously named `demo/screenshots/`.
> The bare `demo/` directory contained only this `screenshots/` subdirectory
> and was easy to confuse with `crates/yororen-ui-demos/` (where the actual
> demo source lives). Flattening to `screenshots/` makes the intent — "this
> is just docs/screenshots, not a demo workspace" — explicit.