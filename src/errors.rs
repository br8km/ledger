extern crate thiserror;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {

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
    
    #[error("Unknown File Error")]
    Unknown,

}


#[derive(Error, Debug)]
pub enum ArgumentError {

    #[error("Bad Argument Usage: {0}")]
    BadArgs(String),
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
