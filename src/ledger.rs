#!allow[unused_imports, dead_code]

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
// use rusty_money::{iso, Money};


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Config {

  currency: String,
  decimal_places: u8,

  theme_color: String,
  theme_font: String,

  budget_alert: u8,
  budget_postpone: bool,

  file_log: String,
  file_limit: usize

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
  records: Vec<Record>
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
      file_log: String::from("ledger.log"),
      file_limit: 10,
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

}