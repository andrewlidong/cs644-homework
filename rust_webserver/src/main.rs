use std::fs::{OpenOptions, File, rename, remove_file};
use std::io::{Write, BufRead, BufReader};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use clap::{Parser, Subcommand};
use fs2::FileExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const MAX_LOG_FILES: u32 = 5;
const LOG_FILE: &str = "http.log";

#[derive(Parser)]
#[command(name = "rust_webserver")]
#[command(about = "A simple web server with logging capabilities")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "run", about = "Run the server and continuously log")]
    Run,
    
    #[command(name = "count", about = "Count the number of log entries")]
    Count,
    
    #[command(name = "rotate", about = "Rotate log files")]
    Rotate,
}

fn rotate_logs() -> std::io::Result<()> {
    // Remove the oldest log file if it exists
    let oldest = format!("http.{}.log", MAX_LOG_FILES);
    if Path::new(&oldest).exists() {
        remove_file(&oldest)?;
    }

    // Rotate existing log files
    for i in (1..MAX_LOG_FILES).rev() {
        let current = format!("http.{}.log", i);
        let next = format!("http.{}.log", i + 1);
        if Path::new(&current).exists() {
            rename(&current, &next)?;
        }
    }

    // Rotate the main log file
    if Path::new(LOG_FILE).exists() {
        rename(LOG_FILE, "http.1.log")?;
    }

    Ok(())
}

fn append_log(message: &str, log_file: &Path) -> std::io::Result<()> {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("[{}] {}\n", timestamp, message);
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(log_file)?;
    
    // Acquire an exclusive lock
    FileExt::lock_exclusive(&file)?;
    
    // Write the log entry
    file.write_all(log_entry.as_bytes())?;
    
    // Explicitly unlock
    FileExt::unlock(&file)?;
    
    Ok(())
}

fn count_logs(log_file: &Path) -> std::io::Result<usize> {
    let file = File::open(log_file)?;
    
    // Acquire a shared lock for reading
    FileExt::lock_shared(&file)?;
    
    let reader = BufReader::new(&file);
    let count = reader.lines().count();
    
    // Release the lock
    FileExt::unlock(&file)?;
    
    Ok(count)
}

fn run_server(running: Arc<AtomicBool>) -> std::io::Result<()> {
    println!("Server started. Press Ctrl+C to stop.");
    
    while running.load(Ordering::SeqCst) {
        append_log("Server heartbeat", Path::new(LOG_FILE))?;
        sleep(Duration::from_secs(1));
    }
    
    Ok(())
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Run) => {
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();
            
            ctrlc::set_handler(move || {
                println!("\nShutting down server...");
                r.store(false, Ordering::SeqCst);
            }).expect("Error setting Ctrl-C handler");
            
            run_server(running)?;
        }
        Some(Commands::Count) => {
            let count = count_logs(Path::new(LOG_FILE))?;
            println!("Number of log entries: {}", count);
        }
        Some(Commands::Rotate) => {
            rotate_logs()?;
            println!("Log files rotated successfully");
        }
        None => {
            println!("No command specified. Use --help to see available commands.");
        }
    }

    Ok(())
} 