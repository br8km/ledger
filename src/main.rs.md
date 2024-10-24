
#![allow(unused_imports, dead_code)]
#![forbid(unsafe_code)]

#[macro_use] extern crate log;
// extern crate clap;

// use clap::Parser;


pub mod cli;
pub mod logger;
pub mod ledger;


use {
    once_cell::sync::Lazy,
    regex::Regex,
};


fn some_helper_function(haystack: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"...").unwrap());
    RE.is_match(haystack)
}

fn main() {
    assert!(some_helper_function("abc"));
    assert!(!some_helper_function("ac"));
    assert!(some_helper_function("xbc"));
    assert!(!some_helper_function("xc"));

    let file_log = String::from("ledger.log");
    logger::init(&file_log);
    info!("file_path_x");
    
    let entries = logger::parsing(&file_log);
    println!("{}", entries.len());
    let filtered = logger::filtering(entries, 10);
    println!("{}", filtered.len());
    for entry in filtered.iter() {
        println!("{0}, {1}", entry.timestamp, entry.filepath);
    }

}


fn demo_parse_arguments () {
    // let arguments = cli::args::LedgerArgs::parse();

    // println!("{:?}", arguments);

}


fn demo_logger () {
    // let file_name = String::from("test.log");

    // cli::logger::init(file_name);
    // error!("Magenta error");
    // warn!("Yellow warning");
    // info!("Blue info");
    // debug!("Cyan debug");
    // trace!("Green trace");

}

