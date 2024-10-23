#![allow(unused_imports, dead_code)]
#![forbid(unsafe_code)]

use std::fs;

use ledger::LedgerFile;

pub mod ledger;



fn main() {

    let filepath = String::from("ledger.yaml");
    let yaml = fs::read_to_string(filepath).unwrap(); 
    let ledger: LedgerFile = serde_yaml::from_str(&yaml)?;

    println!("{:?}", ledger);

}


