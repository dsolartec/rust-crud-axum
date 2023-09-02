use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Logger;

impl Logger {
    pub fn new() -> Logger {
        Logger{}
    }

    pub fn info<T: Display>(&self, message: T) {
        println!("[INFO] {}", message)
    }
}
