extern crate mini_gl_fb;
extern crate cgmath;
extern crate blit;

use mini_gl_fb::glutin as glutin;
use mini_gl_fb::glutin::GlContext;
use std::collections::HashMap;

use cgmath::*;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const SCALE: u32 = 4;

struct Player {
    pos: Vector2<i32>,
    size: Vector2<i32>,
    sprite: blit::BlitBuffer,
}

struct Noise {
    noise: u32,
    carry: u32,
    seed: u32,
}

struct World {
    running: bool,
    pl: Player,
}

fn main() {
    let mut noise = Noise {
        noise: 0,
        carry: 0,
        seed: 0xbeefu32,
    };
    let mut buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];
    create_static(&mut buffer, &mut noise);

    const PL_SIZE: i32 = 16;
    let mut world = World {
        running: true,
        pl: Player {
            pos: Vector2::new((WIDTH / 2) as i32, (HEIGHT / 2) as i32),
            size: Vector2::new(PL_SIZE, PL_SIZE),
            // sprite buffer gets filled as red square
            sprite: blit::BlitBuffer::from_buffer(&[0xff0000ff_u32; (PL_SIZE * PL_SIZE) as usize], PL_SIZE, blit::Color::from_u32(0xff00ff)),
        }
    };

	let config = mini_gl_fb::Config {
		window_title: "Hello world!",
		window_size: ((WIDTH * SCALE) as f64, (HEIGHT * SCALE) as f64),
		resizable: false,
		buffer_size: (WIDTH, HEIGHT),
		.. Default::default()
    };

    let mut fb = mini_gl_fb::get_fancy(config);
    fb.update_buffer(&buffer);

    let mini_gl_fb::GlutinBreakout {
        mut events_loop,
        gl_window,
        mut fb,
    } = fb.glutin_breakout();

    let mut keys = HashMap::new();

    let mut window_focused = false;
    while world.running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => {
                        world.running = false;
                    },
                    glutin::WindowEvent::Resized(_logical_size) => {
                        let _dpi_factor = gl_window.get_hidpi_factor();
                    },
                    glutin::WindowEvent::Focused(focus) => {
                        window_focused = focus;
                        println!("{}", focus);
                    },
                    glutin::WindowEvent::CursorMoved { .. } => {
                    },
                    glutin::WindowEvent::MouseInput { state: _, .. } => {
                        // if state == glutin::ElementState::Pressed {
                        //     mouse_down = true;
                        // } else {
                        //     mouse_down = false;
                        // }
                    },
                    _ => {},
                }
                glutin::Event::DeviceEvent { event, .. } => match event {
                    glutin::DeviceEvent::Key(input) => {
                        if window_focused {
                            populate_keys(&input, &mut keys);
                        }
                    },
                    _ => {},
                },
                _ => {},
            }
        });

        handle_input(&mut world, &keys);

        create_static(&mut buffer, &mut noise);

        world.pl.sprite.blit(&mut buffer, WIDTH as usize, (world.pl.pos.x - world.pl.size.x / 2, world.pl.pos.y - world.pl.size.y / 2));

        render(&mut fb, &mut buffer);
        fb.redraw();
        gl_window.swap_buffers().unwrap();
    }
}

fn populate_keys(input: &glutin::KeyboardInput, keys: &mut HashMap<glutin::VirtualKeyCode, glutin::ElementState>) {
    if input.virtual_keycode.is_some() {
        keys.insert(input.virtual_keycode.unwrap(), input.state);
    }
}

fn is_key_pressed(key: glutin::VirtualKeyCode, keys: &HashMap<glutin::VirtualKeyCode, glutin::ElementState>) -> bool {
    let v = keys.get(&key);
    v == Some(&glutin::ElementState::Pressed)
}

fn handle_input(world: &mut World, keys: &HashMap<glutin::VirtualKeyCode, glutin::ElementState>) {
    if is_key_pressed(glutin::VirtualKeyCode::Escape, &keys) {
        world.running = false;
    }
    if is_key_pressed(glutin::VirtualKeyCode::Left, &keys) {
        world.pl.pos.x -= 1;
    }
    if is_key_pressed(glutin::VirtualKeyCode::Right, &keys) {
        world.pl.pos.x += 1;
    }
    if is_key_pressed(glutin::VirtualKeyCode::Up, &keys) {
        world.pl.pos.y += 1;
    }
    if is_key_pressed(glutin::VirtualKeyCode::Down, &keys) {
        world.pl.pos.y -= 1;
    }
}

fn create_static(buffer: &mut Vec<u32>, noise: &mut Noise) {
    for i in buffer.iter_mut() {
        noise.noise = noise.seed;
        noise.noise >>= 3;
        noise.noise ^= noise.seed;
        noise.carry = noise.noise & 1;
        noise.noise >>= 1;
        noise.seed >>= 1;
        noise.seed |= noise.carry << 30;
        noise.noise &= 0xFF;
        *i = (noise.noise << 16) | (noise.noise << 8) | noise.noise;
    }
}

fn render(fb: &mut mini_gl_fb::Framebuffer, buffer: &Vec<u32>) {
    fb.update_buffer(&buffer);
}
