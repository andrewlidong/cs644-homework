use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::time::Instant;

/// Reads a file line-by-line and counts the total number of lines
fn count_lines(filename: &str) -> std::io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut line_count = 0;

    for line in reader.lines() {
        match line {
            Ok(_) => line_count += 1,
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok(line_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    let start = Instant::now();
    match count_lines(filename) {
        Ok(total_lines) => {
            let duration = start.elapsed();
            println!("Total lines in file: {}", total_lines);
            println!("Execution time: {:?}", duration);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}


// use std::fs::File;
// use std::io::{BufReader, Read};
// use std::env;
// use std::time::Instant;

// /// Reads a file in chunks and counts both total bytes and whitespace characters
// fn count_bytes_and_whitespace(filename: &str, chunk_size: usize) -> std::io::Result<(usize, usize)> {
//     let file = File::open(filename)?;
//     let mut reader = BufReader::new(file);
//     let mut buffer = vec![0; chunk_size];
//     let mut total_bytes = 0;
//     let mut whitespace_count = 0;

//     while let Ok(bytes_read) = reader.read(&mut buffer) {
//         if bytes_read == 0 {
//             break; // EOF
//         }

//         total_bytes += bytes_read;
//         whitespace_count += buffer[..bytes_read]
//             .iter()
//             .filter(|&&c| c == b' ' || c == b'\t' || c == b'\n')
//             .count();
//     }

//     Ok((total_bytes, whitespace_count))
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 3 {
//         println!("Usage: {} <filename> <chunk_size>", args[0]);
//         return;
//     }

//     let filename = &args[1];
//     let chunk_size: usize = args[2].parse().expect("Chunk size must be a number");

//     let start = Instant::now();
//     match count_bytes_and_whitespace(filename, chunk_size) {
//         Ok((total_bytes, whitespace_count)) => {
//             let duration = start.elapsed();
//             println!("Total file size: {} bytes", total_bytes);
//             println!("Total whitespace characters: {}", whitespace_count);
//             println!("Execution time: {:?}", duration);
//         }
//         Err(e) => eprintln!("Error reading file: {}", e),
//     }
// }


// // use std::fs::File;
// // use std::io::{BufReader, Read};
// // use std::env;
// // use std::time::Instant;

// // /// Reads a file in fixed-size chunks and counts the total number of bytes
// // fn count_bytes(filename: &str, chunk_size: usize) -> std::io::Result<usize> {
// //     let file = File::open(filename)?;
// //     let mut reader = BufReader::new(file);
// //     let mut buffer = vec![0; chunk_size];
// //     let mut total_bytes = 0;

// //     while let Ok(bytes_read) = reader.read(&mut buffer) {
// //         if bytes_read == 0 {
// //             break; // EOF
// //         }
// //         total_bytes += bytes_read;
// //     }

// //     Ok(total_bytes)
// // }

// // fn main() {
// //     let args: Vec<String> = env::args().collect();

// //     if args.len() < 3 {
// //         println!("Usage: {} <filename> <chunk_size>", args[0]);
// //         return;
// //     }

// //     let filename = &args[1];
// //     let chunk_size: usize = args[2].parse().expect("Chunk size must be a number");

// //     let start = Instant::now();
// //     match count_bytes(filename, chunk_size) {
// //         Ok(total) => {
// //             let duration = start.elapsed();
// //             println!("Total file size: {} bytes", total);
// //             println!("Execution time: {:?}", duration);
// //         }
// //         Err(e) => eprintln!("Error reading file: {}", e),
// //     }
// // }
