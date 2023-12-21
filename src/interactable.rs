use sdl2::event::Event;

pub trait Interactable {
    fn handle_input(&mut self, event: &Event);
}
