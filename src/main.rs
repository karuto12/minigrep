use std::env;
use std::process;
use dotenv::dotenv;

use minigrep::Config;

fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
