//! Windows XP My Computer scene.

use gpui::{
    Context, Div, FontWeight, InteractiveElement, IntoElement, ParentElement, Render, Stateful,
    StatefulInteractiveElement, Styled, Window, div, px,
};
use yororen_ui::headless::card::{CardAppearance, card};
use yororen_ui::headless::dropdown_menu::{DropdownItem, DropdownMenuState, dropdown_menu};
use yororen_ui::headless::image::{ImageSource, image};
use yororen_ui::headless::menu::{MenuState, menu};
use yororen_ui::theme::ActiveTheme;
use yororen_ui::xp_renderer::XpAppWindow;

use crate::menu_data;

/// (id, label, items) triple driving the six top menus.
type MenuSpec = (&'static str, &'static str, fn() -> Vec<DropdownItem>);

struct MenuPair {
    dropdown: gpui::Entity<DropdownMenuState>,
    menu: gpui::Entity<MenuState>,
    label: &'static str,
    id: &'static str,
}

pub struct MyComputerApp {
    chrome: XpAppWindow,
    menus: Vec<MenuPair>,
}

impl MyComputerApp {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // XP window chrome scaffold: always-open modal state with
        // the window title; OS-wired caption buttons live in render.
        let chrome = XpAppWindow::new(cx, "My Computer");

        let specs: [MenuSpec; 6] = [
            ("file", "File", menu_data::file_items),
            ("edit", "Edit", menu_data::edit_items),
            ("view", "View", menu_data::view_items),
            ("favorites", "Favorites", menu_data::favorites_items),
            ("tools", "Tools", menu_data::tools_items),
            ("help", "Help", menu_data::help_items),
        ];

        let mut menus = Vec::new();
        for (id, label, items_fn) in specs {
            let dropdown = DropdownMenuState::new(cx);
            let menu_state = MenuState::new(cx);
            let items = items_fn();
            let dropdown_for_select = dropdown.clone();
            menu_state.update(cx, |s, _| {
                s.set_items(items);
                s.set_on_select(move |item_id, window, cx| {
                    if item_id.as_ref() == "file.close" {
                        window.remove_window();
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

        Self { chrome, menus }
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

        bar.child(div().flex_1()).child(
            self.icon("menu-win-logo", "windows.png", 16., cx)
                .mr(px(4.)),
        )
    }

    fn function_bar(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let toolbar_bg = cx
            .theme()
            .get_color("xp.explorer.toolbar_bg")
            .or_else(|| cx.theme().get_color("surface.base"))
            .unwrap_or_default();

        div()
            .id("function-bar")
            .flex()
            .flex_row()
            .items_center()
            .h(px(36.))
            .px(px(2.))
            .gap(px(2.))
            .bg(toolbar_bg)
            .border_b_1()
            .border_color(self.toolbar_border(cx))
            .child(self.tool_btn_disabled("fb-back", "back.png", 30., Some("Back"), cx))
            .child(self.tool_btn_disabled("fb-forward", "forward.png", 30., None, cx))
            .child(self.tool_btn("fb-up", "up.png", 22., None, cx))
            .child(self.sep())
            .child(self.tool_btn("fb-search", "299(32x32).png", 22., Some("Search"), cx))
            .child(self.tool_btn("fb-folders", "337(32x32).png", 22., Some("Folders"), cx))
            .child(self.sep())
            .child(self.tool_btn("fb-views", "358(32x32).png", 22., None, cx))
    }

    fn tool_btn(
        &mut self,
        id: &'static str,
        icon: &str,
        icon_size: f32,
        text: Option<&str>,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let mut el = div()
            .id(id)
            .flex()
            .flex_row()
            .items_center()
            .h_full()
            .px(px(3.))
            .border_1()
            .border_color(gpui::hsla(0., 0., 0., 0.))
            .hover(|s| {
                s.border_color(gpui::hsla(0., 0., 0., 0.15))
                    .bg(gpui::hsla(0., 0., 1., 0.35))
            })
            .child(self.icon(format!("{id}-icon"), icon, icon_size, cx));
        if let Some(t) = text {
            el = el.child(
                div()
                    .text_size(px(11.))
                    .ml(px(2.))
                    .mr(px(2.))
                    .child(t.to_string()),
            );
        }
        el
    }

    fn tool_btn_disabled(
        &mut self,
        id: &'static str,
        icon: &str,
        icon_size: f32,
        text: Option<&str>,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let mut el = div()
            .id(id)
            .flex()
            .flex_row()
            .items_center()
            .h_full()
            .px(px(3.))
            .opacity(0.55)
            .child(self.icon(format!("{id}-icon"), icon, icon_size, cx));
        if let Some(t) = text {
            el = el.child(
                div()
                    .text_size(px(11.))
                    .ml(px(2.))
                    .mr(px(2.))
                    .child(t.to_string()),
            );
        }
        el
    }

    fn sep(&self) -> Div {
        div()
            .w(px(1.))
            .h(gpui::relative(0.9))
            .mx(px(2.))
            .bg(gpui::hsla(0., 0., 0., 0.2))
    }

    /// Faint etched line under the menu / function / address bars
    /// (reference: `rgba(0, 0, 0, 0.1)`), not the navy
    /// `border.default` reserved for control outlines.
    fn toolbar_border(&self, cx: &mut Context<Self>) -> gpui::Hsla {
        cx.theme()
            .get_color("xp.explorer.toolbar_border")
            .unwrap_or_else(|| gpui::hsla(0., 0., 0., 0.1))
    }

    fn address_bar(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let border = cx
            .theme()
            .get_color("xp.explorer.address_border")
            .or_else(|| cx.theme().get_color("xp.input.border"))
            .unwrap_or_default();
        let toolbar_bg = cx
            .theme()
            .get_color("xp.explorer.toolbar_bg")
            .or_else(|| cx.theme().get_color("surface.base"))
            .unwrap_or_default();

        div()
            .id("address-bar")
            .flex()
            .flex_row()
            .items_center()
            .h(px(22.))
            .px(px(2.))
            .gap(px(4.))
            .bg(toolbar_bg)
            .border_b_1()
            .border_color(self.toolbar_border(cx))
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(gpui::hsla(0., 0., 0., 0.5))
                    .px(px(4.))
                    .child("Address"),
            )
            .child(
                div()
                    .id("address-well")
                    .flex()
                    .flex_row()
                    .items_center()
                    .flex_1()
                    .h_full()
                    .px(px(2.))
                    .bg(gpui::hsla(0., 0., 1., 1.))
                    .border_1()
                    .border_color(border)
                    .child(self.icon("addr-computer", "676(16x16).png", 14., cx))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(11.))
                            .ml(px(4.))
                            .child("My Computer"),
                    )
                    .child(self.icon("addr-dd", "dropdown.png", 15., cx)),
            )
            .child(
                div()
                    .id("addr-go")
                    .flex()
                    .flex_row()
                    .items_center()
                    .h_full()
                    .px(px(4.))
                    .child(self.icon("addr-go-icon", "290.png", 16., cx))
                    .child(div().text_size(px(11.)).ml(px(3.)).child("Go")),
            )
    }

    fn task_row(
        &mut self,
        id: &'static str,
        icon: &str,
        text: &str,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let link = cx
            .theme()
            .get_color("xp.explorer.link")
            .unwrap_or_else(|| gpui::hsla(0.58, 1.0, 0.4, 1.0));
        div()
            .id(id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.))
            .px(px(10.))
            .py(px(2.))
            .child(self.icon(format!("{id}-ic"), icon, 16., cx))
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(link)
                    .child(text.to_string()),
            )
    }

    fn task_card(
        &mut self,
        id: &'static str,
        title: &str,
        rows: &[(&'static str, &'static str, &'static str)],
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let pullup = self.icon(format!("{id}-pull"), "pullup.png", 18., cx);
        let mut card_el = card(id, cx)
            .appearance(CardAppearance::ExplorerTask)
            .title(title.to_string())
            .header_trailing(pullup)
            .render(cx)
            .mb(px(12.));

        for (row_id, icon, text) in rows {
            card_el = card_el.child(self.task_row(row_id, icon, text, cx));
        }
        card_el
    }

    fn details_card(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let pullup = self.icon("details-pull", "pullup.png", 18., cx);
        card("task-details", cx)
            .appearance(CardAppearance::ExplorerTask)
            .title("Details")
            .header_trailing(pullup)
            .render(cx)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .px(px(10.))
                    .py(px(6.))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(4.))
                            .child(self.icon("details-info", "view-info.ico", 16., cx))
                            .child(
                                div()
                                    .text_size(px(11.))
                                    .font_weight(FontWeight::BOLD)
                                    .child("My Computer"),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(gpui::hsla(0., 0., 0.2, 1.))
                            .child("System Folder"),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(gpui::hsla(0., 0., 0.2, 1.))
                            .child("Microsoft Windows XP"),
                    ),
            )
    }

    fn left_pane(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let from = cx
            .theme()
            .get_color("xp.explorer.task_pane_bg_from")
            .unwrap_or_else(|| gpui::hsla(0.64, 0.7, 0.7, 1.));
        let to = cx
            .theme()
            .get_color("xp.explorer.task_pane_bg_to")
            .unwrap_or_else(|| gpui::hsla(0.64, 0.7, 0.5, 1.));

        div()
            .id("left-pane")
            .w(px(180.))
            .h_full()
            .p(px(10.))
            .bg(gpui::linear_gradient(
                180.,
                gpui::linear_color_stop(from, 0.),
                gpui::linear_color_stop(to, 1.),
            ))
            .child(self.task_card(
                "task-system",
                "System Tasks",
                &[
                    ("sys-view", "view-info.ico", "View system information"),
                    ("sys-add", "302(16x16).png", "Add or remove programs"),
                    ("sys-change", "300(16x16).png", "Change a setting"),
                ],
                cx,
            ))
            .child(self.task_card(
                "task-other",
                "Other Places",
                &[
                    ("oth-net", "693(16x16).png", "My Network Places"),
                    ("oth-docs", "308(16x16).png", "My Documents"),
                    ("oth-shared", "318(16x16).png", "Shared Documents"),
                    ("oth-cp", "300(16x16).png", "Control Panel"),
                ],
                cx,
            ))
            .child(self.details_card(cx))
    }

    fn icon_item(
        &mut self,
        id: &'static str,
        icon: &str,
        text: &str,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        div()
            .id(id)
            .flex()
            .flex_col()
            .items_center()
            .w(px(90.))
            .p(px(6.))
            .gap(px(4.))
            .child(self.icon(format!("{id}-ic"), icon, 48., cx))
            .child(
                div()
                    .text_size(px(11.))
                    .whitespace_normal()
                    .child(text.to_string()),
            )
    }

    fn right_group(
        &mut self,
        id: &'static str,
        title: &str,
        items: &[(&'static str, &'static str, &'static str)],
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let header_fg = cx
            .theme()
            .get_color("xp.explorer.group_header")
            .or_else(|| cx.theme().get_color("xp.explorer.task_card_title"))
            .unwrap_or_else(|| gpui::hsla(0.62, 0.8, 0.27, 1.));
        let rule_from = cx
            .theme()
            .get_color("xp.explorer.group_rule_from")
            .unwrap_or_else(|| gpui::rgb(0x70BFFF).into());
        let rule_to = cx
            .theme()
            .get_color("surface.raised")
            .unwrap_or_else(|| gpui::hsla(0., 0., 1., 1.));

        let mut content = div().flex().flex_row().flex_wrap().gap(px(8.)).p(px(8.));
        for (item_id, icon, text) in items {
            content = content.child(self.icon_item(item_id, icon, text, cx));
        }

        div()
            .id(id)
            .flex()
            .flex_col()
            .w_full()
            .mb(px(8.))
            .child(
                div()
                    .px(px(8.))
                    .py(px(4.))
                    .font_weight(FontWeight::BOLD)
                    .text_size(px(11.))
                    .text_color(header_fg)
                    .child(title.to_string()),
            )
            // Reference header `:after` rule: a 1px #70BFFF→white
            // gradient fading out 300px from the pane edge, not a
            // full-width gray line.
            .child(div().h(px(1.)).w(px(300.)).bg(gpui::linear_gradient(
                90.,
                gpui::linear_color_stop(rule_from, 0.),
                gpui::linear_color_stop(rule_to, 1.),
            )))
            .child(content)
    }

    fn right_pane(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let bg = cx
            .theme()
            .get_color("surface.raised")
            .unwrap_or_else(|| gpui::hsla(0., 0., 1., 1.));
        div()
            .id("right-pane")
            .flex_1()
            .h_full()
            .bg(bg)
            .overflow_hidden()
            .child(self.right_group(
                "grp-files",
                "Files Stored on This Computer",
                &[
                    ("it-shared", "318(48x48).png", "Shared Documents"),
                    ("it-user", "318(48x48).png", "User's Documents"),
                ],
                cx,
            ))
            .child(self.right_group(
                "grp-disk",
                "Hard Disk Drives",
                &[("it-c", "334(48x48).png", "Local Disk (C:)")],
                cx,
            ))
            .child(self.right_group(
                "grp-cd",
                "Devices with Removable Storage",
                &[("it-d", "111(48x48).png", "CD Drive (D:)")],
                cx,
            ))
    }

    fn content(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let content_bg = cx
            .theme()
            .get_color("xp.explorer.content_bg")
            .unwrap_or_else(|| gpui::hsla(0., 0., 0.95, 1.));
        div()
            .id("content")
            .flex_1()
            .flex()
            .flex_row()
            .min_h_0()
            .bg(content_bg)
            // Reference `.com__content`: 1px sunken edge with the top
            // side open (`border-top-width: 0`) — separation from the
            // address bar comes from the bar's own etched line.
            .border_1()
            .border_t(px(0.))
            .border_color(gpui::hsla(0., 0., 0., 0.4))
            .child(self.left_pane(cx))
            .child(self.right_pane(cx))
    }
}

impl Render for MyComputerApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title_icon = self.icon("title-icon", "676(16x16).png", 16., cx);

        let body = div()
            .id("my-computer-body")
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .child(self.menu_bar(cx))
            .child(self.function_bar(cx))
            .child(self.address_bar(cx))
            .child(self.content(cx));

        // Window chrome (three-side blue frame, Luna title bar,
        // OS-wired caption buttons, drag region) is the framework
        // scaffold — the demo only supplies title icon and body.
        self.chrome.render(Some(title_icon), body, cx)
    }
}
