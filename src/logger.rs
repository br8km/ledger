extern crate simplelog;
extern crate time;


use simplelog::*;

use std::fs::File;

use time::macros::format_description;
use std::{thread, time::Duration};


pub struct Record {
  timestamp: u32,
  datetime: String,
  file: String,
}


pub fn init () {

  let file_name = String::from("ledger.log");

    let config = ConfigBuilder::new()
        .set_time_format_custom(format_description!(
            version = 2,
            "[unix_timestamp] - [year]-[month]-[day] [hour]:[minute]:[second]"
        ))
        .build();

    let file = File::options().append(true).open(file_name).unwrap();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, config.clone(),  file),
        ]
    ).unwrap();

}


pub fn load(_file_name: String) {
  panic!("not implement yet.")
}


pub fn parse (_line: &str) -> Record {
  LogMessage{
    timestamp:12345, 
    datetime:String::from("yyyy-mm-dd hh:MM::ss"),
    file:String::from("file.log"),
  }
}

pub fn parsing (_lines: Vec<String>) -> Vec<Record> {
  panic!("not implement yet.")
}

  // pub fn filter_and_ordering (messages: Vec![LogMessage]) -> Vec![LogMessage] {
  //   Vec![]
  // }
