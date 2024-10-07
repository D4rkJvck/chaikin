use std::{thread, time::Duration};

use sdl2::{
    event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::Point,
    render::Canvas, video::Window,
};

use crate::Circle;

const TITLE: &str = "CHAIKIN";
const WIDTH: u32 = 1200;
const HEIGHT: u32 = 720;

/// The Interface between the user
/// and the program.
/// Holds all the necessary tools
/// to interact with the user.
pub struct Interface {
    pub canvas: Canvas<Window>,
    pub event_pump: sdl2::EventPump,
    pub points: Vec<Point>,
}

impl Interface {
    /// This method holds all the logic around the `SDL` implementation.
    /// It initializes the "context" that will be then used to create
    /// a new "window" given a `title` and `dimensions`.
    /// Afterwards it turns the window to a "canvas".
    /// It also initializes a "event pump" from the context
    /// to get the `user` input `events`.
    /// It finally initializes the instance of the `interface``.
    pub fn new() -> Self {
        // Initialize the SDL
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem // Create a window
            .window(TITLE, WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window         // Create a canvas from the window
            .into_canvas()
            .build()
            .unwrap();

        let event_pump = sdl_context        // Initialize an event pump from the context
            .event_pump()
            .unwrap();

        let points: Vec<Point> = Vec::new();                // Initialize

        Interface {
            canvas,
            event_pump,
            points,
        }
    }

    /// This method is responsible for anything related to the `events`.
    /// It play a crucial role regarding the interaction between the
    /// user and the program. It is also the unique conveyor of 
    /// the main function for the user's desire to `exit` the program.
    pub fn running(&mut self) -> Result<(), String> {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                // Quit the application wether by
                // clicking on the window's `X` button
                Event::Quit { .. }
                // or by pressing the [ Escape ] key
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => return Err("Exiting...".to_string()),

                // Mark control points on the canvas
                // by clicking on the `Left` mouse button.
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => self.add_point(x, y)?,

                // Start the curve transition by
                // pressing the [ Enter ] key.
                Event::KeyDown {
                    keycode: Some(Keycode::RETURN),
                    ..
                } => self.animate()?,

                // Intended to clear the window
                // so the user can mark new control points.
                Event::KeyDown {
                    keycode: Some(Keycode::SPACE),
                    ..
                } => self.clear(),

                _ => {}
            };
        }

        Ok(())
    }

    /// A shortcut for not only avoid repetition of process
    /// but also secure the direct access to the structure's fields.
    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.canvas.clear();
    }

    /// Acts like the application's state when it comes
    /// to the points that have been marked by the user.
    /// Add each point in the application's points
    /// to have them stored somewhere for later rerendering.
    fn add_point(&mut self, x: i32, y: i32) -> Result<(), String> {
        let point = Point::new(x, y);
        self.points.push(point);
        self.display(None)?;

        Ok(())
    }

    /// Responsible for all the display and rendering of the appliction.
    /// from the control points to their marker, through the lines between
    /// the points generated by the Chaikin algorithm.
    fn display(&mut self, polyline: Option<&Vec<Point>>) -> Result<(), String> {
        // Set the color and draw the control points.
        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        self.canvas.draw_points(self.points.as_slice())?;

        // Draw the control point's marker (circle).
        self.points.iter().for_each(|p| {
            let circle = Circle::new(*p);
            circle.draw(&mut self.canvas);
        });

        // Draw lines from the first point of the given vector.
        if let Some(p) = polyline {
            for i in 0..p.len() - 1 {
                self.canvas.draw_line(p[i], p[i + 1])?;
            }
        }

        self.canvas.present();

        Ok(())
    }

    /// This method might be the most important, when it comes to
    /// the main objective of this application.
    /// It runs the animation that update the polyline transformation
    /// to a curved line within 7 steps of rerendering.
    /// It also allow the input events handling to keep going by
    /// calling the running method within the animation loop.
    fn animate(&mut self) -> Result<(), String> {
        let mut polyline: Vec<Point> = self.points.clone();         // Copy the Interface's store as starting state

        match polyline.len() {
            0 | 1 => {}
            2 => self.display(Some(&polyline))?,
            _ => {
                let mut steps = 7;

                while steps > 0 {
                    self.clear();
                    self.display(Some(&polyline))?;

                    self.running()?;                                // Keep handling inputs events

                    let mut temp = vec![polyline[0]];   // Store the first control point to a temporary buffer

                    for i in 0..polyline.len() - 1 {
                        let p0 = polyline[i];
                        let p1 = polyline[i + 1];

                        let q = Point::new((p0.x * 3 + p1.x) / 4, (p0.y * 3 + p1.y) / 4);
                        let r = Point::new((p0.x + p1.x * 3) / 4, (p0.y + p1.y * 3) / 4);

                        temp.extend_from_slice(&[q, r]);            // Collect the generated point's pair.
                    }

                    temp.push(*polyline.last().unwrap());           // Add the last point of the store
                    polyline = temp;                                // then debuff the final content to the initial polyline.

                    steps -= 1;
                    thread::sleep(Duration::from_millis(500)); // Slow down the animation to have a better look at the transitions
                }

                self.animate()?;
            }
        };

        Ok(())
    }
}
