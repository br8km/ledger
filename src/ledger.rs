#!allow[unused_imports, dead_code]

use log::info;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

use crate::{args::{AdvancedArgs, BasicArgs}, logger};
// use rusty_money::{iso, Money};



#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Config {

  currency: String,
  decimal_places: u8,

  theme_color: String,
  theme_font: String,

  budget_alert: u8,
  budget_postpone: bool,

}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account {
  account: String,  // account name
  amount: f64,
  budget_month: Option<f64>,
  budget_year: Option<f64>
}



#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Record {
  amount: f64,
  account: String,
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CertainRecord {
  amount: f64,
  account: String,
  date: NaiveDate,
  description: String
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction {
  date: NaiveDate,
  description: String,
  pub records: Vec<Record>
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LedgerFile {
    pub config: Config,
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}


impl Config {

  pub fn new() -> Config {
    Config{
      currency: String::from("USD"),
      decimal_places: 6,
      theme_color: String::from("dark"),
      theme_font: String::from("arial"),
      budget_alert: 80,
      budget_postpone: false,
    }
  }

}

impl LedgerFile {

  pub fn get_currency(&self) -> &'static rusty_money::iso::Currency {
    let c = &self.config.currency;

    match rusty_money::iso::find(c) {
        Some(n) => n,
        None => rusty_money::iso::USD,
    }
  }
  
  /// check records for transaction error
  pub fn check_records(self) -> bool {
    for t in self.transactions {
      let mut total = 0.0;
      for r in t.records {
        total += r.amount;
      }
      if total != 0.0 {
        return false
      }
    }
    return true
  }

  /// flatten abbreviated and detailed `LedgerFile` transactions into
  /// a Vec containing individual detailed transactions.
  /// all downstream logic expects this data structure.
  pub fn flatten_records(self) -> Vec<CertainRecord> {
      let mut records: Vec<CertainRecord> = Vec::new();

      for t in self.transactions {
        for r in t.records {
          let record = CertainRecord{
            date: t.date.clone(),
            amount: r.amount.clone(),
            account: r.account.clone(),
            description: t.description.clone()
          };
          records.push(record);
        }
      }

      records
  }

  pub fn print_account (args: &BasicArgs) {

    // everything done, get filepath logged here
    logger::init();
    info!("{0}", args.file);
  }


  pub fn print_balance (args: &AdvancedArgs) {


    // everything done, get filepath logged here
    logger::init();
    info!("{0}", args.file);
  }


  pub fn print_budget (args: &AdvancedArgs) {


    // everything done, get filepath logged here
    logger::init();
    info!("{0}", args.file);
  }


  /// print out register
  pub fn print_register (args: AdvancedArgs) {


    // process register filtering logic


    // everything done, get filepath logged here
    logger::init();
    info!("{0}", args.file);

  }

  /// print out de-duplicated file history
  pub fn print_history() {
    logger::init();
    let entries = logger::parsing();
    let entries = logger::filtering(entries);
    // for entry in entries {
    //   println!("{0} - {1}", entry.datetime, entry.filepath)
    // }
    for index in 0..entries.len() {
      let entry = &entries[index];
      println!("<{0}> {1} - {2}", index, entry.datetime, entry.filepath)
    }

  }


  /// generate example.yaml file
  pub fn generate_example(_outfile: &str) {

  }



  /// parse filepath if file argument is file index number
  pub fn parse_filepath_from_index_number () -> String {
    panic!("not yet.")
  }


  /// validate filepath exists
  pub fn validate_filepath_exists(_filepath: &str) -> bool {
    true
  }













}






