use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const WIN_H: usize = 512;
const WIN_W: usize = 512;
const MAP_W: usize = 16; // map width
const MAP_H: usize = 16; // map heigt
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

fn pack_color(r: u8, g: u8, b: u8, a: Option<u8>) -> u32 {
    match a {
        Some(alpha) => (alpha as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | r as u32,
        None => (255 as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | r as u32,
    }
}

fn unpack_color(color: &u32) -> (u8, u8, u8, u8) {
    let r = ((*color >> 0) & 255) as u8;
    let g = ((*color >> 8) & 255) as u8;
    let b = ((*color >> 16) & 255) as u8;
    let a = ((*color >> 24) & 255) as u8;

    return (r, g, b, a);
}

fn drop_ppm_image(file: &File, buffer: &[u32]) {
    let mut ofs = BufWriter::new(file);
    write!(ofs, "P6\n{} {}\n255\n", WIN_H, WIN_W).unwrap();

    for i in 0..WIN_H * WIN_W {
        let (r, g, b, _) = unpack_color(&buffer[i]);
        ofs.write(&[r, g, b]).unwrap();
    }

    ofs.flush().unwrap();
}

fn fill_rect(buffer: &mut [u32], w: usize, h: usize, x: usize, y: usize, color: u32) {
    for i in 0..w {
        for j in 0..h {
            let cx = x + i;
            let cy = y + j;
            buffer[cx + cy * WIN_W] = color;
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![255; WIN_H * WIN_W];
    for j in 0..WIN_H {
        for i in 0..WIN_W {
            let r = (255 * j / WIN_H) as u8;
            let g = (255 * i / WIN_W) as u8;

            let b = 0;
            buffer[i + j * WIN_W] = pack_color(r, g, b, None);
        }
    }

    assert!(buffer.len() == WIN_H * WIN_W);
    let file_path = Path::new(&"./out.ppm");
    let file = File::create(file_path).unwrap();

    let rect_w = WIN_W / MAP_W;
    let rect_h = WIN_W / MAP_W;
    for i in 0..MAP_W {
        for j in 0..MAP_H {
            if MAP.as_bytes()[i + MAP_H * j] == 32 {
                continue;
            }; //ascii value of space
               //now we need to get the actual pixels, and fill those with a value of 0,255,255
               //the value of the pixel at map coords 3,4 is 3*rectw, 4*recth.
            let map_x_pix = i * rect_w;
            let map_y_pix = j * rect_h;
            fill_rect(
                &mut buffer,
                rect_w,
                rect_h,
                map_x_pix,
                map_y_pix,
                pack_color(0, 255, 255, None),
            );
        }
    }

    let player_x = 3.456;
    let player_y = 2.345;

    let px = (player_x * rect_w as f32) as usize;
    let py = (player_y * rect_h as f32) as usize;

    print!("{} {}", px, py);

    fill_rect(
        &mut buffer,
        5,
        5,
        px,
        py,
        pack_color(0, 255, 255, None),
    );

    drop_ppm_image(&file, &buffer);
    return ();
}
