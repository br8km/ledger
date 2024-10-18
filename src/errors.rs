extern crate thiserror;

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {

    #[error("Invalid File History Index {idx}, expected in range[0, {max}]")]
    FileIndex { idx: usize, max: u8},

    #[error("Invalid Session History Index {idx}, expected in range[0, {max}]")]
    SessionIndex { idx: usize, max: u8},

    // Valid Config File Type: toml;
    // Valid Ledger File Type: yaml;
    #[error("Invalid File Type: (expected {expected:?}, got {found:?})")]
    FileType {
        expected: String,
        found: String,
    },

    #[error("File Path Error")]
    FilePath(#[from] io::Error),
    
    // config, currency, accounts, transactions parsing errors;
    #[error("File Format Error: {0}")]
    FileFormat(String),

    #[error("Unknown File Error")]
    Unknown,

}


#[derive(Error, Debug)]
pub enum ArgumentError {

    #[error("Invalid Argument: {0}")]
    InvalidArgument(String),

    #[error("Missing Argument: {0}")]
    MissingArgument(String),

    #[error("Argument Usage Error: (expected {expected:?}, got {found:?} )")]
    SyntaxError {
        expected: String,
        found: String,
    },

    #[error("Unknown Argument Error")]
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
