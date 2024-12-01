use std::fs::File;
//use std::io::Error;
//use std::io::{BufReader, Read};
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use std::fs;

//const BUFFER_SIZE: usize = 512;

fn main() {

    let path = String::from("text_data.txt");
    //read_file_as_single_bytes(path.clone());
    
    let result = read_file_vec(&path);
    match result {
        Ok(result) => {
            println!("File contents 2: {:?}", result);
        }
        Err(error) => {

        }
    }

    
}

fn read_file_as_single_bytes(path: String) {
   
    let file = File::open(path);
    match file {
        Ok(mut file) => {
            let pos = file.seek(SeekFrom::Start(21));
            match pos {
                Ok(_) => {
                    //let reader = BufReader::with_capacity(BUFFER_SIZE, &file);
                    let mut contents = Vec::new();
                    let read = file.read_to_end(&mut contents);
                    match read {
                       Ok(_) => {
                            println!("File contents: {:?}", contents);
                       }
                       Err(error) => {
                            eprintln!("Seek Error {}", error);
                       }
                    }
                    println!("File contents: {:?}", contents);
                }
                Err(error) => {
                    eprintln!("Seek Error {}", error);
                }

            }
        
        }
        Err(error) => {
            eprintln!("Read Error {}", error);
        }
    }


}

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}
