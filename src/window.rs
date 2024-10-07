
use std::{thread, process, time::Duration};

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, video::Window,
};

use crate::Circle;

const TITLE: &str = "CHAIKIN";
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 720;
pub struct Interface {
    pub canvas: Canvas<Window>,
    pub event_pump: sdl2::EventPump,
    pub points: Vec<Point>,
}

impl Interface {
    /// This method holds all the logic around the SDL implementation.
    /// It initializes the context that will be then used to create
    /// a new window given a title and dimensions.
    /// Afterwards it turns the window to a canvas.
    /// It also initializes a event pump to get the user input events.
    /// It finally initializes the instance of the window.
    pub fn new() -> Self {
        // Initialize the SDL
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Create a window
        let window = video_subsystem
            .window(TITLE, WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        // Create a canvas from the window
        let canvas = window.into_canvas().build().unwrap();

        // Initialize an event pump from the context
        let event_pump = sdl_context.event_pump().unwrap();

        // Initialize
        let points: Vec<Point> = Vec::new();

        Interface {
            canvas,
            event_pump,
            points,
        }
    }

    /// A shortcut for not only avoid repetition of process
    /// but also secure the direct access to the the structure's
    /// fields.
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.canvas.clear();
    }

    /// Acts like the application's state when it comes
    /// to the points that have been marked by the user.
    /// Add each point in the application's points
    /// to have them stored somewhere for later rerendering.
    pub fn add_point(&mut self, x: i32, y: i32) -> Result<(), String> {
        let point = Point::new(x, y);
        self.points.push(point);
        self.display(None)?;

        Ok(())
    }

    /// Responsible for the stored control points' display at  each iteration.
    fn display(&mut self, polyline: Option<&Vec<Point>>) -> Result<(), String> {
        // Set the colot and draw the control points.
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
    pub fn animate(&mut self) -> Result<(), String> {
        let mut polyline: Vec<Point> = self.points.clone();

        match polyline.len() {
            0 | 1 => {}
            2 => self.display(Some(&polyline))?,
            _ => {
                let mut steps = 7;

                while steps > 0 {
                    self.clear();
                    self.display(Some(&polyline))?;

                    let events: Vec<Event> = self.event_pump.poll_iter().collect();
                    for event in events {
                        if let Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } = event
                        {
                            process::exit(0)
                        }
                    }

                    let mut temp = vec![polyline[0]];

                    for i in 0..polyline.len() - 1 {
                        let p0 = polyline[i];
                        let p1 = polyline[i + 1];

                        let q = Point::new((p0.x * 3 + p1.x) / 4, (p0.y * 3 + p1.y) / 4);
                        let r = Point::new((p0.x + p1.x * 3) / 4, (p0.y + p1.y * 3) / 4);

                        temp.extend_from_slice(&[q, r]);
                    }

                    temp.push(*polyline.last().unwrap());
                    polyline = temp;

                    steps -= 1;
                    thread::sleep(Duration::from_millis(500));
                }

                self.animate()?;
            }
        };

        Ok(())
    }
}
