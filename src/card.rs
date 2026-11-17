use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct Card {
    title: Option<String>,
    subtitle: Option<String>,
    body: String,
    header: Option<String>,
    footer: Option<String>,
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }
}

impl Render for Card {
    fn render(&self) -> String {
        let mut out = String::from("<div class=\"card\" style=\"width: 18rem;\">");
        if let Some(header) = &self.header {
            out.push_str(&format!("<div class=\"card-header\">{header}</div>"));
        }
        out.push_str("<div class=\"card-body\">");
        if let Some(title) = &self.title {
            out.push_str(&format!("<h5 class=\"card-title\">{title}</h5>"));
        }
        if let Some(subtitle) = &self.subtitle {
            out.push_str(&format!(
                "<h6 class=\"card-subtitle mb-2 text-body-secondary\">{subtitle}</h6>"
            ));
        }
        out.push_str(&format!("<p class=\"card-text\">{}</p>", self.body));
        out.push_str("</div>");
        if let Some(footer) = &self.footer {
            out.push_str(&format!(
                "<div class=\"card-footer text-body-secondary\">{footer}</div>"
            ));
        }
        out.push_str("</div>");
        out
    }
}
