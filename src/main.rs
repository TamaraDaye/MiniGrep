extern crate mini_grep;

use std::env;
use std::process;


use mini_grep::Config;


fn main() {
    //store command line arguments
    let args: Vec<String> = env::args().collect();

    //set string search and file to search
    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    if let Err(e) = mini_grep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }

}



//struct to hold command line args
