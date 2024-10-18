#![allow(dead_code, unused_imports)]

extern crate clap;

use clap::{Parser, Subcommand};
use crate::error::Error;




#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[arg(short, long, value_name = "FILE")]
    pub file: Option(String),

    // group by: year|month|week|day
    pub group: Option<String>,

    // recent {num}year|month|week|day
    pub recent: Option<u8, String>,

    // period from_datetime, to_datetime
    pub period: Option<String, String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Account,
    Balance,
    Budget,
    Register,
    Config,
    List,
    None,
}
