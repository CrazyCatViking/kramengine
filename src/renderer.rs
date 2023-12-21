use sdl2::{render::WindowCanvas, video::Window};

use crate::renderable::Renderable;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
    }
    
    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn submit(&mut self, render: &impl Renderable) {
        render.render(&mut self.canvas);
    }
}
