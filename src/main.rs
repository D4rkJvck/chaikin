use chaikin::Interface;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton};

pub fn main() -> Result<(), String> {
    let mut window = Interface::new();

    loop {
        // window.clear();

        let events: Vec<Event> = window.event_pump.poll_iter().collect();

        for event in events {
            match event {
                // Quit the application wether by
                // clicking on the window's `X` button
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                // or by pressing the [ Escape ] key
                | Event::Quit { .. } => return Ok(()),

                // Mark control points on the canvas
                // by clicking on the `Left` mouse button.
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => if let Err(e) = window.add_point(x, y) {
                    println!("Oups! {e}")
                }

                // Start the curve transition by
                // pressing the [ Enter ] key.
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => if let Err(e) = window.animate() {
                    println!("Oups! {e}")
                }

                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } =>{
                    println!("Clearing...");
                    window.clear()
                }

                _ => {}
            };
        }
    }
}
