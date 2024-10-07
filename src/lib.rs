pub mod window;

use sdl2::{rect::Point, render::Canvas, video::Window};
pub use window::*;

const RADIUS: i32 = 4;

pub struct Circle {
    center: Point,
}

impl Circle {
    pub fn new(center: Point) -> Self {
        Circle { center }
    }

    pub fn draw(&self, drawer: &mut Canvas<Window>) {
        let mut x = RADIUS;
        let mut y = 0;
        let mut decision_over_2 = 1 - x;

        
        while y <= x {
            drawer
                .draw_point(Point::new(self.center.x + x, self.center.y + y))
                .unwrap();
            drawer
                .draw_point(Point::new(self.center.x - x, self.center.y + y))
                .unwrap();
            drawer
                .draw_point(Point::new(self.center.x + x, self.center.y - y))
                .unwrap();
            drawer
                .draw_point(Point::new(self.center.x - x, self.center.y - y))
                .unwrap();
            drawer
                .draw_point(Point::new(self.center.x + y, self.center.y + x))
                .unwrap();
            drawer
                .draw_point(Point::new(self.center.x - y, self.center.y + x))
                .unwrap();
            drawer
                .draw_point(Point::new(self.center.x + y, self.center.y - x))
                .unwrap();
            drawer
                .draw_point(Point::new(self.center.x - y, self.center.y - x))
                .unwrap();

            y += 1;

            if decision_over_2 <= 0 {
                decision_over_2 += 2 * y + 1;
            } else {
                x -= 1;
                decision_over_2 -= 2 * (y - x) + 1;
            }
        }
    }
}
