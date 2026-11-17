use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Input {
    label: String,
    name: String,
    placeholder: String,
    value: String,
    floating: bool,
    disabled: bool,
}

impl Input {
    pub fn new(label: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            name: String::new(),
            placeholder: placeholder.into(),
            value: String::new(),
            floating: false,
            disabled: false,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn floating(mut self, floating: bool) -> Self {
        self.floating = floating;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for Input {
    fn render(&self) -> String {
        let name_attr = if self.name.is_empty() {
            String::new()
        } else {
            format!(" name=\"{}\"", self.name)
        };
        let value_attr = if self.value.is_empty() {
            String::new()
        } else {
            format!(" value=\"{}\"", self.value)
        };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        if self.floating {
            format!(
                "<div class=\"form-floating mb-3\"><input type=\"text\" class=\"form-control\" placeholder=\"{placeholder}\"{name_attr}{value_attr}{disabled_attr}><label>{}</label></div>",
                self.label,
                placeholder = self.placeholder
            )
        } else {
            format!(
                "<div class=\"mb-3\"><label class=\"form-label\">{}</label><input type=\"text\" class=\"form-control\" placeholder=\"{placeholder}\"{name_attr}{value_attr}{disabled_attr}></div>",
                self.label,
                placeholder = self.placeholder
            )
        }
    }
}

#[derive(Clone, Debug)]
pub struct Select {
    label: String,
    options: Vec<String>,
    name: String,
    disabled: bool,
}

impl Select {
    pub fn new(
        label: impl Into<String>,
        options: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            label: label.into(),
            options: options.into_iter().map(Into::into).collect(),
            name: String::new(),
            disabled: false,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for Select {
    fn render(&self) -> String {
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        let mut out = format!(
            "<div class=\"mb-3\"><label class=\"form-label\">{}</label><select class=\"form-select\"{disabled_attr}>",
            self.label
        );
        for option in &self.options {
            out.push_str(&format!("<option>{option}</option>"));
        }
        out.push_str("</select></div>");
        out
    }
}

#[derive(Clone, Debug)]
pub struct Checkbox {
    label: String,
    checked: bool,
    disabled: bool,
    inline: bool,
}

impl Checkbox {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            checked: false,
            disabled: false,
            inline: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }
}

impl Render for Checkbox {
    fn render(&self) -> String {
        let checked_attr = if self.checked { " checked" } else { "" };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        let wrapper = if self.inline {
            "form-check form-check-inline"
        } else {
            "form-check"
        };
        format!(
            "<div class=\"{wrapper}\"><input class=\"form-check-input\" type=\"checkbox\"{checked_attr}{disabled_attr}><label class=\"form-check-label\">{}</label></div>",
            self.label
        )
    }
}

#[derive(Clone, Debug)]
pub struct Radio {
    group: String,
    value: String,
    label: String,
    checked: bool,
    inline: bool,
}

impl Radio {
    pub fn new(
        group: impl Into<String>,
        value: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            group: group.into(),
            value: value.into(),
            label: label.into(),
            checked: false,
            inline: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }
}

impl Render for Radio {
    fn render(&self) -> String {
        let checked_attr = if self.checked { " checked" } else { "" };
        let wrapper = if self.inline {
            "form-check form-check-inline"
        } else {
            "form-check"
        };
        format!(
            "<div class=\"{wrapper}\"><input class=\"form-check-input\" type=\"radio\" name=\"{}\" value=\"{}\"{checked_attr}><label class=\"form-check-label\">{}</label></div>",
            self.group, self.value, self.label
        )
    }
}

#[derive(Clone, Debug)]
pub struct Switch {
    label: String,
    checked: bool,
    disabled: bool,
}

impl Switch {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            checked: false,
            disabled: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for Switch {
    fn render(&self) -> String {
        let checked_attr = if self.checked { " checked" } else { "" };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        format!(
            "<div class=\"form-check form-switch\"><input class=\"form-check-input\" type=\"checkbox\" role=\"switch\"{checked_attr}{disabled_attr}><label class=\"form-check-label\">{}</label></div>",
            self.label
        )
    }
}

#[derive(Clone, Debug)]
pub struct Range {
    name: String,
    min: i64,
    max: i64,
    value: i64,
}

impl Range {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            min: 0,
            max: 100,
            value: 50,
        }
    }

    pub fn range(mut self, min: i64, max: i64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = value;
        self
    }
}

impl Render for Range {
    fn render(&self) -> String {
        format!(
            "<div class=\"mb-3\"><label class=\"form-label\">{}</label><input type=\"range\" class=\"form-range\" name=\"{}\" min=\"{}\" max=\"{}\" value=\"{}\"></div>",
            self.name, self.name, self.min, self.max, self.value
        )
    }
}
