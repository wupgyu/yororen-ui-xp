use std::sync::OnceLock;

use gpui::SharedString;

pub enum ArrowDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ArrowDirection {
    fn slug(self) -> &'static str {
        match self {
            Self::Up => "arrow-up",
            Self::Down => "arrow-down",
            Self::Left => "arrow-left",
            Self::Right => "arrow-right",
        }
    }
}

pub enum IconName {
    Search,

    Arrow(ArrowDirection),
    Check,
    Warning,
    Info,
    Close,
    Maximize(bool),
    Minimize,
    User,
    Pencil,
    Trash,
    File,
    Folder,
}

/// Cache the `SharedString` per `IconName` so we don't allocate a
/// `String` on every `Icon::from(name)` call. The cache holds at
/// most one entry per variant and is initialized on first use.
fn cached(name: IconName) -> SharedString {
    static CACHE: OnceLock<std::sync::Mutex<std::collections::HashMap<&'static str, SharedString>>> =
        OnceLock::new();
    let key: &'static str = match &name {
        IconName::Search => "search",
        IconName::Check => "check",
        IconName::Warning => "warning",
        IconName::Info => "info",
        IconName::Close => "close",
        IconName::Minimize => "minimize",
        IconName::User => "user",
        IconName::Pencil => "pencil",
        IconName::Trash => "trash",
        IconName::File => "file",
        IconName::Folder => "folder",
        IconName::Arrow(_) => "arrow",
        IconName::Maximize(_) => "maximize",
    };
    let cache = CACHE.get_or_init(|| std::sync::Mutex::new(Default::default()));
    if let Some(s) = cache.lock().expect("icon cache poisoned").get(key).cloned() {
        return s;
    }
    let slug: String = match name {
        IconName::Search => "search".to_string(),
        IconName::Arrow(direction) => direction.slug().to_string(),
        IconName::Check => "check".to_string(),
        IconName::Warning => "warning".to_string(),
        IconName::Info => "info".to_string(),
        IconName::Close => "close".to_string(),
        IconName::Maximize(i) => format!("maximize-{}", if i { "on" } else { "off" }),
        IconName::Minimize => "minimize".to_string(),
        IconName::User => "user".to_string(),
        IconName::Pencil => "pencil".to_string(),
        IconName::Trash => "trash".to_string(),
        IconName::File => "file".to_string(),
        IconName::Folder => "folder".to_string(),
    };
    let s: SharedString = format!("icons/{slug}.svg").into();
    cache.lock().expect("icon cache poisoned").insert(key, s.clone());
    s
}

impl From<IconName> for SharedString {
    fn from(value: IconName) -> Self {
        cached(value)
    }
}
