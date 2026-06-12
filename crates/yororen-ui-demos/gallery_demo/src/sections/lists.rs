//! Section 8 — Lists, Tables, Trees, Forms, Misc. Each
//! component is wrapped in a labelled `cell` so the user can
//! identify every instance.

use std::collections::BTreeSet;

use gpui::{Context, Div, ElementId, IntoElement, ParentElement, Styled, Window, div, px};

use yororen_ui::headless::form::form;
use yororen_ui::headless::form_field::form_field;
use yororen_ui::headless::label::label;
use yororen_ui::headless::list_item::list_item;
use yororen_ui::headless::radio_group::radio_group;
use yororen_ui::headless::spacer::spacer;
use yororen_ui::headless::table::TableColumn;
use yororen_ui::headless::table::table;
use yororen_ui::headless::text_input::text_input;
use yororen_ui::headless::tree::TreeData;
use yororen_ui::headless::tree::node_id;
use yororen_ui::headless::tree::tree;
use yororen_ui::headless::tree_item::tree_item;
use yororen_ui::headless::virtual_list::{uniform_virtual_list, virtual_list};
use yororen_ui::headless::button::button;
use yororen_ui::i18n::Translate;

use crate::sections::cell;
use crate::state::GalleryApp;

const LIST_ITEMS: &[(&str, &str)] = &[
    ("li-1", "First item"),
    ("li-2", "Second item"),
    ("li-3", "Third item"),
];

pub fn render(app: &mut GalleryApp, window: &mut Window, cx: &mut Context<GalleryApp>) -> Div {
    // Sync the virtual_list controller's item_count to the demo's
    // tracked `vl_item_count`. The `on_visible_range_change`
    // callback below only bumps `vl_item_count` (a plain field
    // update); it must NOT call `controller.reset/append(...)`
    // directly, because the callback fires from inside
    // `gpui::list`'s scroll path while `ListState`'s `RefCell` is
    // already borrowed by paint — calling reset/splice (which
    // both `borrow_mut` the same RefCell) would panic with
    // "RefCell already borrowed". Doing the sync here at the top
    // of `render` (one frame later) puts the borrow in a fresh
    // stack with no outstanding aliases.
    //
    // For *growth* we use `controller.append(n)` (which is
    // `splice(end..end, n)` internally) — this preserves the
    // current `logical_scroll_top` so the user's scroll position
    // stays put when new tail items arrive. `controller.reset(n)`
    // would clear `logical_scroll_top = None` and snap the list
    // back to the top, which is the wrong UX for infinite loading.
    // For *shrink* (current > target) we fall back to `reset`
    // because there is no "remove from tail" splice helper.
    let current = app.list_controller.state().item_count();
    if current < app.vl_item_count {
        app.list_controller.append(app.vl_item_count - current);
    } else if current > app.vl_item_count {
        app.list_controller.reset(app.vl_item_count);
    }

    // --- list_item: 3 selectable rows ---
    let mut list_col = div().flex().flex_col().gap(px(4.)).w(px(220.));
    for (i, (id, title)) in LIST_ITEMS.iter().enumerate() {
        list_col = list_col.child(
            list_item(*id, *title, cx)
                .selected(app.selected_list_item == Some(i))
                .render(cx),
        );
    }
    let list_wrapped = cell("list_item (3 rows; selected via props)", list_col, cx);

    // --- form + form_field (with a real text_input + submit button) ---
    let entity_form = cx.entity().clone();
    let entity_text = cx.entity().clone();
    let email_input_el = text_input("lists-ff-email-input")
        .placeholder(cx.t("form.email_placeholder"))
        .on_change(move |new: &str, _w, cx| {
            entity_text.update(cx, |s, _cx| s.form_email_value = new.to_string());
        })
        .render(cx, window);

    let form_field_el = form_field("lists-ff-email", "email", cx)
        .label(cx.t("form.email"))
        .required(true)
        .input(email_input_el)
        .render(cx);

    let form_props = form("lists-form", cx)
        .value("email", app.form_email_value.clone())
        .on_submit(move |vals, _w, cx| {
            entity_form.update(cx, |s, _cx| {
                s.form_submit_count += 1;
                if let Some(e) = vals.get("email") {
                    s.form_email_value = e.to_string();
                    s.form_email_error = if e.contains('@') {
                        None
                    } else {
                        Some("must contain @".to_string())
                    };
                }
            });
        })
        .submit("Submit");

    let submit_btn_el = form_props
        .submit_button(cx)
        .expect("submit_label was set");

    let form_el = form_props
        .render(cx)
        .w(px(300.))
        .child(form_field_el)
        .child(submit_btn_el)
        .child(
            label(
                "form-submit-count",
                format!("submitted: {} | last error: {:?}", app.form_submit_count, app.form_email_error),
                cx,
            )
            .muted(true)
            .render(cx),
        );
    let form_wrapped = cell("form + form_field (email validation)", form_el, cx);

    // --- table ---
    let entity_table = cx.entity().clone();
    let table_el = table("lists-table", cx)
        .columns(vec![
            TableColumn::new("name", "Name").width(120.),
            TableColumn::new("age", "Age").width(60.),
            TableColumn::new("city", "City").width(120.),
        ])
        .rows(vec![
            vec!["Alice".into(), "30".into(), "Beijing".into()],
            vec!["Bob".into(), "25".into(), "Shanghai".into()],
            vec!["Carol".into(), "28".into(), "Shenzhen".into()],
        ])
        .selected(app.selected_table_row.unwrap_or(0))
        .on_select(move |i, _w, cx| {
            entity_table.update(cx, |s, _cx| s.selected_table_row = Some(i));
        })
        .render(cx)
        .w(px(320.));
    let table_wrapped = cell("table (3 rows × 3 cols)", table_el, cx);

    // --- tree (with tree_item rows) ---
    let mut tree_data = TreeData::new();
    tree_data.add(None, node_id("root"), "Root");
    tree_data.add(Some(node_id("root")), node_id("child1"), "Child 1");
    tree_data.add(Some(node_id("root")), node_id("child2"), "Child 2");
    tree_data.add(Some(node_id("child1")), node_id("leaf1"), "Leaf 1");
    tree_data.add(Some(node_id("child1")), node_id("leaf2"), "Leaf 2");
    let tree_data_for_iter = tree_data.clone();
    let entity_tree = cx.entity().clone();
    let tree_expanded: BTreeSet<_> = app.tree_expanded.clone();
    let tree_selected = app.tree_selected.clone();
    let mut tree_el = tree("lists-tree", cx)
        .data(tree_data)
        .render(cx)
        .w(px(240.));
    // Use the `TreeData::flatten` helper to walk only the
    // currently-visible (expanded) nodes, in depth-first
    // pre-order — mirrors v0.2's `flatten_tree` output.
    let visible = tree_data_for_iter.flatten(&tree_expanded);
    for (id, depth) in visible {
        let has_children = !tree_data_for_iter.children_of(&id).is_empty();
        let label_text = tree_data_for_iter
            .label_of(&id)
            .unwrap_or("")
            .to_string();
        let is_expanded = tree_expanded.contains(&id);
        let is_selected = tree_selected.as_ref() == Some(&id);

        let entity_for_toggle = entity_tree.clone();
        let entity_for_select = entity_tree.clone();
        let toggle_id = id.clone();
        let select_id = id.clone();
        // Unique ElementId per row — gpui de-duplicates by id,
        // so identical ids would collapse all rows into one.
        let row_id: ElementId = format!("lists-tree-row-{}", id.0).into();
        // Double-click toggles: the renderer falls back to
        // `on_toggle` when `on_double_click` is not set, so we
        // don't need to wire it explicitly — but we wire it
        // here to make the behaviour explicit at the call site.
        let entity_for_double = entity_tree.clone();
        let double_id = id.clone();
        tree_el = tree_el.child(
            tree_item(row_id, id.clone(), label_text, cx)
                .depth(depth)
                .has_children(has_children)
                .expanded(is_expanded)
                .selected(is_selected)
                .on_toggle(move |_, _, cx| {
                    let toggle_id = toggle_id.clone();
                    entity_for_toggle.update(cx, |s, _cx| {
                        if !s.tree_expanded.remove(&toggle_id) {
                            s.tree_expanded.insert(toggle_id);
                        }
                    });
                })
                .on_click(move |_, _, cx| {
                    entity_for_select.update(cx, |s, _cx| {
                        s.tree_selected = Some(select_id.clone());
                    });
                })
                .on_double_click(move |_, _, cx| {
                    let double_id = double_id.clone();
                    entity_for_double.update(cx, |s, _cx| {
                        if !s.tree_expanded.remove(&double_id) {
                            s.tree_expanded.insert(double_id);
                        }
                    });
                })
                .render(cx, window),
        );
    }
    let tree_wrapped = cell("tree + tree_item (3-5 rows; click chevron or double-click row to expand, click row to select)", tree_el, cx);

    // --- virtual_list ---
    // 1000+ items (grows via infinite scroll), each rendered as
    // a `list_item` via the row closure handed to `gpui::list`.
    // The closure captures the GalleryApp entity so the row reads
    // the *current* selection state on every frame and `on_click`
    // updates it. Three new bits over the basic demo:
    //   1. `on_visible_range_change` updates `app.vl_visible_range`
    //      every scroll and, once the visible end approaches the
    //      logical end, calls `controller.reset(new_count)` to
    //      simulate infinite loading.
    //   2. Two buttons (Top / Bottom) drive
    //      `controller.scroll_to_top()` /
    //      `controller.scroll_to_bottom()`.
    //   3. A live status `label` shows the current visible range,
    //      total item_count, and the auto-loaded batch counter.
    //
    // The button on_click closures route through
    // `entity.update(cx, |s, _| s.list_controller.scroll_to_…())`
    // because `ButtonProps::on_click` requires `Send + Sync` but
    // `VirtualListController` is `Rc<RefCell<…>>` and thus
    // single-threaded. Going through `Entity` keeps the closure
    // Send + Sync (Entity is) and reaches the controller on the
    // main thread inside the update callback.
    let app_entity_for_vl = cx.entity().clone();
    let app_entity_for_range = cx.entity().clone();
    let vl = virtual_list("lists-vl", &app.list_controller, cx)
        .row(move |ix, _window, cx| {
            let app_entity = app_entity_for_vl.clone();
            // Read the *current* selected index from the entity on
            // each row construction; the closure is `FnMut` and is
            // re-invoked per visible row per frame.
            let selected = app_entity.read(cx).selected_list_item == Some(ix);
            let row_id: ElementId = format!("vl-row-{ix}").into();
            list_item(row_id, format!("Item #{ix}"), cx)
                .selected(selected)
                .on_click(move |_ev, _window, cx| {
                    app_entity.update(cx, |s, _cx| {
                        s.selected_list_item = Some(ix);
                    });
                })
                .render(cx)
                .into_any_element()
        })
        .on_visible_range_change(move |range, total, _window, cx| {
            // Bump the demo's tracked counts only — the actual
            // `controller.reset(...)` happens at the top of the
            // next `render` (see comment there). Calling reset
            // from inside this callback would re-enter the
            // ListState `RefCell` that the gpui scroll path is
            // currently borrowing and panic with "RefCell already
            // borrowed".
            app_entity_for_range.update(cx, |s, _cx_inner| {
                s.vl_visible_range = Some(range.clone());
                if range.end + 50 >= total && s.vl_item_count < 5_000 {
                    s.vl_item_count += 100;
                    s.vl_load_count += 1;
                }
            });
        })
        .render(cx)
        .w(px(240.))
        .h(px(180.));
    // Control buttons + status label, stacked below the scrollable list.
    let entity_for_vl_top = cx.entity().clone();
    let entity_for_vl_bottom = cx.entity().clone();
    let top_btn = button("vl-top", cx)
        .on_click(move |_, _, cx| {
            entity_for_vl_top.update(cx, |s, _| s.list_controller.scroll_to_top());
        })
        .render(cx)
        .child("Top");
    let bottom_btn = button("vl-bottom", cx)
        .on_click(move |_, _, cx| {
            entity_for_vl_bottom.update(cx, |s, _| s.list_controller.scroll_to_bottom());
        })
        .render(cx)
        .child("Bottom");
    let controls_row = div()
        .flex()
        .flex_row()
        .gap(px(6.))
        .child(top_btn)
        .child(bottom_btn);
    let status_label = label(
        "vl-status",
        format!(
            "visible: {:?} | item_count: {} | auto-loaded batches: {}",
            app.vl_visible_range, app.vl_item_count, app.vl_load_count
        ),
        cx,
    )
    .muted(true)
    .render(cx);
    let vl_col = div()
        .flex()
        .flex_col()
        .gap(px(6.))
        .child(vl)
        .child(controls_row)
        .child(status_label);
    let vl_wrapped = cell(
        "virtual_list (scroll_to_top/bottom + on_visible_range_change + infinite loading)",
        vl_col,
        cx,
    );

    // --- uniform_virtual_list ---
    // 1000 fixed-height rows. `gpui::uniform_list` measures only
    // the first row and lays out the rest in a line — much faster
    // than `gpui::list` for large uniform-height lists. The cell
    // also has Top / Bottom buttons wired to the
    // `UniformVirtualListController` (via `entity.update` for the
    // same Send + Sync reason as virtual_list above).
    let uvl = uniform_virtual_list("lists-uvl", 1_000, &app.uniform_list_controller, cx)
        .row(move |ix, _w, cx| {
            let row_id: ElementId = format!("uvl-row-{ix}").into();
            list_item(row_id, format!("Uniform row #{ix}"), cx)
                .render(cx)
                .into_any_element()
        })
        .render(cx)
        .w(px(240.))
        .h(px(180.));
    let entity_for_uvl_top = cx.entity().clone();
    let entity_for_uvl_bottom = cx.entity().clone();
    let uvl_top_btn = button("uvl-top", cx)
        .on_click(move |_, _, cx| {
            entity_for_uvl_top.update(cx, |s, _| s.uniform_list_controller.scroll_to_top());
        })
        .render(cx)
        .child("Top");
    let uvl_bottom_btn = button("uvl-bottom", cx)
        .on_click(move |_, _, cx| {
            entity_for_uvl_bottom.update(cx, |s, _| s.uniform_list_controller.scroll_to_bottom());
        })
        .render(cx)
        .child("Bottom");
    let uvl_controls = div()
        .flex()
        .flex_row()
        .gap(px(6.))
        .child(uvl_top_btn)
        .child(uvl_bottom_btn);
    let uvl_col = div()
        .flex()
        .flex_col()
        .gap(px(6.))
        .child(uvl)
        .child(uvl_controls);
    let uvl_wrapped = cell(
        "uniform_virtual_list (1000 items; uniform-height fast path; scroll_to_top/bottom)",
        uvl_col,
        cx,
    );

    // --- spacer ---
    let sp = spacer("lists-spacer", cx)
        .render(cx)
        .h(px(16.))
        .w_full();
    let sp_wrapped = cell("spacer (16px tall)", sp, cx);

    // --- radio_group empty (also used as a layout shell) ---
    let rg_demo = radio_group("lists-rg", cx)
        .name("rg-2")
        .render(cx)
        .child(label("rg-2-info", "Standalone radio_group (no children)", cx).muted(true).render(cx));
    let rg_wrapped = cell("radio_group (empty shell)", rg_demo, cx);

    div()
        .flex()
        .flex_col()
        .gap(px(12.))
        .child(list_wrapped)
        .child(form_wrapped)
        .child(table_wrapped)
        .child(tree_wrapped)
        .child(vl_wrapped)
        .child(uvl_wrapped)
        .child(sp_wrapped)
        .child(rg_wrapped)
}
