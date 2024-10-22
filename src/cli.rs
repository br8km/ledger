
pub mod args;
pub mod logger;


extern crate serde_yaml;

use clap::error::Result;
use crate::ledger::LedgerFile;

pub fn account(filename: &str) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_accounts(deserialized_file);

    Ok(())
}



/// returns balances of all general ledger accounts
pub fn balance(filename: &str) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_balances(deserialized_file);

    Ok(())
}

/// generates budget to actual report for transactions
/// by month or year
pub fn budget(filename: &str, option: &str, group: Group) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    LedgerFile::print_budget_actual(deserialized_file, option, group);

    Ok(())
}

/// returns all general ledger transactions
pub fn register(filename: &str, option: &str, group: Group) -> Result<()> {
    let file = std::fs::File::open(filename)?;
    let deserialized_file: LedgerFile = serde_yaml::from_reader(file).unwrap();

    if group == Group::None {
        LedgerFile::print_register(deserialized_file, option)
    } else {
        LedgerFile::print_register_group(deserialized_file, option, group)
    }

    Ok(())
}

