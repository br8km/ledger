#![allow(unused_imports, dead_code)]

use clap::{
  Args,
  Parser,
  Subcommand,
};


#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct LedgerArgs {

  // sub commands
  #[clap(subcommand)]
  pub command: Command,


}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// show accounts
    Account(BasicArgs),

    /// show balances by option of group|recent|period;
    Balance(AdvancedArgs),

    /// show budget of accounts
    Budget(AdvancedArgs),

    /// show registers by option of group|recent|period;
    Register(AdvancedArgs),

    /// list de-duplicated history files, in reverse order;
    History,

    /// generate example ledger file in define path;
    Example(BasicArgs),

    /// None command;
    None,
}


/// File Only Args 
#[derive(Debug, Args)]
pub struct  BasicArgs {

  /// file path or history index;
  #[arg(short, long)]
  pub file: String,
}



/// File & Options Args 
#[derive(Debug, Args)]
pub struct AdvancedArgs {

  /// file path or history index;
  #[arg(short, long)]
  pub file: String,

  /// group options, eg: {num}year|month|week|day;
  #[arg(short, long)]
  pub group: Option<String>,

  /// recent options, eg: {num}year|month|week|day;
  #[arg(short, long)]
  pub recent: Option<String>,

  /// period options, eg: {yyyy-mm-dd}-{yyyy-mm-dd}
  #[arg(short, long)]
  pub period: Option<String>,

}


#[derive(Clone, Debug)]
pub enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
}


#[derive(Clone, Debug)]
pub struct Group {
    num: u8,
    unit: TimeUnit,
}


#[derive(Clone, Debug)]
pub struct Recent {
    num: u8,
    unit: TimeUnit,
}

#[derive(Clone)]
pub struct Period {
    start: String,
    end: String,
}




