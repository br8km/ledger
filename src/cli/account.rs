extern crate serde_yaml;

use clap::error::Result;
use crate::ledger::LedgerFile;


pub fn account(filename: &str) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_accounts(deserialized_file);

    Ok(())
}
