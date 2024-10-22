
const CFG_FILE: &'static str = "config.toml";

pub struct Config {

  theme_color: String,
  log_file: String,
  decimal_number: u8,
  history_limit: usize,
  budget_alert: u8,
  budget_postpone: bool

}

// impl Config for struct Config {

// pub fn read_config
// pub fn save_config

// }