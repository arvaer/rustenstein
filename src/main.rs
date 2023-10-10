use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const WIN_W: usize = 1024;
const WIN_H: usize = 512;
const MAP_W: usize = 16; // map width
const MAP_H: usize = 16; // map heigt
const RECT_W: usize = WIN_W / (MAP_W * 2);
const RECT_H: usize = WIN_H / MAP_H;
const FOV: f32 = PI / 3.0;
// TODO: Turn this into an as_bytes()
const MAP: &str = "0000222222220000\
                   1              0\
                   1      11111   0\
                   1     0        0\
                   0     0  1110000\
                   0     3        0\
                   0   10000      0\
                   0   0   11100  0\
                   0   0   0      0\
                   0   0   1  00000\
                   0       1      0\
                   2       1      0\
                   0       0      0\
                   0 0000000      0\
                   0              0\
                   0002222222200000";

struct Player {
    x: f32,
    y: f32,
    a: f32,
}
struct Color {
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

fn pack_color(r: u8, g: u8, b: u8, a: Option<u8>) -> u32 {
    match a {
        Some(alpha) => (alpha as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | r as u32,
        None => (255_u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | r as u32,
    }
}

fn unpack_color(color: &u32) -> (u8, u8, u8, u8) {
    let r = (*color >> 0 & 255) as u8;
    let g = (*color >> 8 & 255) as u8;
    let b = (*color >> 16 & 255) as u8;
    let a = (*color >> 24 & 255) as u8;

    return (r, g, b, a);
}

fn drop_ppm_image(file: &File, buffer: &[u32]) {
    let mut ofs = BufWriter::new(file);
    write!(ofs, "P6\n{} {}\n255\n", WIN_W, WIN_H).unwrap();

    for i in 0..WIN_H * WIN_W {
        let (r, g, b, _) = unpack_color(&buffer[i]);
        ofs.write(&[r, g, b]).unwrap();
    }

    ofs.flush().unwrap();
}

fn fill_rect(buffer: &mut [u32], x: usize, y: usize, w: usize, h: usize, color: u32) {
    for i in 0..w {
        for j in 0..h {
            let cx = x + i;
            let cy = y + j;
            if cx >= WIN_W || cy >= WIN_H {
                continue;
            }
            buffer[cx + cy * WIN_W] = color;
        }
    }
}

fn main() {
    let player: Player = Player {
        x: 3.0,
        y: 2.0,
        a: 0.523,
    };
    let white = Color {
        r: 255,
        g: 255,
        b: 255,
        a: None,
    }.new();
    let cyan = Color {
        r: 0,
        g: 255,
        b: 255,
        a: None,
    }.new();
    let gray = Color {
        r: 128,
        g: 128,
        b: 128,
        a: None,
    }.new();

    let file_path = Path::new(&"./out.ppm");
    let file = File::create(file_path).unwrap();
    let mut buffer: Vec<u32> = vec![white; WIN_H * WIN_W];

    for j in 0..MAP_H {
        for i in 0..MAP_W {
            if MAP.as_bytes()[i + MAP_W * j] == 32 {
                continue;
            };
            let map_x_pix = i * RECT_W;
            let map_y_pix = j * RECT_H;
            fill_rect(
                &mut buffer,
                map_x_pix,
                map_y_pix,
                RECT_W,
                RECT_H,
                pack_color(0, 255, 255, None),
            );
        }
    }

    for t in 0..WIN_W / 2 {
        let mut c = 0.0;
        let angle: f32 = player.a - FOV / 2.0 + FOV * t as f32 / WIN_W as f32;
        while c < 20.0 {
            let cx = player.x + c * angle.cos();
            let cy = player.y + c * angle.sin();

            let px = cx * RECT_W as f32;
            let py = cy * RECT_H as f32;
            buffer[px as usize + WIN_W * py as usize] = gray;

            if cx > MAP_W as f32 || cy > MAP_H as f32 {
                break;
            }
            if MAP.as_bytes()[cx as usize + MAP_H * cy as usize] != 32 {
                let column_height = WIN_H as f32 / c;
                fill_rect(
                    &mut buffer,
                    WIN_W / 2 + t,
                    (WIN_H as f32 / 2.0 - column_height / 2.0) as usize,
                    1,
                    column_height as usize,
                    cyan,
                );
            }
            c += 0.05;
        }
    }

    drop_ppm_image(&file, &buffer);
    return ();
}
