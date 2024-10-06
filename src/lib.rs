pub mod drawing;
pub mod window;

pub use drawing::*;
pub use window::*;

pub fn smooth_line(window: &Interface) {
    for (i, p) in window.get_points().iter().enumerate() {}
}
