use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

const SHAPE: &[&str] = &[
    "#...........",
    "##..........",
    "#+#.........",
    "#++#........",
    "#+++#.......",
    "#++++#......",
    "#+++++#.....",
    "#++++++#....",
    "#+++++++#...",
    "#++++++++#..",
    "#+++++++++#.",
    "#+++++#####.",
    "#++#++#.....",
    "#+#.#++#....",
    "##..#++#....",
    "#...#++#....",
    "....#++#....",
    ".....#++#...",
    "......##....",
];

const OUTLINE: [u8; 4] = [0, 0, 0, 255];
const FILL: [u8; 4] = [255, 255, 255, 255];

#[derive(Debug)]
pub struct CursorOverlay {
    x: AtomicI32,
    y: AtomicI32,
    visible: AtomicBool,
}

impl Default for CursorOverlay {
    fn default() -> Self {
        Self {
            x: AtomicI32::new(0),
            y: AtomicI32::new(0),
            visible: AtomicBool::new(false),
        }
    }
}

impl CursorOverlay {
    pub fn place(&self, x: i32, y: i32) {
        self.x.store(x, Ordering::Relaxed);
        self.y.store(y, Ordering::Relaxed);
        self.visible.store(true, Ordering::Relaxed);
    }

    pub fn hide(&self) {
        self.visible.store(false, Ordering::Relaxed);
    }

    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::Relaxed)
    }

    pub fn position(&self) -> (i32, i32) {
        (
            self.x.load(Ordering::Relaxed),
            self.y.load(Ordering::Relaxed),
        )
    }

    pub fn composite(&self, data: &mut [u8], width: u32, height: u32) {
        if !self.is_visible() || width == 0 || height == 0 {
            return;
        }
        let (origin_x, origin_y) = self.position();
        let expected = (width as usize)
            .checked_mul(height as usize)
            .and_then(|pixels| pixels.checked_mul(4));
        if expected != Some(data.len()) {
            return;
        }
        for (row, line) in SHAPE.iter().enumerate() {
            let py = origin_y + row as i32;
            if py < 0 || py >= height as i32 {
                continue;
            }
            for (col, glyph) in line.bytes().enumerate() {
                let px = origin_x + col as i32;
                if px < 0 || px >= width as i32 {
                    continue;
                }
                let color = match glyph {
                    b'#' => OUTLINE,
                    b'+' => FILL,
                    _ => continue,
                };
                let index = (py as usize * width as usize + px as usize) * 4;
                data[index..index + 4].copy_from_slice(&color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bgra_frame(width: u32, height: u32) -> Vec<u8> {
        vec![0u8; (width as usize) * (height as usize) * 4]
    }

    #[test]
    fn a_hidden_overlay_leaves_the_frame_untouched() {
        let overlay = CursorOverlay::default();
        let mut frame = bgra_frame(64, 48);
        let before = frame.clone();
        overlay.composite(&mut frame, 64, 48);
        assert_eq!(frame, before);
    }

    #[test]
    fn a_placed_overlay_paints_the_pointer_at_the_hotspot() {
        let overlay = CursorOverlay::default();
        overlay.place(10, 10);
        let mut frame = bgra_frame(64, 48);
        overlay.composite(&mut frame, 64, 48);
        let index = (10 * 64 + 10) * 4;
        assert_eq!(&frame[index..index + 4], &OUTLINE);
        assert!(frame.iter().any(|b| *b != 0));
    }

    #[test]
    fn the_overlay_is_clipped_at_the_frame_edges() {
        let overlay = CursorOverlay::default();
        overlay.place(-3, -3);
        let mut frame = bgra_frame(8, 8);
        overlay.composite(&mut frame, 8, 8);
        let overlay = CursorOverlay::default();
        overlay.place(7, 7);
        overlay.composite(&mut frame, 8, 8);
    }
}
