extern crate thiserror;

use std::io;
use std::result;
use thiserror::Error;


pub type Result<T> = result::Result<T, Error>;


#[derive(Error, Debug)]
pub enum Error {

    #[error("Invalid File History Index {idx}, expected in range[0, {max}]")]
    FileIndexError { idx: usize, max: u8},

    // Valid Ledger File Type: yaml;
    #[error("Invalid File Type: (expected {expected:?}, got {found:?})")]
    FileTypeError {
        expected: String,
        found: String,
    },

    #[error("File Path Error")]
    FilePathError(#[from] io::Error),
    
    // config, currency, accounts, transactions parsing errors;
    #[error("File Format Error: {0}")]
    FileFormatError(String),

    #[error("Invalid Argument: {0}")]
    InvalidArgument(String),

    #[error("Missing Argument: {0}")]
    MissingArgument(String),

    #[error("Argument Usage Error: (expected {expected:?}, got {found:?} )")]
    BadSyntax {
        expected: String,
        found: String,
    },

    #[error("Unknown Error")]
    Unknown,

}


/*

# Possible File Errors/prompt.correct.examples

- File Session Index Error
  - FileSessionIndex NotExist

- File Path Error
  - FileNotExist

- File Type Error
  - Config.toml
  - Ledger.toml

- File Format Error
  - Currency
  - Account
  - Transaction


# Possible Argument Errors

  - `argument redundant, missing, wrong`

- Search.Command
  - File|Session Required
  - Account|Budget -> no option.command Needed
  - Balance|Register -> option.command error

- Config.Command

- Info.Command


# Possible Data Parsing Errors





*/


// Prompt.Examples

// Tests
