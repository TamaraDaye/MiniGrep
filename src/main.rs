use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    //store command line arguments
    let args: Vec<String> = env::args().collect();

    //set string search and file to search
    let config  = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("in file {}\n",config.filename);


    //Open file with a mutable variable
    let mut f = File::open(config.filename).expect("file not found");
    
    //variable to hold file contents
    let mut contents = String::new();

    //read data from file as strings
    f.read_to_string(&mut contents)
        .expect("something went wrong reading file");

    println!("With text:\n{} ", contents);

    println!("{}", contents);

}

//struct to hold command line args
struct Config {
    query: String, 
    filename: String,
}


impl Config {
    //function to create command line args
    fn new(args: &[String]) -> Config {
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Config{query, filename}
    }
}

