use std::fmt::{Display, Debug};

#[derive(Clone, Copy)]
pub struct Logger;

impl Logger {
    pub fn new() -> Logger {
        Logger{}
    }

    pub fn info<T: Display>(&self, message: T) {
        println!("[INFO] {}", message)
    }

    pub fn error<E: Debug + Display, T: Display>(&self, error: E, message: T) {
        println!("[ERROR] [{}] {}", error, message)
    }
}
