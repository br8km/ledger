#![allow(unused_imports, dead_code)]
#![forbid(unsafe_code)]

extern crate clap;
extern crate serde_yaml;

use ledger::LedgerFile;

pub mod ledger;
pub mod args;
pub mod errors;
pub mod logger;

use crate::errors::Result;
use clap::Parser;


fn main() -> Result<()> {

    let args = args::LedgerArgs::parse();

    let filepath = String::from("example.yaml");
    let file = std::fs::File::open(filepath)?;
    // let yaml = fs::read_to_string(filepath).unwrap(); 
    let god: LedgerFile = serde_yaml::from_reader(file).unwrap();

    // println!("{:?}\n", god.config);
    // println!("{:?}\n", god.accounts[0]);
    // println!("{:?}\n", god.transactions[0]);

    // let record = &god.transactions[0].records[0];
    // println!("{:?}\n", record);

    // god.print_history();
    god.run_command(args);
    

    // parse command args & options
    // println!("{:?}\n", args);

    

    Ok(())

}


