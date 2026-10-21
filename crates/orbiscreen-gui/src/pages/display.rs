// Orbiscreen - Display & Input Page (GPL-3.0-or-later) https://github.com/shadow-x78/orbiscreen

use std::cell::Cell;
use std::rc::Rc;

use adw::prelude::*;

use crate::app::{AppState, Msg, PageCtx};
use crate::daemon_client::DaemonClient;
use crate::i18n;
use crate::pages::{self as ui};

const RESOLUTIONS: [(u32, u32, &str); 7] = [
    (1920, 1080, "1080p"),
    (2560, 1536, "2.5K"),
    (1920, 1152, "1152p"),
    (2560, 1600, "1600p"),
    (2880, 1800, "1800p"),
    (1280, 768, "768p"),
    (1280, 800, "800p"),
];

const FRAMERATES: [(u32, &str); 4] = [(30, "30 Hz"), (60, "60 Hz"), (90, "90 Hz"), (120, "120 Hz")];

#[derive(Debug)]
pub struct DisplayPage {
    pub root: gtk::Widget,
    res_buttons: Vec<gtk::ToggleButton>,
    fps_buttons: Vec<gtk::ToggleButton>,
    encoder_value: gtk::Label,
    autostart_switch: gtk::Switch,
    updating: Rc<Cell<bool>>,
}

impl DisplayPage {
    pub fn new(ctx: &PageCtx) -> Self {
        let lang = ctx.state.borrow().lang;
        let t = |key: &'static str| i18n::tr(lang, key);
        let updating: Rc<Cell<bool>> = Rc::new(Cell::new(false));

        let (root, content) = ui::shell();

        let geometry = ui::card();
        geometry.append(&ui::card_title(t("resGroup")));

        let resolution_row = gtk::Box::new(gtk::Orientation::Vertical, 6);
        resolution_row.add_css_class("orbi-row");
        resolution_row.append(&row_heading(t("resolution"), t("resDesc")));
        let res_box = gtk::FlowBox::new();
        res_box.set_selection_mode(gtk::SelectionMode::None);
        res_box.set_column_spacing(6);
        res_box.set_row_spacing(6);
        res_box.set_max_children_per_line(4);
        let mut res_buttons: Vec<gtk::ToggleButton> = Vec::new();
        for (w, h, label) in RESOLUTIONS {
            let button = ui::chip_button(label);
            let button_ctx = ctx.clone();
            let siblings = res_buttons.clone();
            let guard = updating.clone();
            button.connect_toggled(move |source| {
                if !source.is_active() || guard.get() {
                    return;
                }
                guard.set(true);
                for other in &siblings {
                    other.set_active(false);
                }
                guard.set(false);
                let fps = button_ctx.state.borrow().status.display_fps;
                button_ctx.state.borrow_mut().status.display_width = w;
                button_ctx.state.borrow_mut().status.display_height = h;
                let tx = button_ctx.tx.clone();
                let rt = button_ctx.rt.clone();
                rt.spawn(async move {
                    let _ = tx
                        .send(Msg::DisplaySet(
                            DaemonClient::set_display_settings(w, h, fps)
                                .await
                                .map(|_| (w, h, fps)),
                        ))
                        .await;
                });
            });
            res_box.insert(&button, -1);
            res_buttons.push(button);
        }
        resolution_row.append(&res_box);
        geometry.append(&resolution_row);

        let fps_row = gtk::Box::new(gtk::Orientation::Vertical, 6);
        fps_row.add_css_class("orbi-row");
        fps_row.append(&row_heading(t("framerate"), t("fpsDesc")));
        let fps_box = gtk::FlowBox::new();
        fps_box.set_selection_mode(gtk::SelectionMode::None);
        fps_box.set_column_spacing(6);
        fps_box.set_row_spacing(6);
        fps_box.set_max_children_per_line(4);
        let mut fps_buttons: Vec<gtk::ToggleButton> = Vec::new();
        for (fps, label) in FRAMERATES {
            let button = ui::chip_button(label);
            let button_ctx = ctx.clone();
            let siblings = fps_buttons.clone();
            let guard = updating.clone();
            button.connect_toggled(move |source| {
                if !source.is_active() || guard.get() {
                    return;
                }
                guard.set(true);
                for other in &siblings {
                    other.set_active(false);
                }
                guard.set(false);
                let (w, h) = {
                    let status = &button_ctx.state.borrow().status;
                    (status.display_width, status.display_height)
                };
                button_ctx.state.borrow_mut().status.display_fps = fps;
                let tx = button_ctx.tx.clone();
                let rt = button_ctx.rt.clone();
                rt.spawn(async move {
                    let _ = tx
                        .send(Msg::DisplaySet(
                            DaemonClient::set_display_settings(w, h, fps)
                                .await
                                .map(|_| (w, h, fps)),
                        ))
                        .await;
                });
            });
            fps_box.insert(&button, -1);
            fps_buttons.push(button);
        }
        fps_row.append(&fps_box);
        geometry.append(&fps_row);

        let encoder_row = ui::spec_row(t("encoder"));
        let encoder_value = ui::value_label("-");
        encoder_row.append(&encoder_value);
        geometry.append(&encoder_row);

        let input = ui::card();
        input.append(&ui::card_title(t("inputGroup")));
        let tiles = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        tiles.set_homogeneous(true);
        for (icon, title, desc) in [
            ("input-touchscreen-symbolic", "touchScreen", "touchDesc"),
            ("input-tablet-symbolic", "stylus", "stylusDesc"),
        ] {
            let tile = feature_tile(icon, t(title), t(desc));
            tiles.append(&tile);
        }
        input.append(&tiles);

        let system = ui::card();
        system.append(&ui::card_title(t("systemGroup")));
        let autostart_row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        autostart_row.add_css_class("orbi-row");
        let autostart_text = gtk::Box::new(gtk::Orientation::Vertical, 2);
        let autostart_title = gtk::Label::new(Some(t("autostart")));
        autostart_title.add_css_class("orbi-value");
        autostart_title.set_halign(gtk::Align::Start);
        let autostart_desc = ui::dim_label(t("autostartDesc"));
        autostart_desc.set_halign(gtk::Align::Start);
        autostart_desc.set_wrap(true);
        autostart_desc.set_xalign(0.0);
        autostart_text.append(&autostart_title);
        autostart_text.append(&autostart_desc);
        autostart_text.set_hexpand(true);
        autostart_row.append(&autostart_text);
        let autostart_switch = gtk::Switch::new();
        autostart_switch.set_valign(gtk::Align::Center);
        autostart_row.append(&autostart_switch);
        system.append(&autostart_row);

        let autostart_ctx = ctx.clone();
        let autostart_guard = updating.clone();
        autostart_switch.connect_active_notify(move |source| {
            if autostart_guard.get() {
                return;
            }
            let enabled = source.is_active();
            match DaemonClient::set_autostart(enabled) {
                Ok(()) => autostart_ctx.state.borrow_mut().autostart = enabled,
                Err(e) => {
                    let _ = autostart_ctx.tx.try_send(Msg::Toast(format!("Error: {e}")));
                    autostart_guard.set(true);
                    source.set_active(!enabled);
                    autostart_guard.set(false);
                }
            }
        });

        content.append(&geometry);
        content.append(&input);
        content.append(&system);

        Self {
            root,
            res_buttons,
            fps_buttons,
            encoder_value,
            autostart_switch,
            updating,
        }
    }

    pub fn update(&self, state: &AppState) {
        self.updating.set(true);

        for (index, (w, h, _label)) in RESOLUTIONS.iter().enumerate() {
            self.res_buttons[index]
                .set_active(*w == state.status.display_width && *h == state.status.display_height);
        }
        for (index, (fps, _label)) in FRAMERATES.iter().enumerate() {
            self.fps_buttons[index].set_active(*fps == state.status.display_fps);
        }
        self.autostart_switch.set_active(state.autostart);

        self.updating.set(false);

        let encoder = if state.status.encoder.is_empty() {
            "-".to_string()
        } else {
            state.status.encoder.clone()
        };
        self.encoder_value.set_label(&encoder);
    }

    pub fn set_autostart(&self, enabled: bool) {
        self.updating.set(true);
        self.autostart_switch.set_active(enabled);
        self.updating.set(false);
    }
}

fn row_heading(title: &str, subtitle: &str) -> gtk::Widget {
    let box_widget = gtk::Box::new(gtk::Orientation::Vertical, 2);
    let title_label = gtk::Label::new(Some(title));
    title_label.add_css_class("orbi-value");
    title_label.set_halign(gtk::Align::Start);
    let subtitle_label = ui::dim_label(subtitle);
    subtitle_label.set_halign(gtk::Align::Start);
    box_widget.append(&title_label);
    box_widget.append(&subtitle_label);
    box_widget.upcast()
}

fn feature_tile(icon: &str, title: &str, desc: &str) -> gtk::Widget {
    let tile = gtk::Box::new(gtk::Orientation::Vertical, 10);
    tile.add_css_class("orbi-tile");
    let header = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    header.append(&ui::icon_chip(icon, "green"));
    let spacer = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header.append(&spacer);
    let status = ui::status_icon("ok");
    header.append(&status);
    tile.append(&header);
    let title_label = gtk::Label::new(Some(title));
    title_label.add_css_class("orbi-value");
    title_label.set_halign(gtk::Align::Start);
    tile.append(&title_label);
    let desc_label = ui::dim_label(desc);
    desc_label.set_halign(gtk::Align::Start);
    desc_label.set_xalign(0.0);
    desc_label.set_wrap(true);
    tile.append(&desc_label);
    tile.upcast()
}
