//! Static menu data for the classic XP Notepad menu bar.
//!
//! Representative subsets only — most items are disabled or no-op.
//! File → New and File → Exit are wired in `app.rs`.

use yororen_ui::headless::dropdown_menu::{DropdownItem, DropdownMenuItem};

fn item(id: &'static str, label: &'static str, disabled: bool) -> DropdownItem {
    DropdownItem::Item(DropdownMenuItem::new(id, label).disabled(disabled))
}

fn item_shortcut(
    id: &'static str,
    label: &'static str,
    disabled: bool,
    shortcut: &'static str,
) -> DropdownItem {
    DropdownItem::Item(
        DropdownMenuItem::new(id, label)
            .disabled(disabled)
            .shortcut(vec![shortcut.to_string()]),
    )
}

pub fn file_items() -> Vec<DropdownItem> {
    vec![
        item_shortcut("file.new", "New", false, "Ctrl+N"),
        item_shortcut("file.open", "Open...", true, "Ctrl+O"),
        item_shortcut("file.save", "Save", true, "Ctrl+S"),
        item("file.save_as", "Save As...", true),
        DropdownItem::Separator,
        item("file.page_setup", "Page Setup...", true),
        item_shortcut("file.print", "Print...", true, "Ctrl+P"),
        DropdownItem::Separator,
        item("file.exit", "Exit", false),
    ]
}

pub fn edit_items() -> Vec<DropdownItem> {
    vec![
        item_shortcut("edit.undo", "Undo", true, "Ctrl+Z"),
        DropdownItem::Separator,
        item_shortcut("edit.cut", "Cut", true, "Ctrl+X"),
        item_shortcut("edit.copy", "Copy", true, "Ctrl+C"),
        item_shortcut("edit.paste", "Paste", true, "Ctrl+V"),
        item_shortcut("edit.delete", "Delete", true, "Del"),
        DropdownItem::Separator,
        item_shortcut("edit.find", "Find...", true, "Ctrl+F"),
        item("edit.replace", "Replace...", true),
        DropdownItem::Separator,
        item_shortcut("edit.select_all", "Select All", true, "Ctrl+A"),
    ]
}

pub fn format_items() -> Vec<DropdownItem> {
    vec![
        item("format.word_wrap", "Word Wrap", true),
        item("format.font", "Font...", true),
    ]
}

pub fn view_items() -> Vec<DropdownItem> {
    vec![item("view.status_bar", "Status Bar", true)]
}

pub fn help_items() -> Vec<DropdownItem> {
    vec![
        item("help.topics", "Help Topics", true),
        DropdownItem::Separator,
        item("help.about", "About Notepad", true),
    ]
}
