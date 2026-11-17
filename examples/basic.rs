use bootstyle::{
    Accordion, Alert, Badge, Breadcrumb, Button, ButtonGroup, ButtonSize, Card, Carousel, Checkbox,
    Color, Dropdown, Input, ListGroup, Modal, NavTabs, Navbar, Offcanvas, Pagination, Progress,
    Radio, Range, Render, Select, Spinner, Switch, Table, Toast, Tooltip,
};

fn main() {
    let mut page = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>bootstyle gallery</title><style>",
    );
    page.push_str(&bootstyle::css::stylesheet());
    page.push_str("body { padding-bottom: 40px; }");
    page.push_str(".demo { padding: 32px 40px; display: grid; gap: 20px; max-width: 760px; }");
    page.push_str(".demo-row { display: flex; gap: 12px; flex-wrap: wrap; align-items: center; }");
    page.push_str("</style></head><body>");

    page.push_str(
        &Navbar::new("bootstyle")
            .active_link("Home")
            .link("Docs")
            .render(),
    );

    page.push_str("<div class=\"demo\">");

    page.push_str("<h2>Buttons</h2><div class=\"demo-row\">");
    page.push_str(&Button::new("Primary").render());
    page.push_str(&Button::new("Secondary").color(Color::Secondary).render());
    page.push_str(&Button::new("Success").color(Color::Success).render());
    page.push_str(&Button::new("Danger").color(Color::Danger).render());
    page.push_str(&Button::new("Warning").color(Color::Warning).render());
    page.push_str(&Button::new("Info").color(Color::Info).render());
    page.push_str(&Button::new("Disabled").disabled(true).render());
    page.push_str("</div><div class=\"demo-row\">");
    page.push_str(&Button::new("Primary").outline(true).render());
    page.push_str(
        &Button::new("Secondary")
            .outline(true)
            .color(Color::Secondary)
            .render(),
    );
    page.push_str(&Button::new("Large").size(ButtonSize::Large).render());
    page.push_str(&Button::new("Small").size(ButtonSize::Small).render());
    page.push_str("</div>");

    page.push_str("<h2>Button group</h2>");
    page.push_str(
        &ButtonGroup::new()
            .button(Button::new("Left"))
            .button(Button::new("Middle"))
            .button(Button::new("Right"))
            .render(),
    );

    page.push_str("<h2>Badges</h2><div class=\"demo-row\">");
    page.push_str(&Badge::new("Primary", Color::Primary).render());
    page.push_str(&Badge::new("Success", Color::Success).pill(true).render());
    page.push_str(&Badge::new("Danger", Color::Danger).render());
    page.push_str("</div>");

    page.push_str("<h2>Card</h2>");
    page.push_str(
        &Card::new()
            .header("Featured")
            .title("Card title")
            .subtitle("Card subtitle")
            .body("Some quick example text to build on the card title.")
            .footer("2 days ago")
            .render(),
    );

    page.push_str("<h2>Forms</h2>");
    page.push_str(
        &Input::new("Email address", "name@example.com")
            .name("email")
            .render(),
    );
    page.push_str(&Select::new("Category", ["Rust", "Cargo", "Crates"]).render());
    page.push_str(&Checkbox::new("Remember me").checked(true).render());
    page.push_str(
        &Radio::new("plan", "free", "Free plan")
            .checked(true)
            .render(),
    );
    page.push_str(&Radio::new("plan", "pro", "Pro plan").render());
    page.push_str(&Switch::new("Enable notifications").render());
    page.push_str(&Range::new("volume").range(0, 100).value(65).render());

    page.push_str("<h2>Table</h2>");
    page.push_str(
        &Table::new()
            .head(["#", "Component", "Status"])
            .row(["1", "Buttons", "Stable"])
            .row(["2", "Forms", "Stable"])
            .row(["3", "Overlays", "Preview"])
            .striped(true)
            .hover(true)
            .render(),
    );

    page.push_str("<h2>Navigation</h2>");
    page.push_str(
        &Breadcrumb::new()
            .crumb("Home")
            .crumb("Library")
            .crumb("Data")
            .render(),
    );
    page.push_str(
        &NavTabs::new()
            .tab("Home", true)
            .tab("Profile", false)
            .tab("Contact", false)
            .render(),
    );
    page.push_str(
        &Pagination::new()
            .disabled("Previous")
            .active("1")
            .page("2")
            .page("3")
            .page("Next")
            .render(),
    );

    page.push_str("<h2>Feedback</h2>");
    page.push_str(
        &Alert::new(Color::Success, "Everything works!")
            .dismissible(true)
            .render(),
    );
    page.push_str(&Alert::new(Color::Danger, "Something went wrong.").render());
    page.push_str(&Progress::new(65).striped(true).animated(true).render());
    page.push_str("<div class=\"demo-row\">");
    page.push_str(&Spinner::new().render());
    page.push_str(&Spinner::new().color(Color::Danger).grow(true).render());
    page.push_str(&Spinner::new().color(Color::Warning).small(true).render());
    page.push_str(
        &Tooltip::new(
            Button::new("Hover me").outline(true).render(),
            "Pure CSS tooltip",
        )
        .render(),
    );
    page.push_str("</div>");
    page.push_str(
        &Toast::new("bootstyle", "Crate compiled successfully.")
            .time("just now")
            .open(true)
            .render(),
    );

    page.push_str("<h2>List group and accordion</h2>");
    page.push_str(
        &ListGroup::new()
            .active("The current item")
            .item("A second item")
            .disabled("A disabled item")
            .render(),
    );
    page.push_str(
        &Accordion::new()
            .item(
                "What is bootstyle?",
                "Bootstrap 5 components for Rust as typed builders.",
            )
            .item(
                "Do I need JavaScript?",
                "No. Everything renders to plain HTML and CSS.",
            )
            .open_first(true)
            .render(),
    );

    page.push_str("<h2>Dropdown and modal</h2><div class=\"demo-row\">");
    page.push_str(
        &Dropdown::new("Actions")
            .item("Copy")
            .item("Move")
            .item("Delete")
            .danger_item(2)
            .open(true)
            .render(),
    );
    page.push_str("</div>");
    page.push_str(
        &Modal::new("Delete draft?", "This action cannot be undone.")
            .footer(format!(
                "{}{}",
                Button::new("Cancel").color(Color::Secondary).render(),
                Button::new("Delete").color(Color::Danger).render()
            ))
            .open(true)
            .render(),
    );

    page.push_str("<h2>Offcanvas and carousel</h2><div class=\"demo-row\">");
    page.push_str(
        &Offcanvas::new("Filters", "Refine your search here.")
            .open(true)
            .render(),
    );
    page.push_str(
        &Carousel::new()
            .slide("<h3>Slide one</h3><p class=\"mb-0\">Static HTML slides.</p>")
            .slide("<h3>Slide two</h3><p class=\"mb-0\">Indicators and controls.</p>")
            .slide("<h3>Slide three</h3><p class=\"mb-0\">All pure CSS.</p>")
            .render(),
    );
    page.push_str("</div>");

    page.push_str("</div></body></html>");
    println!("{page}");
}
