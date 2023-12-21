use crate::interactable::Interactable;

pub struct InputHandler {
    event_pump: sdl2::EventPump
}

impl InputHandler {
    pub fn new(sdl_context: &sdl2::Sdl) -> InputHandler {
        let event_pump = sdl_context.event_pump().unwrap();
        InputHandler { event_pump }
    }

    pub fn handle_events(&mut self, interactable: &mut impl Interactable) {
        for event in self.event_pump.poll_iter() {
            interactable.handle_input(&event);

            match event {
                sdl2::event::Event::Quit { .. } => {
                    std::process::exit(0);
                },
                _ => {}
            }
        }
    }
}

