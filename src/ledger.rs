#!allow[unused_imports, dead_code]

use std::ops::Range;

use log::info;
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

use crate::args::{self, AdvancedArgs, BasicArgs, Command, LedgerArgs};
// use rusty_money::{iso, Money};

use crate::errors::Error;
use crate::logger;

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

  pub fn print_account (self, args: &BasicArgs) {

    // everything done, get filepath logged here
    self.write_log(&args.file);
  }


  pub fn print_balance (self, args: &AdvancedArgs) {


    // everything done, get filepath logged here
    self.write_log(&args.file);
  }


  pub fn print_budget (self, args: &AdvancedArgs) {


    // everything done, get filepath logged here
    self.write_log(&args.file);
  }


  /// print out register
  pub fn print_register (self, args: AdvancedArgs) {


    // process register filtering logic


    // everything done, get filepath logged here
    self.write_log(&args.file);

  }

  pub fn write_log(self, filepath: &str) {
    logger::init();
    info!("{0}", filepath);

  }

  /// print out de-duplicated file history
  pub fn print_history(self) {
    logger::init();
    let entries = logger::parsing();
    let entries = logger::filtering(entries);
    for index in 0..entries.len() {
      let entry = &entries[index];
      println!("<{0}> {1} - {2}", index, entry.datetime, entry.filepath)
    }

  }


  /// generate example.yaml file
  pub fn generate_example(self, _outfile: &str) {

  }


  /// parse real filepath if is file index number
  pub fn real_filepath (filepath: &str) -> String {
    // check if file string is number in range(0, 9)
    let fileindex = filepath.parse::<u8>();
    match fileindex {
      Ok(ok) => {
        let r : Range<u8> = 0..10;
        if r.contains(&ok) {
          // get real file path here
          let ok_usize = ok as usize;
          logger::init();
          let entries = logger::parsing();
          let entries = logger::filtering(entries);
          for index in 0..entries.len() {
            if index == ok_usize {
              let entry = &entries[index];
              println!("{0}, {1}", entry.timestamp, entry.filepath);
              return entry.filepath.to_owned();
            }
          }
        }

      },
      
      // Err(e) => println!("Not Valid FileIndex ({})", e), 
      // Err(e) => Error::FileIndexError { idx: 0, max: 10 }
      // Err(crate::errors::Error::FileIndexError { idx: 0, max: 10 }) => ()
      Err(_e) => panic!("FileIndex Error!")
    }  
    // panic!("not FileIndex.")
    return filepath.to_owned();

  }


  /// validate filepath exists
  pub fn validate_filepath(filepath: &str) -> bool {
    return std::path::Path::new(filepath).exists()
  }


  pub fn run_command(self, args: LedgerArgs) {
    println!("{:?}\n", args.command);
    // let args = matches;
    // match args.command {
    //   Command::Account(_) => {
    //     self.print_account(&args.command);
    //   },
    //   Command::Balance(_) => self.print_balance(&args),
    //   Command::Budget(_) => self.print_budget(&args),
    //   Command::Register(_) => self.print_account(),
    //   Command::History => self.print_account(),
    //   Command::Example(_) => self.print_account(),
    //   Command::None => unreachable!()
    // }

  }











}






