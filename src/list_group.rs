use crate::render::Render;

#[derive(Clone, Debug)]
pub struct ListGroupItem {
    text: String,
    active: bool,
    disabled: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ListGroup {
    items: Vec<ListGroupItem>,
    flush: bool,
}

impl ListGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, text: impl Into<String>) -> Self {
        self.items.push(ListGroupItem {
            text: text.into(),
            active: false,
            disabled: false,
        });
        self
    }

    pub fn active(mut self, text: impl Into<String>) -> Self {
        self.items.push(ListGroupItem {
            text: text.into(),
            active: true,
            disabled: false,
        });
        self
    }

    pub fn disabled(mut self, text: impl Into<String>) -> Self {
        self.items.push(ListGroupItem {
            text: text.into(),
            active: false,
            disabled: true,
        });
        self
    }

    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = flush;
        self
    }
}

impl Render for ListGroup {
    fn render(&self) -> String {
        let mut classes = String::from("list-group");
        if self.flush {
            classes.push_str(" list-group-flush");
        }
        let mut out = format!("<ul class=\"{classes}\">");
        for item in &self.items {
            let mut classes = String::from("list-group-item");
            if item.active {
                classes.push_str(" active");
            }
            if item.disabled {
                classes.push_str(" disabled");
            }
            out.push_str(&format!(
                "<li class=\"{classes}\" aria-current=\"{}\">{}</li>",
                if item.active { "true" } else { "false" },
                item.text
            ));
        }
        out.push_str("</ul>");
        out
    }
}
