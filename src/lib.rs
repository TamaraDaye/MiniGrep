use std::error::Error;
use std::fs::File;
use std::io::prelude::*;



pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    //function to create command line args
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    //Open file with a mutable variable
    let mut f = File::open(config.filename)?;

    //variable to hold file contents
    let mut contents = String::new();

    //read data from file as strings
    f.read_to_string(&mut contents)?;
        

    println!("With text:\n{} ", contents);


    Ok(())
}
