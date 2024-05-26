//! Main module for the application.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use anyhow::Result;
use oxo::app::App;

/// Main entry point.
pub fn main() -> Result<()> {
    App::new().run()?;
    Ok(())
}
