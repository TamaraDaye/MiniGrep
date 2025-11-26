use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    //store command line arguments
    let args: Vec<String> = env::args().collect();

    //set string search and file to search
    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    run(config);

}

fn run(config: Config) -> Result<(), Box<Error>>{
    //Open file with a mutable variable
    let mut f = File::open(config.filename)?;

    //variable to hold file contents
    let mut contents = String::new();

    //read data from file as strings
    f.read_to_string(&mut contents)?;
        

    println!("With text:\n{} ", contents);


    Ok(())
}

//struct to hold command line args
struct Config {
    query: String,
    filename: String,
}

impl Config {
    //function to create command line args
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}
