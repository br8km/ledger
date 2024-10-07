#![allow(dead_code, unused_imports)]

extern crate clap;

use clap::{Parser, Subcommand};
use crate::error::Error;


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
