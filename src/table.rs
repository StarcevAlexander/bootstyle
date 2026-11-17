use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct Table {
    head: Vec<String>,
    rows: Vec<Vec<String>>,
    striped: bool,
    hover: bool,
    bordered: bool,
    dark: bool,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn head(mut self, head: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.head = head.into_iter().map(Into::into).collect();
        self
    }

    pub fn row(mut self, row: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn hover(mut self, hover: bool) -> Self {
        self.hover = hover;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
}

impl Render for Table {
    fn render(&self) -> String {
        let mut classes = String::from("table");
        if self.striped {
            classes.push_str(" table-striped");
        }
        if self.hover {
            classes.push_str(" table-hover");
        }
        if self.bordered {
            classes.push_str(" table-bordered");
        }
        if self.dark {
            classes.push_str(" table-dark");
        }
        let mut out = format!("<table class=\"{classes}\">");
        if !self.head.is_empty() {
            out.push_str("<thead><tr>");
            for cell in &self.head {
                out.push_str(&format!("<th scope=\"col\">{cell}</th>"));
            }
            out.push_str("</tr></thead>");
        }
        out.push_str("<tbody>");
        for row in &self.rows {
            out.push_str("<tr>");
            for cell in row {
                out.push_str(&format!("<td>{cell}</td>"));
            }
            out.push_str("</tr>");
        }
        out.push_str("</tbody></table>");
        out
    }
}
