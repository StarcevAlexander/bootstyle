use crate::color::Color;
use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Badge {
    text: String,
    color: Color,
    pill: bool,
}

impl Badge {
    pub fn new(text: impl Into<String>, color: Color) -> Self {
        Self {
            text: text.into(),
            color,
            pill: false,
        }
    }

    pub fn pill(mut self, pill: bool) -> Self {
        self.pill = pill;
        self
    }
}

impl Render for Badge {
    fn render(&self) -> String {
        let mut classes = format!("badge text-bg-{}", self.color.suffix());
        if self.pill {
            classes.push_str(" rounded-pill");
        }
        format!("<span class=\"{classes}\">{}</span>", self.text)
    }
}
