#![allow(unused_imports, dead_code)]
#![forbid(unsafe_code)]

// extern crate clap;

// use clap::Parser;


pub mod cli;
use cli::logger;

#[macro_use] extern crate log;
// extern crate simplelog;
// extern crate time;


// use simplelog::*;

// use std::fs::File;

// use time::macros::format_description;
// use std::{thread, time::Duration};


fn main() {

    // let arguments = cli::args::LedgerArgs::parse();

    // println!("{:?}", arguments);

    let file_name = String::from("test.log");

    cli::logger::init(file_name);
    error!("Magenta error");
    warn!("Yellow warning");
    info!("Blue info");
    debug!("Cyan debug");
    trace!("Green trace");




}