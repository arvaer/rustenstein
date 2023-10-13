pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>,
}

impl Color {
    fn new(&self) -> u32 {
        match self.a {
            Some(alpha) => {
                (alpha as u32) << 24 | (self.b as u32) << 16 | (self.g as u32) << 8 | self.r as u32
            }
            None => (255_u32) << 24 | (self.b as u32) << 16 | (self.g as u32) << 8 | self.r as u32,
        }
    }
}

pub fn unpack_color(color: &u32) -> (u8, u8, u8, u8) {
    let r = (*color >> 0 & 255) as u8;
    let g = (*color >> 8 & 255) as u8;
    let b = (*color >> 16 & 255) as u8;
    let a = (*color >> 24 & 255) as u8;

    return (r, g, b, a);
}
pub fn get_color_from_map_value(value: u8) -> u32 {
    match value {
        49 => get_red(),
        50 => get_off_white(),
        51 => get_black(),
        _ => get_gray(),
    }
}
pub fn get_white() -> u32 {
    Color {
        r: 255,
        g: 255,
        b: 255,
        a: None,
    }
    .new()
}

pub fn get_off_white() -> u32 {
    Color {
        r: 255,
        g: 255,
        b: 200,
        a: None,
    }
    .new()
}
pub fn get_gray() -> u32 {
    Color {
        r: 128,
        g: 128,
        b: 128,
        a: None,
    }
    .new()
}
pub fn get_red() -> u32 {
    Color {
        r: 255,
        g: 0,
        b: 0,
        a: None,
    }
    .new()
}
pub fn get_black() -> u32 {
    Color {
        r: 0,
        g: 0,
        b: 0,
        a: None,
    }
    .new()
}
