extern crate mini_gl_fb;
use mini_gl_fb::glutin::VirtualKeyCode;

fn main() {
	let config = mini_gl_fb::Config {
		window_title: "Hello world!",
		window_size: (800.0, 600.0),
		resizable: false,
		buffer_size: (320, 240),
		.. Default::default()
		};
    let mut fb = mini_gl_fb::get_fancy(config);

    let mut noise;
    let mut carry;
    let mut seed = 0xbeefu32;
    let mut buffer = vec![[255u8, 0, 0, 255]; 320 * 240];

    

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
