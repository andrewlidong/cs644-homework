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
use nix::unistd::{fork, pipe};
use nix::sys::wait;
use std::process::exit;
use serde::{Serialize, Deserialize};
use nix::fcntl::{fcntl, FcntlArg, OFlag};

const MAX_LOG_FILES: u32 = 5;
const LOG_FILE: &str = "server.log";
const CONFIG_FILE: &str = "server_config.json";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct ServerConfig {
    verbosity: u32,
    max_connections: u32,
    timeout_seconds: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            verbosity: 1,
            max_connections: 100,
            timeout_seconds: 30,
        }
    }
}

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

    #[command(name = "update-config", about = "Update server configuration")]
    UpdateConfig {
        #[arg(long)]
        verbosity: Option<u32>,
        
        #[arg(long)]
        max_connections: Option<u32>,
        
        #[arg(long)]
        timeout_seconds: Option<u32>,
    },
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

fn load_config() -> std::io::Result<ServerConfig> {
    let file = File::open(CONFIG_FILE)?;
    FileExt::lock_shared(&file)?;
    
    let reader = BufReader::new(&file);
    let config: ServerConfig = serde_json::from_reader(reader)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    FileExt::unlock(&file)?;
    Ok(config)
}

fn save_config(config: &ServerConfig) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(CONFIG_FILE)?;
    
    FileExt::lock_exclusive(&file)?;
    
    let writer = std::io::BufWriter::new(&file);
    serde_json::to_writer(writer, config)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    FileExt::unlock(&file)?;
    Ok(())
}

fn run_child_process(id: u32, pipe_fd: i32) -> ! {
    println!("Child process {} started (worker {})", std::process::id(), id);
    
    let mut buffer = [0u8; 1];
    let log_file = Path::new(LOG_FILE);
    
    loop {
        // Check if parent sent shutdown signal
        match nix::unistd::read(pipe_fd, &mut buffer) {
            Ok(_) => {
                println!("Child process {} (worker {}) shutting down", std::process::id(), id);
                exit(0);
            }
            Err(e) if e == nix::errno::Errno::EAGAIN => {
                // Non-blocking read returned no data, continue with normal operation
            }
            Err(e) => {
                eprintln!("Error reading from pipe: {}", e);
            }
        }
        
        match load_config() {
            Ok(config) => {
                if config.verbosity > 0 {
                    println!("Worker {} running with verbosity {}", id, config.verbosity);
                }
                
                // Each worker logs with its own identifier
                let message = format!("Worker {} heartbeat (verbosity: {})", id, config.verbosity);
                if let Err(e) = append_log(&message, log_file) {
                    eprintln!("Error writing to log: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error loading config: {}", e);
            }
        }
        
        // Sleep for a duration based on worker ID to avoid all workers logging at the same time
        sleep(Duration::from_millis((id as u64 + 1) * 500));
    }
}

fn run_server(running: Arc<AtomicBool>) -> std::io::Result<()> {
    println!("Server started. Press Ctrl+C to stop.");
    
    // Initialize config file
    let config = ServerConfig::default();
    save_config(&config)?;
    
    let mut child_pids = Vec::new();
    let mut pipe_fds = Vec::new();
    
    // Fork 4 child processes
    for i in 0..4 {
        let (reader, writer) = pipe().expect("Failed to create pipe");
        
        // Set the reader end to non-blocking mode
        let flags = fcntl(reader, FcntlArg::F_GETFL).expect("Failed to get flags");
        fcntl(reader, FcntlArg::F_SETFL(OFlag::from_bits_truncate(flags) | OFlag::O_NONBLOCK))
            .expect("Failed to set non-blocking mode");
        
        match unsafe { fork() } {
            Ok(nix::unistd::ForkResult::Parent { child }) => {
                println!("Forked child process with PID: {}", child);
                child_pids.push(child);
                pipe_fds.push(writer);
                // Close reader end in parent
                nix::unistd::close(reader).expect("Failed to close read end in parent");
            }
            Ok(nix::unistd::ForkResult::Child) => {
                // Close write end in child
                nix::unistd::close(writer).expect("Failed to close write end in child");
                run_child_process(i, reader);
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
                    nix::unistd::close(pipe_fds.remove(i)).expect("Failed to close pipe");
                }
                Ok(wait::WaitStatus::Signaled(pid, signal, _)) => {
                    println!("Child process {} terminated by signal {:?}", pid, signal);
                    child_pids.remove(i);
                    nix::unistd::close(pipe_fds.remove(i)).expect("Failed to close pipe");
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
        sleep(Duration::from_secs(1));
        
        // If all children have exited, we can break the loop
        if child_pids.is_empty() {
            println!("All child processes have exited");
            break;
        }
    }
    
    // If we're shutting down, send shutdown signal to all remaining children
    for fd in &pipe_fds {
        let _ = nix::unistd::write(*fd, &[1]);
    }
    
    // Close all pipe file descriptors
    for fd in pipe_fds {
        let _ = nix::unistd::close(fd);
    }
    
    // Wait for all children to exit
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
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    match cli.command {
        Some(Commands::Run) => {
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
        Some(Commands::UpdateConfig { verbosity, max_connections, timeout_seconds }) => {
            let mut config = load_config()?;
            if let Some(v) = verbosity {
                config.verbosity = v;
            }
            if let Some(m) = max_connections {
                config.max_connections = m;
            }
            if let Some(t) = timeout_seconds {
                config.timeout_seconds = t;
            }
            save_config(&config)?;
            println!("Configuration updated successfully");
        }
        None => {
            println!("No command specified. Use --help to see available commands.");
        }
    }

    Ok(())
} 