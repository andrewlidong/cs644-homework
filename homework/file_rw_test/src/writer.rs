use std::fs::{OpenOptions, File};
use std::io::{Write, Seek, SeekFrom};
use std::{thread, time};

fn main() {
    let filename = "testfile.txt";
    
    for i in 1..=5 {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // Overwrites the file each time
            .open(filename)
            .expect("Failed to open file for writing");

        let content = format!("Writing iteration {}...\n", i);
        file.write_all(content.as_bytes()).expect("Failed to write");

        println!("Writer: Wrote '{}'", content.trim());

        thread::sleep(time::Duration::from_secs(1)); // Simulate delay
    }
}
