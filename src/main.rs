use std::env; // to import strings
use std::process; //help use exit the program without panicking

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect(); //args method - iterator over arguments passed to program, collect - makes a collection

    let config = Config::new(&args).unwrap_or_else(|err| {
        //unwrap_or_else method - makes a Config object
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);
    println!("{:?}", args);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
