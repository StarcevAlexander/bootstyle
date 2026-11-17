use crate::render::Render;

#[derive(Clone, Debug)]
pub struct NavbarLink {
    text: String,
    active: bool,
}

#[derive(Clone, Debug)]
pub struct Navbar {
    brand: String,
    links: Vec<NavbarLink>,
    dark: bool,
}

impl Navbar {
    pub fn new(brand: impl Into<String>) -> Self {
        Self {
            brand: brand.into(),
            links: Vec::new(),
            dark: false,
        }
    }

    pub fn link(mut self, text: impl Into<String>) -> Self {
        self.links.push(NavbarLink {
            text: text.into(),
            active: false,
        });
        self
    }

    pub fn active_link(mut self, text: impl Into<String>) -> Self {
        self.links.push(NavbarLink {
            text: text.into(),
            active: true,
        });
        self
    }

    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
}

impl Render for Navbar {
    fn render(&self) -> String {
        let mut classes = String::from("navbar");
        if self.dark {
            classes.push_str(" navbar-dark bg-dark");
        } else {
            classes.push_str(" navbar-light bg-body-tertiary");
        }
        let mut out = format!(
            "<nav class=\"{classes}\"><div class=\"container-fluid\"><a class=\"navbar-brand\" href=\"#\">{}</a><ul class=\"navbar-nav me-auto mb-2 mb-lg-0 d-flex flex-row gap-3\">",
            self.brand
        );
        for link in &self.links {
            let link_classes = if link.active {
                "nav-link active"
            } else {
                "nav-link"
            };
            out.push_str(&format!(
                "<li class=\"nav-item\"><a class=\"{link_classes}\" href=\"#\">{}</a></li>",
                link.text
            ));
        }
        out.push_str("</ul></div></nav>");
        out
    }
}

#[derive(Clone, Debug)]
pub struct NavTabs {
    items: Vec<(String, bool)>,
    pills: bool,
}

impl NavTabs {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            pills: false,
        }
    }

    pub fn tab(mut self, text: impl Into<String>, active: bool) -> Self {
        self.items.push((text.into(), active));
        self
    }

    pub fn pills(mut self, pills: bool) -> Self {
        self.pills = pills;
        self
    }
}

impl Default for NavTabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for NavTabs {
    fn render(&self) -> String {
        let nav_class = if self.pills {
            "nav nav-pills"
        } else {
            "nav nav-tabs"
        };
        let mut out = format!("<ul class=\"{nav_class}\">");
        for (text, active) in &self.items {
            let link_class = if *active {
                "nav-link active"
            } else {
                "nav-link"
            };
            out.push_str(&format!(
                "<li class=\"nav-item\"><a class=\"{link_class}\" href=\"#\">{text}</a></li>"
            ));
        }
        out.push_str("</ul>");
        out
    }
}

#[derive(Clone, Debug)]
pub struct Breadcrumb {
    crumbs: Vec<String>,
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self { crumbs: Vec::new() }
    }

    pub fn crumb(mut self, text: impl Into<String>) -> Self {
        self.crumbs.push(text.into());
        self
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Breadcrumb {
    fn render(&self) -> String {
        let last = self.crumbs.len().saturating_sub(1);
        let mut out = String::from("<nav aria-label=\"breadcrumb\"><ol class=\"breadcrumb\">");
        for (i, crumb) in self.crumbs.iter().enumerate() {
            if i == last {
                out.push_str(&format!(
                    "<li class=\"breadcrumb-item active\" aria-current=\"page\">{crumb}</li>"
                ));
            } else {
                out.push_str(&format!(
                    "<li class=\"breadcrumb-item\"><a href=\"#\">{crumb}</a></li>"
                ));
            }
        }
        out.push_str("</ol></nav>");
        out
    }
}

#[derive(Clone, Debug)]
pub struct PageLink {
    text: String,
    active: bool,
    disabled: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Pagination {
    links: Vec<PageLink>,
}

impl Pagination {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, text: impl Into<String>) -> Self {
        self.links.push(PageLink {
            text: text.into(),
            active: false,
            disabled: false,
        });
        self
    }

    pub fn active(mut self, text: impl Into<String>) -> Self {
        self.links.push(PageLink {
            text: text.into(),
            active: true,
            disabled: false,
        });
        self
    }

    pub fn disabled(mut self, text: impl Into<String>) -> Self {
        self.links.push(PageLink {
            text: text.into(),
            active: false,
            disabled: true,
        });
        self
    }
}

impl Render for Pagination {
    fn render(&self) -> String {
        let mut out = String::from("<nav><ul class=\"pagination\">");
        for link in &self.links {
            let mut classes = String::from("page-link");
            if link.active {
                classes.push_str(" active");
            }
            if link.disabled {
                classes.push_str(" disabled");
            }
            out.push_str(&format!(
                "<li class=\"page-item\"><a class=\"{classes}\" href=\"#\">{}</a></li>",
                link.text
            ));
        }
        out.push_str("</ul></nav>");
        out
    }
}
