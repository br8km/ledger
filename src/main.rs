

extern crate clap;

use clap::Parser;

use log::{info, trace, warn};

pub mod cli;



fn main() {
    let arguments = cli::args::LedgerArgs::parse();

    trace!("Commencing yak shaving...");
    warn!("Unable to locate a razor: retrying");
    info!("Razor located: ok.");

    println!("{:?}", arguments);

}