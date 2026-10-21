// Orbiscreen - Brand Theme (GPL-3.0-or-later) https://github.com/shadow-x78/orbiscreen

use adw::prelude::*;

pub const DARK_DEFINES: &str = "
@define-color orbi_bg #11111B;
@define-color orbi_surface #181825;
@define-color orbi_surface_variant #1E1E2E;
@define-color orbi_primary #89B4FA;
@define-color orbi_on_primary #11111B;
@define-color orbi_secondary #B4BEFE;
@define-color orbi_green #A6E3A1;
@define-color orbi_red #F38BA8;
@define-color orbi_amber #FAB387;
@define-color orbi_text #CDD6F4;
@define-color orbi_dim #A6ADC8;
@define-color orbi_outline alpha(#89B4FA, 0.20);
@define-color orbi_hairline alpha(#A6ADC8, 0.10);
@define-color orbi_shadow alpha(#000000, 0.35);
";

pub const LIGHT_DEFINES: &str = "
@define-color orbi_bg #EFF1F5;
@define-color orbi_surface #FFFFFF;
@define-color orbi_surface_variant #E6E9EF;
@define-color orbi_primary #1E66F5;
@define-color orbi_on_primary #FFFFFF;
@define-color orbi_secondary #7287FD;
@define-color orbi_green #40A02B;
@define-color orbi_red #D20F39;
@define-color orbi_amber #FE640B;
@define-color orbi_text #1E1E2E;
@define-color orbi_dim #5B5F6E;
@define-color orbi_outline #CCD0DA;
@define-color orbi_hairline alpha(#4C4F69, 0.12);
@define-color orbi_shadow alpha(#4C4F69, 0.10);
";

pub const STYLES: &str = r#"
window.background {
    background-color: @orbi_bg;
    color: @orbi_text;
    font-size: 14px;
    font-family: "Noto Sans";
}

headerbar {
    background: @orbi_bg;
    border: none;
    box-shadow: none;
    color: @orbi_text;
    min-height: 56px;
}

headerbar .subtitle {
    color: @orbi_dim;
}

.orbi-tabs {
    background: @orbi_surface_variant;
    border-radius: 999px;
    padding: 4px;
}

.orbi-tabs viewswitcher {
    background: transparent;
}

.orbi-tabs viewswitcher button.toggle {
    background: transparent;
    color: @orbi_dim;
    border-radius: 999px;
    border: none;
    box-shadow: none;
    padding: 0px 18px;
    margin: 0px;
    min-height: 38px;
    font-weight: 600;
    font-size: 13.5px;
}

.orbi-tabs viewswitcher button.toggle:hover {
    color: @orbi_text;
    background: alpha(@orbi_primary, 0.12);
}

.orbi-tabs viewswitcher button.toggle:checked {
    background: @orbi_surface;
    color: @orbi_primary;
    box-shadow: 0 1px 5px @orbi_shadow;
}

.orbi-card {
    background: @orbi_surface;
    border-radius: 20px;
    border: 1px solid @orbi_outline;
    box-shadow: 0 1px 6px @orbi_shadow;
    padding: 20px;
}

.orbi-tile {
    background: @orbi_surface_variant;
    border-radius: 16px;
    padding: 16px;
}

.orbi-card-title {
    color: @orbi_primary;
    font-weight: 800;
    font-size: 12px;
    letter-spacing: 1.2px;
}

.orbi-hero-title {
    font-size: 18px;
    font-weight: 800;
    color: @orbi_text;
}

.orbi-dim {
    color: @orbi_dim;
    font-size: 14px;
}

.orbi-value {
    font-weight: 600;
    color: @orbi_text;
    font-size: 14px;
}

.orbi-mono {
    font-family: monospace;
    font-size: 13.5px;
}

.orbi-pill {
    border-radius: 999px;
    padding: 4px 12px;
    font-weight: 700;
    font-size: 11.5px;
    letter-spacing: 0.8px;
}

.orbi-pill.ok {
    background: alpha(@orbi_green, 0.18);
    color: @orbi_green;
}

.orbi-pill.live {
    background: alpha(@orbi_primary, 0.18);
    color: @orbi_primary;
}

.orbi-pill.off {
    background: alpha(@orbi_red, 0.18);
    color: @orbi_red;
}

.orbi-pill.warn {
    background: alpha(@orbi_amber, 0.18);
    color: @orbi_amber;
}

.orbi-chip-icon {
    background: alpha(@orbi_primary, 0.12);
    border-radius: 16px;
    min-width: 52px;
    min-height: 52px;
    padding: 0px;
}

.orbi-chip-icon image {
    color: @orbi_primary;
}

.orbi-chip-icon.green {
    background: alpha(@orbi_green, 0.15);
}

.orbi-chip-icon.green image {
    color: @orbi_green;
}

.orbi-chip-icon.amber {
    background: alpha(@orbi_amber, 0.15);
}

.orbi-chip-icon.amber image {
    color: @orbi_amber;
}

button.orbi-btn {
    border-radius: 14px;
    border: none;
    box-shadow: none;
    padding: 0px 20px;
    min-height: 48px;
    font-weight: 700;
    font-size: 14px;
    background: @orbi_surface_variant;
    color: @orbi_text;
}

button.orbi-btn:hover {
    background: alpha(@orbi_primary, 0.20);
}

button.orbi-btn.primary {
    background: @orbi_primary;
    color: @orbi_on_primary;
}

button.orbi-btn.primary:hover {
    background: shade(@orbi_primary, 1.08);
}

button.orbi-btn.primary:active {
    background: shade(@orbi_primary, 0.94);
}

button.orbi-btn.danger {
    background: @orbi_red;
    color: @orbi_on_primary;
}

button.orbi-btn.danger:hover {
    background: shade(@orbi_red, 1.06);
}

button.orbi-btn.danger:active {
    background: shade(@orbi_red, 0.94);
}

button.orbi-btn.tonal {
    background: alpha(@orbi_secondary, 0.16);
    color: @orbi_text;
}

button.orbi-btn.tonal:hover {
    background: alpha(@orbi_secondary, 0.24);
}

button.orbi-chip {
    border-radius: 999px;
    padding: 0px 16px;
    min-height: 40px;
    background: @orbi_surface_variant;
    color: @orbi_dim;
    font-weight: 600;
    font-size: 13.5px;
    border: none;
    box-shadow: none;
}

button.orbi-chip:hover {
    background: alpha(@orbi_primary, 0.14);
    color: @orbi_text;
}

button.orbi-chip:checked {
    background: @orbi_primary;
    color: @orbi_on_primary;
}

button.orbi-chip:checked:hover {
    background: shade(@orbi_primary, 1.06);
}

button.orbi-icon-btn {
    background: transparent;
    color: @orbi_dim;
    border-radius: 999px;
    border: none;
    box-shadow: none;
    padding: 0px;
    min-width: 40px;
    min-height: 40px;
}

button.orbi-icon-btn:hover {
    background: @orbi_surface_variant;
    color: @orbi_text;
}

.orbi-row {
    padding: 12px 0px;
}

.orbi-row + .orbi-row {
    border-top: 1px solid @orbi_hairline;
}

.orbi-sep {
    border-top: 1px solid @orbi_hairline;
    margin-top: 8px;
}

.orbi-qr {
    background: #FFFFFF;
    border-radius: 16px;
    padding: 12px;
}

.orbi-num {
    background: alpha(@orbi_primary, 0.12);
    border-radius: 999px;
    min-width: 32px;
    min-height: 32px;
    padding: 0px;
    color: @orbi_primary;
    font-weight: 800;
    font-size: 14px;
}

.orbi-status {
    border-radius: 999px;
    min-width: 28px;
    min-height: 28px;
    padding: 0px;
    background: alpha(@orbi_dim, 0.08);
}

.orbi-status.ok {
    color: @orbi_green;
    background: alpha(@orbi_green, 0.14);
}

.orbi-status.warn {
    color: @orbi_amber;
    background: alpha(@orbi_amber, 0.14);
}

.orbi-status.bad {
    color: @orbi_red;
    background: alpha(@orbi_red, 0.14);
}

switch {
    background: @orbi_surface_variant;
    border-radius: 999px;
    border: none;
    box-shadow: none;
    min-width: 54px;
    min-height: 30px;
}

switch:checked {
    background: @orbi_green;
}

switch slider {
    background: @orbi_dim;
    border-radius: 999px;
    min-width: 26px;
    min-height: 26px;
    margin: 2px;
}

flowboxchild {
    padding: 0px;
}

scrollbar {
    background-color: transparent;
}

scrollbar slider {
    background: alpha(@orbi_dim, 0.25);
    border-radius: 999px;
    min-width: 7px;
    min-height: 7px;
}

popover > contents {
    background: @orbi_surface;
    color: @orbi_text;
    border-radius: 14px;
    border: 1px solid @orbi_outline;
}

popover menuitem:hover {
    background: alpha(@orbi_primary, 0.15);
}
"#;

pub fn stylesheet(dark: bool) -> String {
    let defines = if dark { DARK_DEFINES } else { LIGHT_DEFINES };
    format!("{defines}\n{STYLES}")
}

pub fn attach(window: &adw::ApplicationWindow) {
    let widget = window.upcast_ref::<gtk::Widget>();
    let display = widget.display();
    let provider = gtk::CssProvider::new();
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    let style_manager = adw::StyleManager::default();
    provider.load_from_string(&stylesheet(style_manager.is_dark()));

    let notify_provider = provider.clone();
    style_manager.connect_dark_notify(move |manager| {
        notify_provider.load_from_string(&stylesheet(manager.is_dark()));
    });
}
