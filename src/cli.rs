#![allow(dead_code, unused_imports)]


extern crate clap;
extern crate serde_yaml;

use clap::error::Result;
use clap::{Parser, Subcommand};

use crate::error::Error;
use crate::ledger::LedgerFile;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_name = "FILE")]
    pub file: String,

    pub groupby: Option<String>,
    pub recent: Option<String>,
    pub period: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Account,
    Balance,
    Budget,
    Register,
    None,
}


/// returns balances of all general ledger accounts
pub fn balance(filename: &str) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_balances(deserialized_file);

    Ok(())
}

