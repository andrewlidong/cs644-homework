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
use nix::unistd::fork;
use nix::sys::wait;
use std::process::exit;

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

fn run_child_process(id: u32) -> ! {
    println!("Child process {} started", std::process::id());
    // Sleep for a variable amount of time based on the child ID
    let sleep_duration = Duration::from_secs((id as u64 + 1) * 3);
    sleep(sleep_duration);
    println!("Child process {} exiting", std::process::id());
    exit(id as i32)
}

fn run_server(running: Arc<AtomicBool>) -> std::io::Result<()> {
    println!("Server started. Press Ctrl+C to stop.");
    
    let mut child_pids = Vec::new();
    
    // Fork 4 child processes
    for i in 0..4 {
        match unsafe { fork() } {
            Ok(nix::unistd::ForkResult::Parent { child }) => {
                println!("Forked child process with PID: {}", child);
                child_pids.push(child);
            }
            Ok(nix::unistd::ForkResult::Child) => {
                run_child_process(i);
            }
            Err(err) => {
                eprintln!("Fork failed: {}", err);
                return Ok(());
            }
        }
    }
    
    // Parent process continues with main loop while monitoring children
    while running.load(Ordering::SeqCst) {
        // Check if any child has exited without blocking
        let mut i = 0;
        while i < child_pids.len() {
            match wait::waitpid(Some(child_pids[i]), Some(wait::WaitPidFlag::WNOHANG)) {
                Ok(wait::WaitStatus::Exited(pid, status)) => {
                    println!("Child process {} exited with status {}", pid, status);
                    child_pids.remove(i);
                }
                Ok(wait::WaitStatus::Signaled(pid, signal, _)) => {
                    println!("Child process {} terminated by signal {:?}", pid, signal);
                    child_pids.remove(i);
                }
                Ok(wait::WaitStatus::StillAlive) => {
                    i += 1;
                }
                Err(err) => {
                    eprintln!("Error waiting for child process: {}", err);
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
        
        // Continue with normal server operations
        append_log("Server heartbeat", Path::new(LOG_FILE))?;
        sleep(Duration::from_secs(1));
        
        // If all children have exited, we can break the loop
        if child_pids.is_empty() {
            println!("All child processes have exited");
            break;
        }
    }
    
    // If we're shutting down, make sure to wait for any remaining children
    for pid in child_pids {
        match wait::waitpid(Some(pid), None) {
            Ok(wait::WaitStatus::Exited(pid, status)) => {
                println!("Child process {} exited with status {}", pid, status);
            }
            Ok(wait::WaitStatus::Signaled(pid, signal, _)) => {
                println!("Child process {} terminated by signal {:?}", pid, signal);
            }
            Err(err) => {
                eprintln!("Error waiting for child process: {}", err);
            }
            _ => {}
        }
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