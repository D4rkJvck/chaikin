use chaikin::Interface;
use std::process;

/// Make sure to have minimum responsibility
/// while having an essential responsibility.
/// Responsible for exiting the program when needed.
pub fn main() -> Result<(), String> {
    let mut window = Interface::new();

    loop {
        if let Err(_) = window.running() {
            process::exit(0)
        }
    }
}
