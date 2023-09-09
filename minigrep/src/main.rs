use std::process;
use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}",err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){ //We don't care about the result state, so we check only on Err
        eprintln!("Application error: {}",e);
        process::exit(1);
    }
}
