#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl Color {
    pub fn suffix(self) -> &'static str {
        match self {
            Color::Primary => "primary",
            Color::Secondary => "secondary",
            Color::Success => "success",
            Color::Danger => "danger",
            Color::Warning => "warning",
            Color::Info => "info",
            Color::Light => "light",
            Color::Dark => "dark",
        }
    }
}
