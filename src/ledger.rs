#![allow(dead_code, unused_imports)]


use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use prettytable::{format, Table};
use rusty_money::{iso, Money};


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LedgerFile {
    pub currency: String,
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account: String,
    pub amount: f64,
    pub budget_month: Option<f64>,
    pub budget_year: Option<f64>,
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(deserialize_with = "deserialize_date_from_str")]
    pub date: NaiveDate,
    pub account: Option<String>,
    pub amount: Option<f64>,
    pub description: String,
    pub offset_account: Option<String>,
    pub transactions: Option<Vec<TransactionList>>,
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionList {
    pub account: String,
    pub amount: f64,
}

impl LedgerFile {
    /// obtain ISO 4217 currency for reference
    fn get_currency(&self) -> &'static rusty_money::iso::Currency {
        let c = &self.currency;

        match iso::find(c) {
            Some(n) => n,
            None => rusty_money::iso::USD,
        }
    }

}


// Read FileName in history of commands;

// Write FileName in history of commands;

// Custom color theme settings;