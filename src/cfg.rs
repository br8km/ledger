use clap::builder::Str;


pub struct Config {

  currency: String,
  decimal_places: u8,

  theme_color: String,
  theme_font: String,

  budget_alert: u8,
  budget_postpone: bool,

  log_file: String,
  log_limit: usize

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