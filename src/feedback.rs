use crate::color::Color;
use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Alert {
    text: String,
    color: Color,
    dismissible: bool,
}

impl Alert {
    pub fn new(color: Color, text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color,
            dismissible: false,
        }
    }

    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }
}

impl Render for Alert {
    fn render(&self) -> String {
        let mut classes = format!("alert alert-{}", self.color.suffix());
        if self.dismissible {
            classes.push_str(" alert-dismissible fade show");
        }
        let close = if self.dismissible {
            "<button type=\"button\" class=\"btn-close\" data-bs-dismiss=\"alert\" aria-label=\"Close\"></button>"
        } else {
            ""
        };
        format!(
            "<div class=\"{classes}\" role=\"alert\">{}{close}</div>",
            self.text
        )
    }
}

#[derive(Clone, Debug)]
pub struct Toast {
    title: String,
    body: String,
    time: Option<String>,
    open: bool,
}

impl Toast {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            time: None,
            open: false,
        }
    }

    pub fn time(mut self, time: impl Into<String>) -> Self {
        self.time = Some(time.into());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl Render for Toast {
    fn render(&self) -> String {
        let class = if self.open { "toast show" } else { "toast" };
        let time = self
            .time
            .as_ref()
            .map(|t| format!("<small class=\"text-body-secondary\">{t}</small>"))
            .unwrap_or_default();
        format!(
            "<div class=\"{class}\" role=\"alert\" aria-live=\"assertive\" aria-atomic=\"true\"><div class=\"toast-header\"><strong class=\"me-auto\">{}</strong>{time}<button type=\"button\" class=\"btn-close\" data-bs-dismiss=\"toast\"></button></div><div class=\"toast-body\">{}</div></div>",
            self.title, self.body
        )
    }
}

#[derive(Clone, Debug)]
pub struct Spinner {
    color: Color,
    grow: bool,
    small: bool,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            color: Color::Primary,
            grow: false,
            small: false,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn grow(mut self, grow: bool) -> Self {
        self.grow = grow;
        self
    }

    pub fn small(mut self, small: bool) -> Self {
        self.small = small;
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Spinner {
    fn render(&self) -> String {
        let kind = if self.grow {
            "spinner-grow"
        } else {
            "spinner-border"
        };
        let size = if self.small {
            format!("{kind}-sm")
        } else {
            String::new()
        };
        format!(
            "<div class=\"{kind} {size} text-{}\" role=\"status\"><span class=\"visually-hidden\">Loading...</span></div>",
            self.color.suffix()
        )
    }
}

#[derive(Clone, Debug)]
pub struct Progress {
    value: u8,
    color: Color,
    striped: bool,
    animated: bool,
}

impl Progress {
    pub fn new(value: u8) -> Self {
        Self {
            value: value.clamp(0, 100),
            color: Color::Primary,
            striped: false,
            animated: false,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl Render for Progress {
    fn render(&self) -> String {
        let mut classes = format!("progress-bar bg-{}", self.color.suffix());
        if self.striped {
            classes.push_str(" progress-bar-striped");
        }
        if self.animated {
            classes.push_str(" progress-bar-animated");
        }
        format!(
            "<div class=\"progress\"><div class=\"{classes}\" role=\"progressbar\" style=\"width: {}%;\" aria-valuenow=\"{}\" aria-valuemin=\"0\" aria-valuemax=\"100\"></div></div>",
            self.value, self.value
        )
    }
}

#[derive(Clone, Debug)]
pub struct Placeholder {
    width: u8,
    glow: bool,
}

impl Placeholder {
    pub fn new(width_percent: u8) -> Self {
        Self {
            width: width_percent.clamp(1, 100),
            glow: false,
        }
    }

    pub fn glow(mut self, glow: bool) -> Self {
        self.glow = glow;
        self
    }
}

impl Render for Placeholder {
    fn render(&self) -> String {
        let mut classes = String::from("placeholder");
        if self.glow {
            classes.push_str(" placeholder-glow");
        }
        format!(
            "<span class=\"{classes} w-100 col-{}\" style=\"width: {}%;\" aria-hidden=\"true\"></span>",
            self.width, self.width
        )
    }
}
