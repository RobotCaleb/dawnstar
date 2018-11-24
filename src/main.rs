extern crate mini_gl_fb;
extern crate cgmath;
extern crate blit;

use mini_gl_fb::glutin::VirtualKeyCode;
use cgmath::*;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const SCALE: u32 = 4;

struct Player {
    pos: Vector2<i32>,
    size: Vector2<i32>,
    sprite: blit::BlitBuffer,
}

fn main() {
	let config = mini_gl_fb::Config {
		window_title: "Hello world!",
		window_size: ((WIDTH * SCALE) as f64, (HEIGHT * SCALE) as f64),
		resizable: false,
		buffer_size: (WIDTH, HEIGHT),
		.. Default::default()
		};
    let mut fb = mini_gl_fb::get_fancy(config);

    let mut noise: u32;
    let mut carry: u32;
    let mut seed = 0xbeefu32;
    let mut buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];

    // red square
    let sprite = [0xff0000ff_u32; (8 * 8) as usize];
    let mut pl = Player {
        pos: Vector2::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32),
        size: Vector2::new(8, 8),
        sprite: blit::BlitBuffer::from_buffer(&sprite, 8, blit::Color::from_u32(0xff00ff)),
    };

    loop {
        for i in buffer.iter_mut() {
            noise = seed;
            noise >>= 3;
            noise ^= seed;
            carry = noise & 1;
            noise >>= 1;
            seed >>= 1;
            seed |= carry << 30;
            noise &= 0xFF;
            *i = (noise << 16) | (noise << 8) | noise;
        }

        pl.sprite.blit(&mut buffer, WIDTH as usize, (pl.pos.x - pl.size.x / 2, pl.pos.y - pl.size.y / 2));

        fb.update_buffer(&buffer);
        fb.redraw();
        fb.glutin_handle_basic_input(|_fb, input| {
            if input.key_is_down(VirtualKeyCode::Escape) {
                std::process::exit(0);
            }
            if input.key_is_down(VirtualKeyCode::Left) {
                pl.pos.x -= 1;
            }
            if input.key_is_down(VirtualKeyCode::Right) {
                pl.pos.x += 1;
            }
            if input.key_is_down(VirtualKeyCode::Up) {
                pl.pos.y += 1;
            }
            if input.key_is_down(VirtualKeyCode::Down) {
                pl.pos.y -= 1;
            }
            false
        });
    }
}
