use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};
use std::env;
use chrono::Local; // For timestamps
use std::error::Error;

/// Log file path
const LOG_FILE: &str = "server.log";

/// Append a log entry with a timestamp
fn run() -> Result<(), Box<dyn Error>> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_entry = format!("[{}] New log entry\n", timestamp);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)?;

    file.write_all(log_entry.as_bytes())?;
    println!("Log entry added.");
    
    Ok(())
}

/// Count the number of log entries
fn count() -> Result<(), Box<dyn Error>> {
    let file = File::open(LOG_FILE)?;
    let reader = BufReader::new(file);
    let line_count = reader.lines().count();

    println!("Total log entries: {}", line_count);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  {} run    (Append a log entry)", args[0]);
        println!("  {} count  (Count log entries)", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "run" => run()?,
        "count" => count()?,
        _ => println!("Invalid command. Use 'run' or 'count'."),
    }

    Ok(())
}
