//! Top-level application and associated utilities.

use crate::err::SysErr;

/// Application entry-point.
#[derive(Clone, Debug)]
pub struct App;

impl App {
    /// Create a new instance.
    pub fn new() -> Self {
        Self
    }
}

impl App {
    /// Run the application logic.
    pub fn run(&self) -> Result<(), SysErr> {
        Ok(())
    }
}
