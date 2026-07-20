//! Windows XP Notepad scene.

use gpui::{
    Context, Div, InteractiveElement, IntoElement, ParentElement, Render, Stateful,
    StatefulInteractiveElement, Styled, Window, div, px,
};
use yororen_ui::headless::dropdown_menu::{DropdownItem, DropdownMenuState, dropdown_menu};
use yororen_ui::headless::image::{ImageSource, image};
use yororen_ui::headless::menu::{MenuState, menu};
use yororen_ui::headless::text_area::text_area;
use yororen_ui::theme::ActiveTheme;
use yororen_ui::xp_renderer::XpAppWindow;

use crate::menu_data;

/// Startup sample text shown in the editor (File → New remounts empty).
const SAMPLE_TEXT: &str =
    "The quick brown fox jumps over the lazy dog.\n\nThis is a Windows XP Notepad visual acceptance demo for yororen-ui.";

/// (id, label, items) triple driving the five top menus.
type MenuSpec = (&'static str, &'static str, fn() -> Vec<DropdownItem>);

struct MenuPair {
    dropdown: gpui::Entity<DropdownMenuState>,
    menu: gpui::Entity<MenuState>,
    label: &'static str,
    id: &'static str,
}

pub struct NotepadApp {
    chrome: XpAppWindow,
    menus: Vec<MenuPair>,
    /// Bumped on File → New so the text_area remounts with a fresh empty state.
    editor_generation: u64,
}

impl NotepadApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let chrome = XpAppWindow::new(cx, "Untitled - Notepad");
        let app_entity = cx.entity().clone();

        let specs: [MenuSpec; 5] = [
            ("file", "File", menu_data::file_items),
            ("edit", "Edit", menu_data::edit_items),
            ("format", "Format", menu_data::format_items),
            ("view", "View", menu_data::view_items),
            ("help", "Help", menu_data::help_items),
        ];

        let mut menus = Vec::new();
        for (id, label, items_fn) in specs {
            let dropdown = DropdownMenuState::new(cx);
            let menu_state = MenuState::new(cx);
            let items = items_fn();
            let dropdown_for_select = dropdown.clone();
            let app_entity = app_entity.clone();
            menu_state.update(cx, |s, _| {
                s.set_items(items);
                s.set_on_select(move |item_id, window, cx| {
                    match item_id.as_ref() {
                        "file.exit" => {
                            window.remove_window();
                        }
                        "file.new" => {
                            app_entity.update(cx, |app, cx| {
                                app.editor_generation = app.editor_generation.wrapping_add(1);
                                cx.notify();
                            });
                        }
                        _ => {}
                    }
                    dropdown_for_select.update(cx, |st, _| st.close());
                });
            });
            menus.push(MenuPair {
                dropdown,
                menu: menu_state,
                label,
                id,
            });
        }

        Self {
            chrome,
            menus,
            editor_generation: 0,
        }
    }

    fn toolbar_border(&self, cx: &mut Context<Self>) -> gpui::Hsla {
        cx.theme()
            .get_color("xp.explorer.toolbar_border")
            .unwrap_or_else(|| gpui::hsla(0., 0., 0., 0.1))
    }

    fn icon(
        &self,
        id: impl Into<gpui::ElementId>,
        path: &str,
        size: f32,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        image(
            id,
            ImageSource::Resource(format!("windowsIcons/{path}").into()),
            cx,
        )
        .render(cx)
        .w(px(size))
        .h(px(size))
    }

    fn menu_bar(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let mut bar = div()
            .id("menu-bar")
            .flex()
            .flex_row()
            .items_center()
            .h(px(24.))
            .px(px(2.))
            .bg(cx.theme().get_color("surface.base").unwrap_or_default())
            .border_b_1()
            .border_color(self.toolbar_border(cx));

        for m in &self.menus {
            let dd = m.dropdown.clone();
            let trigger_id = format!("menu-trigger-{}", m.id);
            let trigger = div()
                .id(trigger_id)
                .px(px(6.))
                .py(px(2.))
                .text_size(px(11.))
                .cursor_pointer()
                .hover(|s| {
                    s.bg(cx
                        .theme()
                        .get_color("xp.selection.hover_bg")
                        .unwrap_or_default())
                })
                .on_click({
                    let dd = dd.clone();
                    move |_, _, cx| {
                        dd.update(cx, |st, _| st.toggle());
                    }
                })
                .child(m.label);

            let body = menu(format!("menu-body-{}", m.id), m.menu.clone()).render(cx);
            let content = if m.dropdown.read(cx).is_visible() {
                body.into_any_element()
            } else {
                div().into_any_element()
            };

            let el = dropdown_menu(format!("menu-dd-{}", m.id), m.dropdown.clone())
                .trigger(trigger.into_any_element())
                .content(content)
                .render(cx);
            bar = bar.child(el);
        }

        bar
    }

    fn editor(&mut self, window: &mut Window, cx: &mut Context<Self>) -> gpui::AnyElement {
        let generation = self.editor_generation;
        let editor_id = format!("notepad-editor-{generation}");
        // Generation 0 starts with sample text; File → New remounts empty.
        let mut ta = text_area(editor_id);
        if generation == 0 {
            ta = ta.value(SAMPLE_TEXT);
        }
        ta.render(cx, window)
    }
}

impl Render for NotepadApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title_icon = self.icon("title-icon", "ie-paper.png", 16., cx);
        let editor = self.editor(window, cx);

        let body = div()
            .id("notepad-body")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(cx.theme().get_color("surface.base").unwrap_or_default())
            .child(self.menu_bar(cx))
            .child(
                div()
                    .id("notepad-editor-host")
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .p(px(2.))
                    .child(editor),
            );

        // Window chrome (three-side blue frame, Luna title bar,
        // OS-wired caption buttons, drag region) is the framework
        // scaffold — the demo only supplies title icon and body.
        self.chrome.render(Some(title_icon), body, cx)
    }
}
