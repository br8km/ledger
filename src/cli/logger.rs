

extern crate simplelog;
extern crate time;


use simplelog::*;

use std::fs::File;

use time::macros::format_description;
use std::{thread, time::Duration};


pub struct LogMessage {
  timestamp: u32,
  datetime: String,
  file: String,
}


pub fn init (file_name: String) {

    let config = ConfigBuilder::new()
        .set_time_format_custom(format_description!(
            version = 2,
            // self.time_format,
            // ""
            // self.time_format
            "[unix_timestamp] - [year]-[month]-[day] [hour]:[minute]:[second][subsecond]"
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


pub fn parse (_line: &str) -> LogMessage {
  LogMessage{
    timestamp:12345, 
    datetime:String::from("yyyy-mm-dd hh:MM::ss"),
    file:String::from("file.log"),
  }
}

pub fn parsing (_lines: Vec<String>) -> Vec<LogMessage> {
  panic!("not implement yet.")
}

  // pub fn filter_and_ordering (messages: Vec![LogMessage]) -> Vec![LogMessage] {
  //   Vec![]
  // }
