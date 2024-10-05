#![allow(dead_code)]

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Capitalizes the first character in s.
pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn print_metadata (){
    println!("{name} V{version}", 
        name=capitalize(&NAME),
        version=&VERSION,
    );
    for author in AUTHORS.split(":")  {
        println!("Author: {}", author);

    }
    println!();

}