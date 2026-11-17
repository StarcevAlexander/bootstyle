use crate::color::Color;
use crate::render::Render;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Normal,
    Large,
    Small,
}

impl ButtonSize {
    pub fn class(self) -> &'static str {
        match self {
            ButtonSize::Normal => "",
            ButtonSize::Large => " btn-lg",
            ButtonSize::Small => " btn-sm",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Button {
    text: String,
    color: Color,
    outline: bool,
    size: ButtonSize,
    disabled: bool,
    href: Option<String>,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: Color::Primary,
            outline: false,
            size: ButtonSize::Normal,
            disabled: false,
            href: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn outline(mut self, outline: bool) -> Self {
        self.outline = outline;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn link(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
}

impl Render for Button {
    fn render(&self) -> String {
        let prefix = if self.outline { "btn-outline-" } else { "btn-" };
        let classes = format!("btn {}{}{}", prefix, self.color.suffix(), self.size.class());
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        match &self.href {
            Some(href) => format!(
                "<a class=\"{classes}\" href=\"{href}\"{disabled_attr}>{}</a>",
                self.text
            ),
            None => format!(
                "<button type=\"button\" class=\"{classes}\"{disabled_attr}>{}</button>",
                self.text
            ),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ButtonGroup {
    buttons: Vec<Button>,
    vertical: bool,
    size: ButtonSize,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
}

impl Render for ButtonGroup {
    fn render(&self) -> String {
        let mut classes = if self.vertical {
            String::from("btn-group-vertical")
        } else {
            String::from("btn-group")
        };
        classes.push_str(self.size.class());
        let mut out = format!("<div class=\"{classes}\" role=\"group\">");
        for button in &self.buttons {
            out.push_str(&button.render());
        }
        out.push_str("</div>");
        out
    }
}
