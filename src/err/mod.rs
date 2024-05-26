//! Error conditions and associated utilities.

use std::io::Error as IOError;
use thiserror::Error;

/// Top-level system error.
#[derive(Debug, Error)]
pub enum SysErr {
    /// Error interfacing with the external world.
    #[error("I/O error interfacing with the environment")]
    IOErr(#[from] IOError),
}
