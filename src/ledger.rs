#!allow[unused_imports, dead_code]

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};
use rusty_money::{iso, Money};


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Configs {

  currency: String,
  decimal_places: u8,

  theme_color: String,
  theme_font: String,

  budget_alert: u8,
  budget_postpone: bool,

  log_file: String,
  history_limit: usize

}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account {
  account: String,
  amount: f64,
  budget_month: Option<f64>,
  budget_year: Option<f64>
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction {
  date: NaiveDate,
  amount: f64,
  description: String,
  account_from: String,
  account_to: String
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
      log_file: String::from("ledger.log"),
      log_limit: 10,
      budget_alert: 80,
      budget_postpone: false,
    }
  }

  pub fn read_config (_file_path: &str) -> Config {
    Config::new()
  }

  pub fn write_config (_config: &Config, _file_path: &str) -> bool {
    true
  }

}