use std::{thread, time::Duration};

use drawing::Interface;

pub fn main() {
    let mut window = Interface::new();

    'running: loop {
        if !window.running() {
            break 'running;
        }

        window.clear();
        window.display();

        thread::sleep(Duration::from_millis(16));
    }
}