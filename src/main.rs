extern crate mini_gl_fb;
use mini_gl_fb::glutin::VirtualKeyCode;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const SCALE: u32 = 3;

fn main() {
	let config = mini_gl_fb::Config {
		window_title: "Hello world!",
		window_size: ((WIDTH * SCALE) as f64, (HEIGHT * SCALE) as f64),
		resizable: false,
		buffer_size: (WIDTH, HEIGHT),
		.. Default::default()
		};
    let mut fb = mini_gl_fb::get_fancy(config);

    let mut noise;
    let mut carry;
    let mut seed = 0xbeefu32;
    let mut buffer = vec![[255u8, 0, 0, 255]; (WIDTH * HEIGHT) as usize];

    

    loop {
        for i in 0..buffer.len() {
            noise = seed;
            noise >>= 3;
            noise ^= seed;
            carry = noise & 1;
            noise >>= 1;
            seed >>= 1;
            seed |= carry << 30;
            noise &= 0xFF;
            buffer[i] = [noise as u8, noise as u8, noise as u8, 255];
        }
        fb.update_buffer(&buffer);
        fb.redraw();
        fb.glutin_handle_basic_input(|_fb, input| {
            if input.key_is_down(VirtualKeyCode::Escape) {
                std::process::exit(0);
            }
            false
        });
    }
}
