use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct Carousel {
    slides: Vec<String>,
}

impl Carousel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn slide(mut self, slide: impl Into<String>) -> Self {
        self.slides.push(slide.into());
        self
    }
}

impl Render for Carousel {
    fn render(&self) -> String {
        let mut out = String::from(
            "<div id=\"bsCarousel\" class=\"carousel slide\" data-bs-ride=\"carousel\"><div class=\"carousel-indicators\">",
        );
        for (i, _) in self.slides.iter().enumerate() {
            let active = if i == 0 { " active" } else { "" };
            out.push_str(&format!(
                "<button type=\"button\" data-bs-target=\"#bsCarousel\" data-bs-slide-to=\"{i}\" class=\"{active}\" aria-label=\"Slide {}\"></button>",
                i + 1
            ));
        }
        out.push_str("</div><div class=\"carousel-inner\">");
        for (i, slide) in self.slides.iter().enumerate() {
            let active = if i == 0 { " active" } else { "" };
            out.push_str(&format!(
                "<div class=\"carousel-item{active}\"><div class=\"d-flex align-items-center justify-content-center bg-body-secondary p-5\">{slide}</div></div>"
            ));
        }
        out.push_str("</div>");
        out.push_str("<button class=\"carousel-control-prev\" type=\"button\" data-bs-target=\"#bsCarousel\" data-bs-slide=\"prev\"><span class=\"carousel-control-prev-icon\"></span><span class=\"visually-hidden\">Previous</span></button>");
        out.push_str("<button class=\"carousel-control-next\" type=\"button\" data-bs-target=\"#bsCarousel\" data-bs-slide=\"next\"><span class=\"carousel-control-next-icon\"></span><span class=\"visually-hidden\">Next</span></button>");
        out.push_str("</div>");
        out
    }
}
