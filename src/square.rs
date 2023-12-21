use sdl2::{render::WindowCanvas, rect::{Rect, Point}, pixels::Color};

use crate::{renderable::Renderable, interactable::Interactable, updateable::Updateable};

pub struct Square {
    x: f64,
    y: f64,
    w: u32,
    h: u32,

    right_pressed: bool,
    left_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
}

impl Square {
    pub fn new(x: f64, y: f64, w: u32, h: u32) -> Square {
        Square {
            x,
            y,
            w,
            h,
            right_pressed: false,
            left_pressed: false,
            up_pressed: false,
            down_pressed: false
        }
    }
}

impl Renderable for Square {
    fn render(&self, canvas: &mut WindowCanvas) {
        let rect = Rect::from_center(Point::new(self.x as i32, self.y as i32), self.w, self.h);

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(rect).unwrap();
    }
}

impl Updateable for Square {
    fn update(&mut self, delta_time: f64) {
        if self.right_pressed {
            self.x += 100.0 * delta_time
        }
        if self.left_pressed {
            self.x -= 100.0 * delta_time;
        }
        if self.up_pressed {
            self.y -= 100.0 * delta_time;
        }
        if self.down_pressed {
            self.y += 100.0 * delta_time;
        }
    }
}

impl Interactable for Square {
    fn handle_input(&mut self, event: &sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => {
                self.left_pressed = true;
            },
            sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => {
                self.right_pressed = true;
            },
            sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {
                self.up_pressed = true;
            },
            sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => {
                self.down_pressed = true;
            },
            sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => {
                self.left_pressed = false;
            },
            sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => {
                self.right_pressed = false;
            },
            sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => {
                self.up_pressed = false;
            },
            sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => {
                self.down_pressed = false;
            },
            _ => {}
        }
    }
}
