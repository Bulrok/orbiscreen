// Orbiscreen - QR Code Widget (GPL-3.0-or-later) https://github.com/shadow-x78/orbiscreen

use std::cell::RefCell;
use std::rc::Rc;

use gtk::cairo;
use gtk::prelude::*;

pub struct QrWidget {
    area: gtk::DrawingArea,
    code: Rc<RefCell<Option<qrcode::QrCode>>>,
}

impl QrWidget {
    pub fn new(size: i32) -> Self {
        let code: Rc<RefCell<Option<qrcode::QrCode>>> = Rc::new(RefCell::new(None));
        let area = gtk::DrawingArea::new();
        area.set_size_request(size, size);
        let draw_code = code.clone();
        area.set_draw_func(
            move |_area: &gtk::DrawingArea, cr: &cairo::Context, width: i32, height: i32| {
                let current = draw_code.borrow();
                if let Some(code) = current.as_ref() {
                    draw(cr, width, height, code);
                }
            },
        );
        Self { area, code }
    }

    pub fn widget(&self) -> &gtk::Widget {
        self.area.upcast_ref()
    }

    pub fn set_url(&self, url: &str) {
        let code = qrcode::QrCode::with_error_correction_level(url, qrcode::EcLevel::M).ok();
        *self.code.borrow_mut() = code;
        self.area.queue_draw();
    }
}

fn draw(cr: &cairo::Context, width: i32, height: i32, code: &qrcode::QrCode) {
    let modules = code.width() as f64;
    let quiet = 2.0;
    let total = modules + quiet * 2.0;
    let side = width.min(height) as f64;
    let cell = side / total;
    let origin_x = (width as f64 - side) / 2.0;
    let origin_y = (height as f64 - side) / 2.0;

    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.rectangle(0.0, 0.0, width as f64, height as f64);
    let _ = cr.fill();

    cr.set_source_rgb(0.0, 0.0, 0.0);
    let colors = code.to_colors();
    for (index, color) in colors.iter().enumerate() {
        if !matches!(color, qrcode::Color::Dark) {
            continue;
        }
        let x = index % code.width();
        let y = index / code.width();
        cr.rectangle(
            origin_x + (quiet + x as f64) * cell,
            origin_y + (quiet + y as f64) * cell,
            cell.ceil(),
            cell.ceil(),
        );
    }
    let _ = cr.fill();
}
