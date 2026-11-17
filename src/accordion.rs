use crate::render::Render;

#[derive(Clone, Debug)]
pub struct AccordionItem {
    title: String,
    body: String,
}

#[derive(Clone, Debug, Default)]
pub struct Accordion {
    items: Vec<AccordionItem>,
    open_first: bool,
}

impl Accordion {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, title: impl Into<String>, body: impl Into<String>) -> Self {
        self.items.push(AccordionItem {
            title: title.into(),
            body: body.into(),
        });
        self
    }

    pub fn open_first(mut self, open_first: bool) -> Self {
        self.open_first = open_first;
        self
    }
}

impl Render for Accordion {
    fn render(&self) -> String {
        let mut out = String::from("<div class=\"accordion\" id=\"matuiAccordion\">");
        for (i, item) in self.items.iter().enumerate() {
            let id = format!("accordion-item-{i}");
            let expanded = self.open_first && i == 0;
            let button_classes = if expanded {
                "accordion-button"
            } else {
                "accordion-button collapsed"
            };
            let collapse_classes = if expanded {
                "accordion-collapse collapse show"
            } else {
                "accordion-collapse collapse"
            };
            let aria = if expanded { "true" } else { "false" };
            out.push_str(&format!(
                "<div class=\"accordion-item\"><h2 class=\"accordion-header\"><button class=\"{button_classes}\" type=\"button\" data-bs-toggle=\"collapse\" aria-expanded=\"{aria}\" aria-controls=\"{id}\">{}</button></h2><div id=\"{id}\" class=\"{collapse_classes}\"><div class=\"accordion-body\">{}</div></div></div>",
                item.title, item.body
            ));
        }
        out.push_str("</div>");
        out
    }
}
