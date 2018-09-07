extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    let config = Config::new(&args, case_sensitive).unwrap_or_else( |err| {
        println!("Problem while parsing arguments: {}", err);
        process::exit(1);
    });

    match minigrep::run(config) {
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        },
        Ok(_) => {}
    }
    
}