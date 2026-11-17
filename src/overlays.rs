use crate::button::Button;
use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Modal {
    title: String,
    body: String,
    footer: Option<String>,
    open: bool,
}

impl Modal {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            footer: None,
            open: false,
        }
    }

    pub fn footer(mut self, content: impl Into<String>) -> Self {
        self.footer = Some(content.into());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for Modal {
    fn render(&self) -> String {
        let display = if self.open { " display: block;" } else { "" };
        let backdrop = if self.open {
            "<div class=\"modal-backdrop fade show\"></div>"
        } else {
            ""
        };
        let footer = self
            .footer
            .as_ref()
            .map(|f| format!("<div class=\"modal-footer\">{f}</div>"))
            .unwrap_or_default();
        format!(
            "<div class=\"modal\" tabindex=\"-1\" style=\"{display}\"><div class=\"modal-dialog\"><div class=\"modal-content\"><div class=\"modal-header\"><h5 class=\"modal-title\">{}</h5><button type=\"button\" class=\"btn-close\" data-bs-dismiss=\"modal\"></button></div><div class=\"modal-body\">{}</div>{footer}</div></div></div>{backdrop}",
            self.title, self.body
        )
    }
}

#[derive(Clone, Debug)]
pub struct Offcanvas {
    title: String,
    body: String,
    open: bool,
}

impl Offcanvas {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            open: false,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for Offcanvas {
    fn render(&self) -> String {
        let class = if self.open {
            "offcanvas show"
        } else {
            "offcanvas"
        };
        format!(
            "<div class=\"{class}\" tabindex=\"-1\"><div class=\"offcanvas-header\"><h5 class=\"offcanvas-title\">{}</h5><button type=\"button\" class=\"btn-close\"></button></div><div class=\"offcanvas-body\">{}</div></div>",
            self.title, self.body
        )
    }
}

#[derive(Clone, Debug)]
pub struct Dropdown {
    label: String,
    items: Vec<String>,
    open: bool,
    danger_item: Option<usize>,
}

impl Dropdown {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
            open: false,
            danger_item: None,
        }
    }

    pub fn item(mut self, text: impl Into<String>) -> Self {
        self.items.push(text.into());
        self
    }

    pub fn danger_item(mut self, index: usize) -> Self {
        self.danger_item = Some(index);
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for Dropdown {
    fn render(&self) -> String {
        let menu_class = if self.open {
            "dropdown-menu show"
        } else {
            "dropdown-menu"
        };
        let mut out = format!(
            "<div class=\"dropdown\"><button class=\"btn btn-primary dropdown-toggle\" type=\"button\" data-bs-toggle=\"dropdown\">{}</button><ul class=\"{menu_class}\">",
            self.label
        );
        for (i, item) in self.items.iter().enumerate() {
            let item_class = if self.danger_item == Some(i) {
                "dropdown-item text-danger"
            } else {
                "dropdown-item"
            };
            out.push_str(&format!(
                "<li><a class=\"{item_class}\" href=\"#\">{item}</a></li>"
            ));
        }
        out.push_str("</ul></div>");
        out
    }
}

#[derive(Clone, Debug)]
pub struct Tooltip {
    content: String,
    tip: String,
}

impl Tooltip {
    pub fn new(content: impl Into<String>, tip: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            tip: tip.into(),
        }
    }
}

impl Render for Tooltip {
    fn render(&self) -> String {
        format!(
            "<span class=\"bs-tooltip-wrap\">{}<span class=\"bs-tooltip-bubble\">{}</span></span>",
            self.content, self.tip
        )
    }
}

#[derive(Clone, Debug)]
pub struct ModalFooterButton {
    button: Button,
}

impl ModalFooterButton {
    pub fn new(button: Button) -> Self {
        Self { button }
    }
}

impl Render for ModalFooterButton {
    fn render(&self) -> String {
        self.button.render()
    }
}
