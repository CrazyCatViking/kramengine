use updateable::Updateable;

mod renderer;
mod renderable;
mod input;
mod interactable;
mod updateable;
mod square;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem.window("Kramengine", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = renderer::Renderer::new(window).unwrap();
    let mut square = square::Square::new(400.0, 300.0, 100, 100);

    let mut input_handler = input::InputHandler::new(&sdl_context);
    let should_exit = false;

    let mut ticks_count = 0;

    while !should_exit {
        input_handler.handle_events(&mut square);

        let delta_time = (timer_subsystem.ticks() - ticks_count) as f64 / 1000.0;
        ticks_count = timer_subsystem.ticks();

        square.update(delta_time);

        renderer.clear();
        renderer.submit(&square);
        renderer.present();
    }
}
