use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{thread, time};

fn main() {
    let filename = "testfile.txt";

    for _ in 0..5 {
        let file = File::open(filename);

        match file {
            Ok(f) => {
                let reader = BufReader::new(f);
                let mut lines = Vec::new();
                
                for line in reader.lines() {
                    match line {
                        Ok(content) => lines.push(content),
                        Err(e) => println!("Reader: Error reading line - {}", e),
                    }
                }

                println!("Reader: {:?}", lines);
            }
            Err(e) => println!("Reader: Failed to open file - {}", e),
        }

        thread::sleep(time::Duration::from_secs(1)); // Read every second
    }
}
