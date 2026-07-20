//! Static menu data for the My Computer menu bar.
//!
//! Ported (flattened) from `xp_react/MyComputer/dropDownData.js`.
//! Nested fly-out menus are represented as flat items; commands
//! have no real business logic.

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
        item("file.create_shortcut", "Create Shortcut", true),
        item("file.delete", "Delete", true),
        item("file.rename", "Rename", true),
        item("file.properties", "Properties", true),
        DropdownItem::Separator,
        item("file.close", "Close", false),
    ]
}

pub fn edit_items() -> Vec<DropdownItem> {
    vec![
        item_shortcut("edit.undo", "Undo", true, "Ctrl+Z"),
        DropdownItem::Separator,
        item_shortcut("edit.cut", "Cut", true, "Ctrl+X"),
        item_shortcut("edit.copy", "Copy", true, "Ctrl+C"),
        item_shortcut("edit.paste", "Paste", true, "Ctrl+V"),
        item("edit.paste_shortcut", "Paste Shortcut", true),
        DropdownItem::Separator,
        item("edit.copy_to", "Copy To Folder...", true),
        item("edit.move_to", "Move To Folder...", true),
        DropdownItem::Separator,
        item_shortcut("edit.select_all", "Select All", false, "Ctrl+A"),
        item("edit.invert", "Invert Selection", false),
    ]
}

pub fn view_items() -> Vec<DropdownItem> {
    vec![
        item("view.toolbars", "Toolbars", false),
        item("view.status_bar", "Status Bar", false),
        item("view.explorer_bar", "Explorer Bar", false),
        DropdownItem::Separator,
        item("view.thumbnails", "Thumbnails", false),
        item("view.tiles", "Tiles", false),
        item("view.icons", "Icons", false),
        item("view.list", "List", false),
        item("view.details", "Details", false),
        DropdownItem::Separator,
        item("view.refresh", "Refresh", false),
    ]
}

pub fn favorites_items() -> Vec<DropdownItem> {
    vec![
        item("fav.add", "Add to Favorites...", false),
        item("fav.organize", "Organize Favorites...", false),
        DropdownItem::Separator,
        item("fav.links", "Links", false),
        item("fav.msn", "MSN.com", false),
    ]
}

pub fn tools_items() -> Vec<DropdownItem> {
    vec![
        item("tools.map", "Map Network Drive...", false),
        item("tools.disconnect", "Disconnect Network Drive...", false),
        item("tools.sync", "Synchronize...", false),
        DropdownItem::Separator,
        item("tools.folder_options", "Folder Options...", false),
    ]
}

pub fn help_items() -> Vec<DropdownItem> {
    vec![
        item("help.center", "Help and Support Center", false),
        DropdownItem::Separator,
        item("help.legal", "Is this copy of Windows legal?", false),
        item("help.about", "About Windows", false),
    ]
}
