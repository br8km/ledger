extern crate thiserror;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {

    #[error("Invalid File History Index: (expected {expected:?}, got {found:?} )")]
    FileIndex {
        expected: String,
        found: String,
    },

    #[error("Invalid File Format: (expected {expected:?} )")]
    FileFormat {
        expected: String,
        // found: String,
    },

    #[error("Bad Argument Usage: {0}")]
    BadArgs(String),

}
