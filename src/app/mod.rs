//! Top-level application and associated utilities.

use crate::err::SysErr;
use clap::{Parser, Subcommand};

/// Application entry-point.
#[derive(Clone, Debug, Parser)]
#[command(about, name("oxo"), version, long_about(None))]
pub struct App {
    /// Command to execute.
    #[command(subcommand)]
    cmd: Cmd,
}

/// Command available to execute.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Subcommand)]
pub enum Cmd {
    /// Launch an interactive read-eval-print-loop.
    Repl,
}

impl App {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::parse()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Run the application logic.
    pub fn run(&self) -> Result<(), SysErr> {
        Ok(())
    }
}
