# chaikin

[![RUST](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)](src/main.rs)

## Table of Contents
- [Overview](#overview)
- [Installation](#installation)
    - [Cloning](#cloning)
    - [SDL2 library](#sdl2-library)
        - [Description](#description)
        - [Installing](#installing)
- [Usage](#usage)
    - [Initialization](#initialization)
    - [Window](#window)
    - [Events](#events)
    - [Control Points](#control-points)
    - [Algorithm](#algorithm)
- [Contributors](#contributors)
- [License](#license)
- [SDL2-Documentation-in-Rust](#sdl2-documentation-in-rust)



## Overview

## Installation

### Cloning

```shell
$ git clone http://learn.zone01dakar.sn/git/addiedhiou/chaikin.git

$ cd chaikin
```

### SDL2 Library

Before running the program, the sdl2 library needs to be installed.

#### Description

SDL2 (Simple DirectMedia Layer) is a multimedia library that provides a simple interface for managing graphics, sound, user input, and other elements necessary for developing games and multimedia applications. It is widely used to create applications that require low-level access to display, audio, and input devices.

#### Installing

Make sure the SDL2 libraries is intalled on the system. The installation instructions may vary depending on the operating system:

- **On Ubuntu/Debian:**
  ```bash
  sudo apt-get install libsdl2-dev
  ```

- **On macOS:**
  ```bash
  brew install sdl2
  ```

To use SDL2 in a Rust project, the `sdl2` dependency needs to be added to the `Cargo.toml` file:

```toml
[dependencies]
sdl2 = "0.34" // Check for later verions
```

## Usage

### Initialization
Firstly, the sdl2 needs to be initialized using the **`init()`** function of the **`sdl2`** library to get a subsystem layer context.  
Then the video subsystem responsible for managing the windows and rendering is retrieved from the resulting context via the **`video()`** method.  
At this point the program has everything it needs to create windows and canvas.

### Window

A window can be initialized using the **`window()`** method on the video subsystem retrived from the subsystem context. The method takes the specified title and dimensions as arguments. To center the window on the screen, there is the **`position_centered()`** method. then the **`build()`** method is used to create the window.  
And finally, the window needs to be transformed into a canvas for graphical rendering via the **`into_canvas()`** method on the window intance.

### Events

Some events are to be handled to provide a user interaction.  
To begin with a event pump is initialized from the context using the **`event_pump()`** method. This event pump act like a pipeline transferring the user input events from the sdl context events queue to the program.  

An infinite loop si then triggered to run the program as long as the user does not close the window.  
Within the loop, the program can continuously listen to input events and handle them.  
Some of those input events can be:
- **`Event::Quit`**: Triggered when the user tries to close the window.
- **`Event::KeyDown`**: Detects if a key is pressed, and here we specifically check if the **Escape** key was pressed to exit the loop.

   - **`canvas.present()`**: Updates the display with the current rendering of the canvas.
   - **`sleep()`**: Limits the number of frames per second to 60 using a delay.  



To begin, run the program.

```rust
$ cargo run
```

this will open a new centered window.  
Next, **`left click`** anywhere on the inner window to place a **`control point`**.

### Control Points

Control points are selected pixels of the canvas surrounded by a **small cicle**. They are consecutively joined together with a **line** in order to form a polyline that will be then progressively curved through a **`7 steps`** loop usinf the **`Chaikin`** Algorithm.

### Algorithm

The **`Chaikin`** algorithm is pretty simple in its definition as it's about adding **`new control points`** between the previous ones at a ratio of **`25%`** and **`75%`** of each segment (line) 's size:  

```rust
    
```

This process is repeated **`7 times`** applying the same rules and as a result the polyline transitions to a curved line.  
After those **`7 steps`** transition, the process restarts with the initial polyline.

Control Point are what 

## Contributors

[![addiedhiou](https://shields.io/badge/addiedhiou-Zone01-blue)](http://learn.zone01dakar.sn/git/addiedhiou)
[![sefaye](https://shields.io/badge/sefaye-Zone01-blue)](http://learn.zone01dakar.sn/git/sefaye)
[![jefaye](https://shields.io/badge/jefaye-Zone01-blue)](http://learn.zone01dakar.sn/git/jefaye)

## License

[![MIT](https://shields.io/badge/License-MIT-yellow)](LICENSE)



## SDL2-in-Rust

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


## Conclusion

SDL2 is a powerful and flexible library that simplifies the development of multimedia applications by providing low-level abstractions for display, audio, and input. By using Rust with SDL2, you can leverage Rust's memory safety and performance while developing games or interactive applications.

For more in-depth information, refer to the [official SDL2 documentation](https://wiki.libsdl.org/) and [crates.io for sdl2](https://crates.io/crates/sdl2) for more examples and available features.
