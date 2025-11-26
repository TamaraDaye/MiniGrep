use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    //store command line arguments
    let args: Vec<String> = env::args().collect();

    //set string search and file to search
    let (query, file_name) = parse_args(&args);

    println!("Searching for {}", query);
    println!("in file {}\n",file_name);


    //Open file with a mutable variable
    let mut f = File::open(file_name).expect("file not found");
    
    //variable to hold file contents
    let mut contents = String::new();

    //read data from file as strings
    f.read_to_string(&mut contents)
        .expect("something went wrong reading file");

    println!("With text:\n{} ", contents);

    println!("{}", contents);

}


fn parse_args(args: &[String]) -> (&str, &str) {
    let query: &str = &args[1];
    let file_name: &str = &args[2];

    (query, file_name)
}
