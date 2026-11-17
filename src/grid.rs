use crate::render::Render;

pub struct Container {
    inner: String,
    fluid: bool,
}

impl Container {
    pub fn new(inner: impl Into<String>) -> Self {
        Self {
            inner: inner.into(),
            fluid: false,
        }
    }

    pub fn fluid(mut self, fluid: bool) -> Self {
        self.fluid = fluid;
        self
    }
}

impl Render for Container {
    fn render(&self) -> String {
        let class = if self.fluid {
            "container-fluid"
        } else {
            "container"
        };
        format!("<div class=\"{class}\">{}</div>", self.inner)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Row {
    cols: Vec<String>,
}

impl Row {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn col(mut self, content: impl Into<String>, span: u8) -> Self {
        let span = span.clamp(1, 12);
        self.cols.push(format!(
            "<div class=\"col-md-{span}\">{}</div>",
            content.into()
        ));
        self
    }

    pub fn auto(mut self, content: impl Into<String>) -> Self {
        self.cols
            .push(format!("<div class=\"col\">{}</div>", content.into()));
        self
    }
}

impl Render for Row {
    fn render(&self) -> String {
        let mut out = String::from("<div class=\"row\">");
        for col in &self.cols {
            out.push_str(col);
        }
        out.push_str("</div>");
        out
    }
}
