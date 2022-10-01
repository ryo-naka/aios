use super::{frame_buffer::{PixelDrawer, Vector2D, PixelColor, FrameBuffer}, font};

pub const COLOR_WHITE: PixelColor = PixelColor { r: 255, g: 255, b: 255 };
pub const DESKTOP_BG_COLOR: PixelColor = PixelColor {
    r: 45,
    g: 118,
    b: 237,
};
pub const DESKTOP_FG_COLOR: PixelColor = COLOR_WHITE;

pub fn init(pixel_drawer: PixelDrawer) {
    fill_rect(pixel_drawer, Vector2D::new(0, 0),
        Vector2D::new(pixel_drawer.width(), pixel_drawer.height()), DESKTOP_BG_COLOR);

    font::ShinonomeFont.draw_char(pixel_drawer, Vector2D::new(0, 0),
        DESKTOP_FG_COLOR, DESKTOP_BG_COLOR, 'A');
}

pub fn fill_rect(pixel_drawer: PixelDrawer, pos: Vector2D<isize>, size: Vector2D<usize>, color: PixelColor) {
    for dy in 0..size.y {
        for dx in 0..size.x {
            pixel_drawer.draw_pixel(Vector2D::new(pos.x + dx as isize, pos.y + dy as isize), color)
        }
    }
}
