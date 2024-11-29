use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::{BufRead, BufReader};



fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        println!("The file path is argument is: {}", args[1]);
        let file = File::open(&args[1]);
        match file {
            Ok(file) => {
                let reader = BufReader::new(file);    
                for (index, line) in reader.lines().enumerate() {
                    let line = line; // Ignore errors.
                    match line {
                        Ok(line) => {
                                // Show the line and its number.
                            println!("{:4}. \t {}", index + 1, line);
                        }
                        Err(error) => {
                            eprintln!("Read Error {}", error);
                        }
                    }    
                }
            }
            Err(error) =>  { 
                eprintln!("Open Error: {} not found!", args[1]);
            }
        };
    
        




    }    

}