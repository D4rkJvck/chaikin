use crate::drawing::*;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};

const TITLE: &str = "CHAIKIN";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
pub struct Interface {
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    points: Vec<Point>,
}

impl Interface {
    pub fn new() -> Self {
        // Initialize the SDL
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // Create a window
        let window = video_subsystem
            .window(TITLE, WIDTH, HEIGHT)
            .fullscreen()
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

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    pub fn display(&mut self) {
        self.canvas.present();
    }

    pub fn running(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                _ => {}
            }
        }

        true
    }

    pub fn get_points(&self) -> &Vec<Point> {
        &self.points
    }

    pub fn place_ctrl_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn draw_new_point(&mut self, point: Point, position: usize) {
        self.points.insert(position, point);
    }
}
