use bootstyle::{
    Accordion, Alert, Badge, Breadcrumb, Button, ButtonGroup, ButtonSize, Card, Checkbox, Color,
    Dropdown, Input, ListGroup, Modal, NavTabs, Navbar, Offcanvas, Pagination, Progress, Radio,
    Render, Select, Spinner, Switch, Table, Toast, Tooltip,
};

#[test]
fn button_renders_bootstrap_classes() {
    let html = Button::new("Save").render();
    assert!(html.contains("btn btn-primary"));
    assert!(html.contains(">Save</button>"));
}

#[test]
fn outline_button_uses_outline_prefix() {
    let html = Button::new("Nope")
        .outline(true)
        .color(Color::Danger)
        .render();
    assert!(html.contains("btn-outline-danger"));
}

#[test]
fn button_sizes() {
    assert!(
        Button::new("L")
            .size(ButtonSize::Large)
            .render()
            .contains("btn-lg")
    );
    assert!(
        Button::new("S")
            .size(ButtonSize::Small)
            .render()
            .contains("btn-sm")
    );
}

#[test]
fn disabled_button_carries_attribute() {
    assert!(
        Button::new("No")
            .disabled(true)
            .render()
            .contains(" disabled")
    );
}

#[test]
fn button_group_wraps_buttons() {
    let html = ButtonGroup::new()
        .button(Button::new("Left"))
        .button(Button::new("Right"))
        .render();
    assert!(html.contains("btn-group"));
    assert_eq!(html.matches("btn btn-primary").count(), 2);
}

#[test]
fn badge_renders_color_and_pill() {
    let html = Badge::new("New", Color::Success).pill(true).render();
    assert!(html.contains("text-bg-success"));
    assert!(html.contains("rounded-pill"));
}

#[test]
fn card_renders_all_sections() {
    let html = Card::new()
        .header("Featured")
        .title("Title")
        .subtitle("Sub")
        .body("Text")
        .footer("Footer")
        .render();
    assert!(html.contains("card-header\">Featured"));
    assert!(html.contains("card-title\">Title</h5>"));
    assert!(html.contains("card-subtitle"));
    assert!(html.contains("card-text\">Text"));
    assert!(html.contains("card-footer"));
}

#[test]
fn input_renders_label_and_control() {
    let html = Input::new("Email", "name@example.com")
        .name("email")
        .render();
    assert!(html.contains("form-label\">Email</label>"));
    assert!(html.contains("placeholder=\"name@example.com\""));
    assert!(html.contains("name=\"email\""));
}

#[test]
fn floating_input_uses_floating_wrapper() {
    let html = Input::new("Email", "name@example.com")
        .floating(true)
        .render();
    assert!(html.contains("form-floating"));
}

#[test]
fn select_wraps_options() {
    let html = Select::new("Pick", ["a", "b"]).render();
    assert!(html.contains("form-select"));
    assert!(html.contains("<option>a</option>"));
    assert!(html.contains("<option>b</option>"));
}

#[test]
fn checkbox_checked_state() {
    let on = Checkbox::new("Remember").checked(true).render();
    assert!(on.contains(" checked"));
    assert!(on.contains("form-check-input"));
}

#[test]
fn radio_uses_group_name() {
    let html = Radio::new("plan", "pro", "Pro").render();
    assert!(html.contains("name=\"plan\""));
    assert!(html.contains("value=\"pro\""));
}

#[test]
fn switch_uses_role() {
    let html = Switch::new("Notify").render();
    assert!(html.contains("form-switch"));
    assert!(html.contains("role=\"switch\""));
}

#[test]
fn table_renders_head_rows_and_modifiers() {
    let html = Table::new()
        .head(["A", "B"])
        .row(["1", "2"])
        .striped(true)
        .hover(true)
        .bordered(true)
        .dark(true)
        .render();
    assert!(html.contains("table table-striped table-hover table-bordered table-dark"));
    assert!(html.contains("<th scope=\"col\">A</th>"));
    assert!(html.contains("<td>2</td>"));
}

#[test]
fn navbar_renders_brand_and_links() {
    let html = Navbar::new("bootstyle")
        .active_link("Home")
        .link("Docs")
        .render();
    assert!(html.contains("navbar-brand\" href=\"#\">bootstyle</a>"));
    assert!(html.contains("nav-link active"));
    assert!(html.contains("bg-body-tertiary"));
    let dark = Navbar::new("b").dark(true).render();
    assert!(dark.contains("navbar-dark bg-dark"));
}

#[test]
fn nav_tabs_mark_active() {
    let html = NavTabs::new()
        .tab("Home", true)
        .tab("Profile", false)
        .render();
    assert!(html.contains("nav-tabs"));
    assert!(html.contains("nav-link active\" href=\"#\">Home</a>"));
    let pills = NavTabs::new().tab("Home", true).pills(true).render();
    assert!(pills.contains("nav-pills"));
}

#[test]
fn breadcrumb_marks_last_as_active() {
    let html = Breadcrumb::new()
        .crumb("Home")
        .crumb("Library")
        .crumb("Data")
        .render();
    assert!(html.contains("breadcrumb-item\"><a href=\"#\">Home</a></li>"));
    assert!(html.contains("breadcrumb-item active\" aria-current=\"page\">Data</li>"));
}

#[test]
fn pagination_states() {
    let html = Pagination::new()
        .disabled("Previous")
        .active("1")
        .page("2")
        .render();
    assert!(html.contains("page-link active\" href=\"#\">1</a>"));
    assert!(html.contains("page-link disabled\" href=\"#\">Previous</a>"));
}

#[test]
fn alert_dismissible_has_close_button() {
    let html = Alert::new(Color::Success, "Works!")
        .dismissible(true)
        .render();
    assert!(html.contains("alert-success alert-dismissible"));
    assert!(html.contains("btn-close"));
}

#[test]
fn toast_hidden_unless_open() {
    let closed = Toast::new("T", "B").render();
    assert!(closed.contains("toast\""));
    let open = Toast::new("T", "B").time("now").open(true).render();
    assert!(open.contains("toast show"));
    assert!(open.contains(">now</small>"));
}

#[test]
fn spinner_variants() {
    let border = Spinner::new().render();
    assert!(border.contains("spinner-border"));
    let grow = Spinner::new().grow(true).small(true).render();
    assert!(grow.contains("spinner-grow-sm"));
}

#[test]
fn progress_renders_width_and_stripes() {
    let html = Progress::new(65).striped(true).animated(true).render();
    assert!(html.contains("width: 65%"));
    assert!(html.contains("progress-bar-striped progress-bar-animated"));
}

#[test]
fn list_group_marks_active_and_disabled() {
    let html = ListGroup::new()
        .active("Current")
        .item("Plain")
        .disabled("Off")
        .render();
    assert!(html.contains("list-group-item active"));
    assert!(html.contains("list-group-item disabled"));
}

#[test]
fn accordion_first_item_expanded() {
    let html = Accordion::new()
        .item("One", "Body one")
        .item("Two", "Body two")
        .open_first(true)
        .render();
    assert!(html.contains("class=\"accordion-button\""));
    assert!(html.contains("collapse show"));
    assert!(html.contains("accordion-button collapsed"));
}

#[test]
fn dropdown_menu_and_danger_item() {
    let html = Dropdown::new("Actions")
        .item("Copy")
        .item("Delete")
        .danger_item(1)
        .open(true)
        .render();
    assert!(html.contains("dropdown-menu show"));
    assert!(html.contains("dropdown-item text-danger"));
    assert!(html.contains("dropdown-toggle"));
}

#[test]
fn modal_open_renders_backdrop() {
    let open = Modal::new("Title", "Body")
        .footer(Button::new("Close").render())
        .open(true)
        .render();
    assert!(open.contains("style=\" display: block;\""));
    assert!(open.contains("modal-backdrop fade show"));
    assert!(open.contains("modal-title\">Title</h5>"));
    let closed = Modal::new("Title", "Body").render();
    assert!(closed.contains("tabindex=\"-1\""));
    assert!(!closed.contains("modal-backdrop"));
}

#[test]
fn offcanvas_show_state() {
    let open = Offcanvas::new("Filters", "Body").open(true).render();
    assert!(open.contains("offcanvas show"));
    let closed = Offcanvas::new("Filters", "Body").render();
    assert!(closed.contains("offcanvas\""));
}

#[test]
fn tooltip_wraps_content_and_bubble() {
    let html = Tooltip::new(Button::new("Hover").render(), "Tip text").render();
    assert!(html.contains("bs-tooltip-wrap"));
    assert!(html.contains("bs-tooltip-bubble\">Tip text</span>"));
}
