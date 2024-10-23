// #![allow(unused_imports, dead_code)]
// #![forbid(unsafe_code)]

use ledger::LedgerFile;

pub mod ledger;
pub mod errors;

use crate::errors::Result;


fn main() -> Result<()> {

    let filepath = String::from("ledger.yaml");
    let file = std::fs::File::open(filepath)?;
    // let yaml = fs::read_to_string(filepath).unwrap(); 
    let ledger: LedgerFile = serde_yaml::from_reader(file).unwrap();

    println!("{:?}\n", ledger.config);
    println!("{:?}\n", ledger.accounts[0]);
    println!("{:?}\n", ledger.transactions[0]);

    Ok(())

}


