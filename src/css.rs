pub fn stylesheet() -> String {
    String::from(
        r#"
:root {
  --bs-primary: #0d6efd;
  --bs-secondary: #6c757d;
  --bs-success: #198754;
  --bs-danger: #dc3545;
  --bs-warning: #ffc107;
  --bs-info: #0dcaf0;
  --bs-light: #f8f9fa;
  --bs-dark: #212529;
  --bs-body-bg: #ffffff;
  --bs-body-color: #212529;
  --bs-border-color: #dee2e6;
  --bs-border-radius: 0.375rem;
  --bs-font: system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
}

*, *::before, *::after { box-sizing: border-box; }
body {
  margin: 0;
  font-family: var(--bs-font);
  font-size: 1rem;
  line-height: 1.5;
  color: var(--bs-body-color);
  background: var(--bs-body-bg);
}
h1, h2, h3, h4, h5, h6 { margin: 0 0 0.5rem; font-weight: 500; line-height: 1.2; }
p { margin: 0 0 1rem; }
a { color: var(--bs-primary); text-decoration: none; }
a:hover { text-decoration: underline; }

.container { width: 100%; max-width: 1140px; margin: 0 auto; padding: 0 12px; }
.container-fluid { width: 100%; padding: 0 12px; }
.row { display: flex; flex-wrap: wrap; margin: 0 -12px; }
.row > * { padding: 0 12px; }
.col { flex: 1 0 0%; }
.col-md-1 { flex: 0 0 auto; width: 8.33333333%; }
.col-md-2 { flex: 0 0 auto; width: 16.66666667%; }
.col-md-3 { flex: 0 0 auto; width: 25%; }
.col-md-4 { flex: 0 0 auto; width: 33.33333333%; }
.col-md-5 { flex: 0 0 auto; width: 41.66666667%; }
.col-md-6 { flex: 0 0 auto; width: 50%; }
.col-md-7 { flex: 0 0 auto; width: 58.33333333%; }
.col-md-8 { flex: 0 0 auto; width: 66.66666667%; }
.col-md-9 { flex: 0 0 auto; width: 75%; }
.col-md-10 { flex: 0 0 auto; width: 83.33333333%; }
.col-md-11 { flex: 0 0 auto; width: 91.66666667%; }
.col-md-12 { flex: 0 0 auto; width: 100%; }

.btn {
  display: inline-block;
  padding: 0.375rem 0.75rem;
  border: 1px solid transparent;
  border-radius: var(--bs-border-radius);
  font-family: var(--bs-font);
  font-size: 1rem;
  line-height: 1.5;
  text-align: center;
  text-decoration: none;
  cursor: pointer;
  transition: color 0.15s ease, background 0.15s ease, border-color 0.15s ease, box-shadow 0.15s ease;
}
.btn-primary { background: var(--bs-primary); border-color: var(--bs-primary); color: #fff; }
.btn-primary:hover { background: #0b5ed7; border-color: #0a58ca; color: #fff; }
.btn-secondary { background: var(--bs-secondary); border-color: var(--bs-secondary); color: #fff; }
.btn-secondary:hover { background: #5c636a; border-color: #565e64; color: #fff; }
.btn-success { background: var(--bs-success); border-color: var(--bs-success); color: #fff; }
.btn-success:hover { background: #157347; border-color: #146c43; color: #fff; }
.btn-danger { background: var(--bs-danger); border-color: var(--bs-danger); color: #fff; }
.btn-danger:hover { background: #bb2d3b; border-color: #b02a37; color: #fff; }
.btn-warning { background: var(--bs-warning); border-color: var(--bs-warning); color: #000; }
.btn-warning:hover { background: #ffca2c; border-color: #ffc720; color: #000; }
.btn-info { background: var(--bs-info); border-color: var(--bs-info); color: #000; }
.btn-info:hover { background: #31d2f2; border-color: #25cff2; color: #000; }
.btn-light { background: var(--bs-light); border-color: var(--bs-light); color: #000; }
.btn-light:hover { background: #d3d4d5; border-color: #c6c7c8; color: #000; }
.btn-dark { background: var(--bs-dark); border-color: var(--bs-dark); color: #fff; }
.btn-dark:hover { background: #1c1f23; border-color: #1a1e21; color: #fff; }

.btn-outline-primary { color: var(--bs-primary); border-color: var(--bs-primary); background: transparent; }
.btn-outline-primary:hover { background: var(--bs-primary); color: #fff; }
.btn-outline-secondary { color: var(--bs-secondary); border-color: var(--bs-secondary); background: transparent; }
.btn-outline-secondary:hover { background: var(--bs-secondary); color: #fff; }
.btn-outline-success { color: var(--bs-success); border-color: var(--bs-success); background: transparent; }
.btn-outline-success:hover { background: var(--bs-success); color: #fff; }
.btn-outline-danger { color: var(--bs-danger); border-color: var(--bs-danger); background: transparent; }
.btn-outline-danger:hover { background: var(--bs-danger); color: #fff; }
.btn-outline-warning { color: #000; border-color: var(--bs-warning); background: transparent; }
.btn-outline-warning:hover { background: var(--bs-warning); color: #000; }
.btn-outline-info { color: #000; border-color: var(--bs-info); background: transparent; }
.btn-outline-info:hover { background: var(--bs-info); color: #000; }
.btn-outline-light { color: #000; border-color: var(--bs-light); background: transparent; }
.btn-outline-light:hover { background: var(--bs-light); color: #000; }
.btn-outline-dark { color: var(--bs-dark); border-color: var(--bs-dark); background: transparent; }
.btn-outline-dark:hover { background: var(--bs-dark); color: #fff; }

.btn-lg { padding: 0.5rem 1rem; font-size: 1.25rem; border-radius: 0.5rem; }
.btn-sm { padding: 0.25rem 0.5rem; font-size: 0.875rem; border-radius: 0.25rem; }
.btn:disabled { opacity: 0.65; cursor: not-allowed; }
.btn-close { border: none; background: none; font-size: 1rem; cursor: pointer; line-height: 1; padding: 0.25rem 0.5rem; }

.btn-group { display: inline-flex; }
.btn-group > .btn { border-radius: 0; }
.btn-group > .btn:first-child { border-top-left-radius: var(--bs-border-radius); border-bottom-left-radius: var(--bs-border-radius); }
.btn-group > .btn:last-child { border-top-right-radius: var(--bs-border-radius); border-bottom-right-radius: var(--bs-border-radius); }
.btn-group-vertical { display: inline-flex; flex-direction: column; align-items: stretch; }
.btn-group-vertical > .btn { border-radius: 0; }
.btn-group-vertical > .btn:first-child { border-top-left-radius: var(--bs-border-radius); border-top-right-radius: var(--bs-border-radius); }
.btn-group-vertical > .btn:last-child { border-bottom-left-radius: var(--bs-border-radius); border-bottom-right-radius: var(--bs-border-radius); }

.badge { display: inline-block; padding: 0.35em 0.65em; font-size: 0.75em; font-weight: 700; line-height: 1; color: #fff; text-align: center; vertical-align: baseline; border-radius: var(--bs-border-radius); }
.text-bg-primary { background: var(--bs-primary) !important; color: #fff !important; }
.text-bg-secondary { background: var(--bs-secondary) !important; color: #fff !important; }
.text-bg-success { background: var(--bs-success) !important; color: #fff !important; }
.text-bg-danger { background: var(--bs-danger) !important; color: #fff !important; }
.text-bg-warning { background: var(--bs-warning) !important; color: #000 !important; }
.text-bg-info { background: var(--bs-info) !important; color: #000 !important; }
.text-bg-light { background: var(--bs-light) !important; color: #000 !important; }
.text-bg-dark { background: var(--bs-dark) !important; color: #fff !important; }
.rounded-pill { border-radius: 50rem !important; }

.card { display: flex; flex-direction: column; min-width: 0; background: var(--bs-body-bg); border: 1px solid var(--bs-border-color); border-radius: var(--bs-border-radius); }
.card-header { padding: 0.75rem 1rem; border-bottom: 1px solid var(--bs-border-color); }
.card-body { flex: 1 1 auto; padding: 1rem; }
.card-title { margin-bottom: 0.5rem; }
.card-subtitle { margin-top: -0.25rem; }
.card-text { margin-bottom: 1rem; }
.card-footer { padding: 0.75rem 1rem; border-top: 1px solid var(--bs-border-color); }

.form-label { margin-bottom: 0.5rem; }
.form-control {
  display: block;
  width: 100%;
  padding: 0.375rem 0.75rem;
  font-family: var(--bs-font);
  font-size: 1rem;
  color: var(--bs-body-color);
  background: var(--bs-body-bg);
  border: 1px solid #ced4da;
  border-radius: var(--bs-border-radius);
  transition: border-color 0.15s ease, box-shadow 0.15s ease;
}
.form-control:focus { border-color: #86b7fe; outline: none; box-shadow: 0 0 0 0.25rem rgba(13, 110, 253, 0.25); }
.form-control:disabled { opacity: 0.65; }
.form-select { display: block; width: 100%; padding: 0.375rem 0.75rem; font-family: var(--bs-font); font-size: 1rem; color: var(--bs-body-color); background: var(--bs-body-bg); border: 1px solid #ced4da; border-radius: var(--bs-border-radius); }
.form-select:focus { border-color: #86b7fe; outline: none; box-shadow: 0 0 0 0.25rem rgba(13, 110, 253, 0.25); }
.form-check { display: block; min-height: 1.5rem; padding-left: 1.5em; margin-bottom: 0.125rem; }
.form-check-input { margin-left: -1.5em; width: 1em; height: 1em; vertical-align: top; background: var(--bs-body-bg); border: 1px solid rgba(0, 0, 0, 0.25); appearance: none; cursor: pointer; }
.form-check-input[type="checkbox"] { border-radius: 0.25em; }
.form-check-input[type="radio"] { border-radius: 50%; }
.form-check-input:checked { background: var(--bs-primary); border-color: var(--bs-primary); }
.form-check-label { cursor: pointer; }
.form-check-inline { display: inline-block; margin-right: 1rem; }
.form-switch { padding-left: 2.5em; }
.form-switch .form-check-input { width: 2em; margin-left: -2.5em; border-radius: 2em; }
.form-range { width: 100%; height: 1.5rem; appearance: none; background: transparent; }
.form-range:focus { outline: none; }
.form-floating { position: relative; }
.form-floating > label { position: absolute; left: 0.75rem; top: 0.5rem; color: var(--bs-secondary); pointer-events: none; transition: all 0.15s ease; }
.form-floating > .form-control:focus + label,
.form-floating > .form-control:not(:placeholder-shown) + label { top: 0; font-size: 0.75rem; }
.form-floating > .form-control { height: calc(3.5rem + 2px); padding-top: 1.625rem; padding-bottom: 0.625rem; }

.table { width: 100%; margin-bottom: 1rem; color: var(--bs-body-color); border-collapse: collapse; }
.table th, .table td { padding: 0.5rem; border-bottom: 1px solid var(--bs-border-color); }
.table thead th { vertical-align: bottom; border-bottom: 2px solid var(--bs-border-color); }
.table-striped tbody tr:nth-child(odd) { background: rgba(0, 0, 0, 0.03); }
.table-hover tbody tr:hover { background: rgba(0, 0, 0, 0.05); }
.table-bordered th, .table-bordered td { border: 1px solid var(--bs-border-color); }
.table-dark { color: #fff; background: var(--bs-dark); }
.table-dark th, .table-dark td { border-color: #373b3e; }
.table-dark.table-striped tbody tr:nth-child(odd) { background: rgba(255, 255, 255, 0.05); }

.navbar { display: flex; align-items: center; padding: 0.5rem 1rem; flex-wrap: wrap; }
.navbar-brand { font-size: 1.25rem; margin-right: 1rem; white-space: nowrap; }
.navbar-light .navbar-brand { color: rgba(0, 0, 0, 0.9); }
.navbar-dark .navbar-brand { color: #fff; }
.navbar-light .nav-link { color: rgba(0, 0, 0, 0.55); }
.navbar-dark .nav-link { color: rgba(255, 255, 255, 0.55); }
.navbar-light .nav-link:hover, .navbar-light .nav-link.active { color: rgba(0, 0, 0, 0.9); }
.navbar-dark .nav-link:hover, .navbar-dark .nav-link.active { color: #fff; }

.nav { display: flex; flex-wrap: wrap; margin-bottom: 1rem; list-style: none; padding-left: 0; }
.nav-link { display: block; padding: 0.5rem 1rem; color: var(--bs-primary); }
.nav-tabs { border-bottom: 1px solid var(--bs-border-color); }
.nav-tabs .nav-link { margin-bottom: -1px; border: 1px solid transparent; color: var(--bs-body-color); }
.nav-tabs .nav-link:hover { border-color: var(--bs-border-color); }
.nav-tabs .nav-link.active { background: var(--bs-body-bg); border-color: var(--bs-border-color) var(--bs-border-color) var(--bs-body-bg); color: var(--bs-body-color); }
.nav-pills .nav-link { border-radius: var(--bs-border-radius); color: var(--bs-body-color); }
.nav-pills .nav-link.active { background: var(--bs-primary); color: #fff; }

.breadcrumb { display: flex; flex-wrap: wrap; list-style: none; padding: 0.5rem 0; margin: 0; }
.breadcrumb-item + .breadcrumb-item::before { content: "/"; padding: 0 0.5rem; color: var(--bs-secondary); }
.breadcrumb-item.active { color: var(--bs-secondary); }

.pagination { display: flex; list-style: none; padding-left: 0; margin: 0; }
.page-item .page-link { display: block; padding: 0.375rem 0.75rem; color: var(--bs-primary); background: var(--bs-body-bg); border: 1px solid var(--bs-border-color); margin-left: -1px; }
.page-item:first-child .page-link { border-radius: var(--bs-border-radius) 0 0 var(--bs-border-radius); }
.page-item:last-child .page-link { border-radius: 0 var(--bs-border-radius) var(--bs-border-radius) 0; }
.page-item.active .page-link { background: var(--bs-primary); border-color: var(--bs-primary); color: #fff; }
.page-item.disabled .page-link { color: var(--bs-secondary); pointer-events: none; }

.alert { position: relative; padding: 1rem; margin-bottom: 1rem; border: 1px solid transparent; border-radius: var(--bs-border-radius); }
.alert-dismissible { padding-right: 3rem; }
.alert-primary { background: #cfe2ff; border-color: #b6d4fe; color: #084298; }
.alert-secondary { background: #e2e3e5; border-color: #d3d6d8; color: #41464b; }
.alert-success { background: #d1e7dd; border-color: #a3cfbb; color: #0f5132; }
.alert-danger { background: #f8d7da; border-color: #f5c2c7; color: #842029; }
.alert-warning { background: #fff3cd; border-color: #ffecb5; color: #664d03; }
.alert-info { background: #cff4fc; border-color: #b6effb; color: #055160; }
.alert-light { background: var(--bs-light); border-color: var(--bs-border-color); color: #41464b; }
.alert-dark { background: #d3d3d4; border-color: #bcbebf; color: #141619; }
.alert-dismissible .btn-close { position: absolute; top: 1rem; right: 1rem; }

.toast { width: 350px; max-width: 100%; background: rgba(255, 255, 255, 0.85); border: 1px solid rgba(0, 0, 0, 0.175); border-radius: var(--bs-border-radius); pointer-events: auto; }
.toast:not(.show) { display: none; }
.toast-header { display: flex; align-items: center; padding: 0.5rem 0.75rem; border-bottom: 1px solid rgba(0, 0, 0, 0.05); }
.toast-body { padding: 0.75rem; }

.spinner-border { display: inline-block; width: 2rem; height: 2rem; border: 0.25em solid currentColor; border-right-color: transparent; border-radius: 50%; animation: bs-spin 0.75s linear infinite; }
.spinner-border-sm { width: 1rem; height: 1rem; border-width: 0.2em; }
.spinner-grow { display: inline-block; width: 2rem; height: 2rem; background: currentColor; border-radius: 50%; opacity: 0; animation: bs-grow 0.75s linear infinite; }
@keyframes bs-spin { to { transform: rotate(360deg); } }
@keyframes bs-grow { 0% { transform: scale(0); opacity: 0; } 50% { opacity: 1; } 100% { transform: scale(1); opacity: 0; } }

.progress { display: flex; height: 1rem; overflow: hidden; background: var(--bs-light); border-radius: var(--bs-border-radius); }
.progress-bar { display: flex; flex-direction: column; justify-content: center; color: #fff; text-align: center; transition: width 0.3s ease; }
.progress-bar-striped { background-image: linear-gradient(45deg, rgba(255, 255, 255, 0.15) 25%, transparent 25%, transparent 50%, rgba(255, 255, 255, 0.15) 50%, rgba(255, 255, 255, 0.15) 75%, transparent 75%, transparent); background-size: 1rem 1rem; }
.progress-bar-animated { animation: bs-progress 1s linear infinite; }
@keyframes bs-progress { to { background-position: 1rem 0; } }

.placeholder { display: inline-block; min-height: 1em; vertical-align: middle; background: #e9ecef; border-radius: 0.25rem; opacity: 0.5; }
.placeholder-glow .placeholder { animation: bs-glow 1.5s linear infinite; }
@keyframes bs-glow { 50% { opacity: 0.2; } }

.list-group { display: flex; flex-direction: column; border-radius: var(--bs-border-radius); }
.list-group:not(.list-group-flush) { border: 1px solid var(--bs-border-color); }
.list-group-flush { border-radius: 0; }
.list-group-item { position: relative; display: block; padding: 0.5rem 1rem; background: var(--bs-body-bg); border-bottom: 1px solid var(--bs-border-color); color: var(--bs-body-color); }
.list-group-item:last-child { border-bottom: none; }
.list-group-item:first-child { border-top-left-radius: inherit; border-top-right-radius: inherit; }
.list-group-item.active { background: var(--bs-primary); border-color: var(--bs-primary); color: #fff; }
.list-group-item.disabled { color: var(--bs-secondary); pointer-events: none; }

.accordion-item { border: 1px solid var(--bs-border-color); margin-bottom: -1px; background: var(--bs-body-bg); }
.accordion-button { display: flex; align-items: center; justify-content: space-between; width: 100%; padding: 1rem 1.25rem; border: none; background: var(--bs-body-bg); font: inherit; color: var(--bs-body-color); text-align: left; cursor: pointer; }
.accordion-button:not(.collapsed) { background: #e7f1ff; color: var(--bs-primary); }
.accordion-button::after { content: "▾"; }
.accordion-button:not(.collapsed)::after { content: "▴"; }
.accordion-collapse.collapse:not(.show) { display: none; }
.accordion-body { padding: 1rem 1.25rem; }

.dropdown { position: relative; display: inline-block; }
.dropdown-toggle::after { content: "▾"; margin-left: 0.5rem; }
.dropdown-menu { position: absolute; top: 100%; left: 0; min-width: 10rem; padding: 0.5rem 0; margin: 0.125rem 0 0; list-style: none; background: var(--bs-body-bg); border: 1px solid rgba(0, 0, 0, 0.175); border-radius: var(--bs-border-radius); z-index: 10; }
.dropdown-menu:not(.show) { display: none; }
.dropdown-item { display: block; width: 100%; padding: 0.25rem 1rem; color: var(--bs-body-color); background: none; border: none; text-align: left; text-decoration: none; cursor: pointer; }
.dropdown-item:hover { background: var(--bs-light); }
.dropdown-item.text-danger { color: var(--bs-danger); }

.modal { position: fixed; top: 0; left: 0; z-index: 1055; width: 100%; outline: none; }
.modal-dialog { position: relative; width: auto; max-width: 500px; margin: 1.75rem auto; pointer-events: none; }
.modal-content { position: relative; display: flex; flex-direction: column; background: var(--bs-body-bg); border-radius: 0.5rem; box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.15); pointer-events: auto; }
.modal-header { display: flex; align-items: center; justify-content: space-between; padding: 1rem; border-bottom: 1px solid var(--bs-border-color); }
.modal-title { margin: 0; font-size: 1.25rem; }
.modal-body { padding: 1rem; }
.modal-footer { display: flex; flex-wrap: wrap; justify-content: flex-end; gap: 0.25rem; padding: 1rem; border-top: 1px solid var(--bs-border-color); }
.modal-backdrop { position: fixed; top: 0; left: 0; z-index: 1050; width: 100vw; height: 100vh; background: rgba(0, 0, 0, 0.5); }

.offcanvas { position: fixed; bottom: 0; z-index: 1045; display: flex; flex-direction: column; width: 400px; max-width: 100%; background: var(--bs-body-bg); border: 1px solid rgba(0, 0, 0, 0.175); visibility: hidden; }
.offcanvas:not(.show) { transform: translateX(-100%); }
.offcanvas-header { display: flex; align-items: center; justify-content: space-between; padding: 1rem; }
.offcanvas-title { margin: 0; font-size: 1rem; line-height: 1.5; }
.offcanvas-body { flex-grow: 1; padding: 1rem 1rem; }

.bs-tooltip-wrap { position: relative; display: inline-flex; cursor: default; }
.bs-tooltip-bubble {
  visibility: hidden;
  position: absolute;
  bottom: calc(100% + 6px);
  left: 50%;
  transform: translateX(-50%);
  padding: 0.25rem 0.5rem;
  border-radius: var(--bs-border-radius);
  background: rgba(0, 0, 0, 0.85);
  color: #fff;
  font-family: var(--bs-font);
  font-size: 0.75rem;
  white-space: nowrap;
  z-index: 20;
}
.bs-tooltip-wrap:hover .bs-tooltip-bubble { visibility: visible; }

.carousel { position: relative; }
.carousel-indicators { position: absolute; right: 0; bottom: 0; left: 0; display: flex; justify-content: center; padding: 0; margin-bottom: 1rem; z-index: 2; }
.carousel-indicators [data-bs-target] { width: 30px; height: 3px; margin: 0 3px; border: none; background: rgba(255, 255, 255, 0.5); border-radius: 2px; cursor: pointer; }
.carousel-indicators .active { background: #fff; }
.carousel-inner { position: relative; width: 100%; overflow: hidden; }
.carousel-item { position: relative; display: none; width: 100%; }
.carousel-item.active { display: block; }
.carousel-control-prev, .carousel-control-next { position: absolute; top: 0; bottom: 0; width: 15%; border: none; background: none; cursor: pointer; z-index: 1; display: flex; align-items: center; justify-content: center; }
.carousel-control-prev { left: 0; }
.carousel-control-next { right: 0; }
.carousel-control-prev-icon::before { content: "‹"; font-size: 2rem; color: var(--bs-dark); }
.carousel-control-next-icon::before { content: "›"; font-size: 2rem; color: var(--bs-dark); }

.text-primary { color: var(--bs-primary) !important; }
.text-secondary { color: var(--bs-secondary) !important; }
.text-success { color: var(--bs-success) !important; }
.text-danger { color: var(--bs-danger) !important; }
.text-warning { color: #b98900 !important; }
.text-info { color: #0aa2c0 !important; }
.text-muted { color: var(--bs-secondary) !important; }
.bg-primary { background: var(--bs-primary) !important; color: #fff; }
.bg-secondary { background: var(--bs-secondary) !important; color: #fff; }
.bg-success { background: var(--bs-success) !important; color: #fff; }
.bg-danger { background: var(--bs-danger) !important; color: #fff; }
.bg-warning { background: var(--bs-warning) !important; color: #000; }
.bg-info { background: var(--bs-info) !important; color: #000; }
.bg-light { background: var(--bs-light) !important; color: #000; }
.bg-dark { background: var(--bs-dark) !important; color: #fff; }
.bg-body-secondary { background: var(--bs-light) !important; }
.lead { font-size: 1.25rem; font-weight: 300; }
.fw-bold { font-weight: 700 !important; }
.mb-2 { margin-bottom: 0.5rem !important; }
.mb-3 { margin-bottom: 1rem !important; }
.mt-3 { margin-top: 1rem !important; }
.p-3 { padding: 1rem !important; }
.gap-2 { gap: 0.5rem !important; }
.gap-3 { gap: 1rem !important; }
.d-flex { display: flex !important; }
.visually-hidden { position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; border: 0; }
"#,
    )
}
