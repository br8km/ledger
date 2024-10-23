extern crate simplelog;
extern crate time;

use simplelog::*;

use std::{fs, fs::File};
// use std::str::FromStr;

use time::macros::format_description;
// use std::{thread, time::Duration};
// use core::cmp::Ordering;
use std::cmp::Reverse;

use {
    // once_cell::sync::Lazy,
    regex::Regex,
};


const LOG_FILE: &'static str = "ledger.log";
// const TIME_FORMAT: &'static str = "[unix_timestamp] - [year]-[month]-[day] [hour]:[minute]:[second]";
const LOG_LIMIT: usize = 10;


#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct LogEntry {
  pub timestamp: u32,
  pub datetime: String,
  pub filepath: String,
}


pub fn init () {

  let config = ConfigBuilder::new()
    .set_time_format_custom(
      format_description!(
        version = 2,
        "[unix_timestamp] - [year]-[month]-[day] [hour]:[minute]:[second]"
    ))
    .build();

  let file = File::options().append(true).open(LOG_FILE).unwrap();

  CombinedLogger::init(
  vec![
      TermLogger::new(LevelFilter::Trace, config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
      WriteLogger::new(LevelFilter::Trace, config.clone(),  file),
    ]
  ).unwrap();

}


pub fn parsing  () -> Vec<LogEntry> {
  let mut entries: Vec<LogEntry> = vec![];

  let hay  = fs::read_to_string(LOG_FILE).unwrap();

  let re: Regex = Regex::new(r"(\d{10}) - (\d{4}-\d{2}-\d{2} \d{2}\:\d{2}\:\d{2}) \[INFO\] (\S+)").unwrap();
  // let hay  = "1729433684 - 2024-10-20 14:14:442135376 [INFO] file=file_path_example".to_owned();

  for (_, [timestamp, datetime, filepath]) in re.captures_iter(&hay).map(|c| c.extract()) {
    entries.push(
      LogEntry {
        timestamp: timestamp.parse::<u32>().unwrap(),
        datetime: datetime.to_owned(), 
        filepath: filepath.to_owned() 
      }
    );
    };

  entries

}

pub fn filtering (mut entries: Vec<LogEntry>) -> Vec<LogEntry> {
  entries.sort_by_key(|r| (Reverse(r.filepath.clone()), Reverse(r.timestamp.clone())));
  let mut results: Vec<LogEntry> = vec![];
  let mut fps: Vec<String> = vec![];
  for entry in entries.iter() {
    if !results.contains(&entry) && !fps.contains(&entry.filepath) && results.len() < LOG_LIMIT {
      fps.push(entry.filepath.clone());
      results.push(entry.clone());
    }
  }

  results

}
