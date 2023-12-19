use rand::{rngs::ThreadRng, Rng};
use sdl2::{pixels::Color, event::Event, keyboard::Keycode, render::Canvas, video::Window};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Kramengine", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut should_exit = false;

    let mut rng = rand::thread_rng();

    while !should_exit {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => should_exit = true,
                Event::MouseButtonDown {..} => {
                    set_color(&mut rng, &mut canvas);
                },
                Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _
                } => {
                    match keycode {
                        Some(Keycode::F) => set_color(&mut rng, &mut canvas),
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    canvas.present();
}

fn set_color(rng: &mut ThreadRng, canvas: &mut Canvas<Window>) {
    let r = rng.gen::<u8>();
    let g = rng.gen::<u8>();
    let b = rng.gen::<u8>();

    canvas.set_draw_color(Color::RGB(r, g, b));
    canvas.clear();
    canvas.present();
} 
