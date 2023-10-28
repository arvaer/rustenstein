use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

const WIN_W: usize = 1024;
const RAY_INCREMENT: f32 = 0.01;
const MAX_RAY_DISTANCE: f32 = 20.0;
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

mod color;
use color::*;

struct Player {
    x: f32,
    y: f32,
    a: f32,
}

fn drop_ppm_image(file_path: &Path, buffer: &[u32]) {
    let file = File::create(file_path).unwrap();
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

fn calculate_ray_angle(player_angle: f32, t: usize, half_window_width: f32) -> f32 {
    (player_angle - (FOV / 2.0)) + (FOV * t as f32 / half_window_width)
}

fn is_out_of_map_bounds(cx: f32, cy: f32) -> bool {
    cx >= MAP_W as f32 || cy >= MAP_H as f32 || cx < 0.0 || cy < 0.0
}
fn main() {
    let mut player: Player = Player {
        x: 12.0,
        y: 5.0,
        a: 0.0,
    };
    let mut buffer: Vec<u32> = vec![get_white(); WIN_H * WIN_W];

    for frame in 0..60 {
        let file_path_str = format!("out/{}.ppm", frame);
        let file_path = Path::new(&file_path_str);
        player.a += 2.0 * PI / 360.0;
        for j in 0..MAP_H {
            for i in 0..MAP_W {
                if MAP.as_bytes()[i + MAP_W * j] == 32 {
                    continue;
                };
                let map_x_pix = i * RECT_W;
                let map_y_pix = j * RECT_H;
                let color;
                if MAP.as_bytes()[i + MAP_W * j] == 49 {
                    color = get_red();
                } else if MAP.as_bytes()[i + MAP_W * j] == 50 {
                    color = get_off_white();
                } else if MAP.as_bytes()[i + MAP_W * j] == 51 {
                    color = get_black();
                } else {
                    color = get_gray();
                }
                fill_rect(&mut buffer, map_x_pix, map_y_pix, RECT_W, RECT_H, color);
            }
        }

        for t in 0..WIN_W / 2 {
            let mut ray_distance = 0.0;
            let half_window_width = WIN_W as f32 / 2.0;
            let ray_angle = calculate_ray_angle(player.a, t, half_window_width);

            while ray_distance < MAX_RAY_DISTANCE {
                let cx = player.x + (ray_distance * ray_angle.cos());
                let cy = player.y + (ray_distance * ray_angle.sin());

                let px = cx * RECT_W as f32;
                let py = cy * RECT_H as f32;

                if is_out_of_map_bounds(cx, cy) {
                    break;
                }

                buffer[px as usize + WIN_W * py as usize] = get_gray();

                let map_value = MAP.as_bytes()[cx as usize + MAP_W * cy as usize];
                if map_value != 32 {
                    let color = get_color_from_map_value(map_value);
                    let column_height = (WIN_H as f32 / (ray_distance * (ray_angle - player.a).cos())).abs();

                    fill_rect(
                        &mut buffer,
                        (WIN_W / 2) + t,
                        ((WIN_H as f32 / 2.0) - (column_height / 2.0)) as usize,
                        1,
                        column_height as usize,
                        color,
                    );
                    break;
                }

                ray_distance += RAY_INCREMENT;
            }
        }

        drop_ppm_image(&file_path, &buffer);
        buffer = vec![get_white(); WIN_H * WIN_W];
    }
    return ();
}
