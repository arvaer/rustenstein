use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const WIN_H: usize = 512;
const WIN_W: usize = 512;

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
    let mut ofs = BufWriter::new(file);
    write!(ofs, "P6\n{} {}\n255\n", WIN_H, WIN_W).unwrap();

    for i in 0..WIN_H * WIN_W {
        let (r, g, b, _) = unpack_color(&buffer[i]);
        ofs.write(&[r, g, b]).unwrap();
    }

    ofs.flush().unwrap();

    return ();
}
