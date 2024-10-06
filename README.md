# chaikin

[![RUST](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)](src/main.rs)

## Table of Contents
- [Overview](#overview)
- [Usage](#usage)
- [Contributors](#contributors)
- [License](#license)
- [SDL2-Documentation-in-Rust](#sdl2-documentation-in-rust)



## Overview

## Usage

## Contributors

[![addiedhiou](https://shields.io/badge/addiedhiou-Zone01-blue)](http://learn.zone01dakar.sn/git/addiedhiou)
[![sefaye](https://shields.io/badge/sefaye-Zone01-blue)](http://learn.zone01dakar.sn/git/sefaye)
[![jefaye](https://shields.io/badge/jefaye-Zone01-blue)](http://learn.zone01dakar.sn/git/jefaye)

## License

[![MIT](https://shields.io/badge/License-MIT-yellow)](LICENSE)



## SDL2-Documentation-in-Rust

## What is SDL2?

SDL2 is a multimedia library that provides a simple interface for managing graphics, sound, user input, and other elements necessary for developing games and multimedia applications. It is widely used to create applications that require low-level access to display, audio, and input devices.

## Installing SDL2 with Rust

To use SDL2 in a Rust project, you need to add the `sdl2` dependency to your `Cargo.toml` file:

```toml
[dependencies]
sdl2 = "0.34"  # Check the latest version on crates.io
```

Next, make sure you have installed the SDL2 libraries on your system. The installation instructions may vary depending on your operating system:

- **On Ubuntu/Debian:**
  ```bash
  sudo apt-get install libsdl2-dev
  ```

- **On macOS:**
  ```bash
  brew install sdl2
  ```

## Initialization

Before you can use SDL2, you need to initialize the library. Here’s an example of code that shows how to do it:

```rust
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    // Create a canvas for rendering
    let mut canvas = window.into_canvas().build().unwrap();

    // Main application loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;  // Update color
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        // Event handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;  // Exit the loop if the user closes the window or presses Escape
                },
                _ => {}
            }
        }

        // Render the display
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));  // Limit to 60 FPS
    }
}
```

## Code Details

### Initialization

1. **`sdl2::init()`**: Initializes the SDL2 subsystems. Here, we use `unwrap()` to handle errors if the initialization fails.
2. **`video_subsystem`**: Retrieves the video subsystem for managing windows and rendering.

### Creating the Window

3. **`window()`**: Creates a new window with the specified title and dimensions. `position_centered()` positions the window in the center of the screen.
4. **`into_canvas()`**: Transforms the window into a canvas for graphical rendering.

### Main Loop

5. **`event_pump`**: Retrieves input events (like key presses and mouse movements).
6. **Loop `'running`**: An infinite loop to run the program as long as the user does not close the window.
   - **Update color**: `i` is incremented and used to change the background color of the window.
   - **Event handling**: Checks for window close events and key presses.
   - **`canvas.present()`**: Updates the display with the current rendering of the canvas.
   - **`sleep()`**: Limits the number of frames per second to 60 using a delay.

### Events

- **`Event::Quit`**: Triggered when the user tries to close the window.
- **`Event::KeyDown`**: Detects if a key is pressed, and here we specifically check if the **Escape** key was pressed to exit the loop.

## Conclusion

SDL2 is a powerful and flexible library that simplifies the development of multimedia applications by providing low-level abstractions for display, audio, and input. By using Rust with SDL2, you can leverage Rust's memory safety and performance while developing games or interactive applications.

For more in-depth information, refer to the [official SDL2 documentation](https://wiki.libsdl.org/) and [crates.io for sdl2](https://crates.io/crates/sdl2) for more examples and available features.
