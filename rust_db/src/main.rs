use std::fs::File;
use std::io::{self, Write, BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use shared_memory::{Shmem, ShmemConf};
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};
use libc::fork;

/// Database file path
const DB_FILE: &str = "db.txt";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the database server
    Serve {
        /// Number of child processes to spawn
        #[arg(short, long, default_value_t = 4)]
        workers: u32,
    },
    /// Run a single command and exit
    Run {
        /// Command to execute (get/set/delete)
        #[arg(short, long)]
        cmd: String,
        /// Key for the command
        #[arg(short, long)]
        key: String,
        /// Value for set command
        #[arg(short, long)]
        value: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
struct Cache {
    entries: Vec<Entry>,
    dirty: bool,
}

pub struct DB {
    path: PathBuf,
    cache: Option<Arc<Shmem>>,
}

impl DB {
    pub fn new(path: &str) -> io::Result<DB> {
        let path = PathBuf::from(path);
        // Create file if it doesn't exist
        File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        Ok(DB {
            path,
            cache: None,
        })
    }

    pub fn with_cache(mut self, shmem: Shmem) -> Self {
        self.cache = Some(Arc::new(shmem));
        self
    }

    fn get_from_cache(&self, key: &str) -> io::Result<Option<String>> {
        if let Some(cache) = &self.cache {
            let cache_data = unsafe { &*(cache.as_ptr() as *const Cache) };
            for entry in &cache_data.entries {
                if entry.key == key {
                    return Ok(Some(entry.value.clone()));
                }
            }
        }
        Ok(None)
    }

    fn set_in_cache(&self, key: &str, value: &str) -> io::Result<()> {
        if let Some(cache) = &self.cache {
            let cache_data = unsafe { &mut *(cache.as_ptr() as *mut Cache) };
            let mut found = false;
            for entry in &mut cache_data.entries {
                if entry.key == key {
                    entry.value = value.to_string();
                    found = true;
                    break;
                }
            }
            if !found {
                cache_data.entries.push(Entry {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            }
            cache_data.dirty = true;
        }
        Ok(())
    }

    pub fn get(&self, key: &str) -> io::Result<Option<String>> {
        // Try cache first
        if let Some(value) = self.get_from_cache(key)? {
            return Ok(Some(value));
        }

        // Fall back to file
        let mut file = File::open(&self.path)?;
        file.seek(SeekFrom::Start(0))?;
        let reader = BufReader::new(&file);
        for line in reader.lines() {
            let line = line?;
            if let Ok(entry) = serde_json::from_str::<Entry>(&line) {
                if entry.key == key {
                    return Ok(Some(entry.value));
                }
            }
        }
        Ok(None)
    }

    pub fn set(&mut self, key: &str, value: &str) -> io::Result<()> {
        // Update cache
        self.set_in_cache(key, value)?;

        // Update file
        let mut entries = Vec::new();
        let mut file = File::open(&self.path)?;
        file.seek(SeekFrom::Start(0))?;
        let reader = BufReader::new(&file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(mut entry) = serde_json::from_str::<Entry>(&line) {
                    if entry.key == key {
                        entry.value = value.to_string();
                    }
                    entries.push(entry);
                }
            }
        }

        // Check if key exists
        if !entries.iter().any(|e| e.key == key) {
            entries.push(Entry {
                key: key.to_string(),
                value: value.to_string(),
            });
        }

        // Write back to file
        let mut file = File::create(&self.path)?;
        for entry in entries {
            writeln!(file, "{}", serde_json::to_string(&entry)?)?;
        }
        Ok(())
    }

    pub fn delete(&mut self, key: &str) -> io::Result<()> {
        // Remove from cache
        if let Some(cache) = &self.cache {
            let cache_data = unsafe { &mut *(cache.as_ptr() as *mut Cache) };
            cache_data.entries.retain(|e| e.key != key);
            cache_data.dirty = true;
        }

        // Remove from file
        let mut entries = Vec::new();
        let mut file = File::open(&self.path)?;
        file.seek(SeekFrom::Start(0))?;
        let reader = BufReader::new(&file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(entry) = serde_json::from_str::<Entry>(&line) {
                    if entry.key != key {
                        entries.push(entry);
                    }
                }
            }
        }

        let mut file = File::create(&self.path)?;
        for entry in entries {
            writeln!(file, "{}", serde_json::to_string(&entry)?)?;
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { workers } => {
            // Create shared memory for cache
            let shmem_conf = ShmemConf::new()
                .size(1024 * 1024) // 1MB shared memory
                .os_id("rust_db_cache");
            
            let shmem = shmem_conf.create().expect("Failed to create shared memory");
            let cache_ptr = shmem.as_ptr() as *mut Cache;
            
            // Initialize cache
            unsafe {
                *cache_ptr = Cache {
                    entries: Vec::new(),
                    dirty: false,
                };
            }

            // Create database instance with cache
            let _db = DB::new(DB_FILE)?.with_cache(shmem);

            // Fork child processes
            for i in 0..workers {
                let pid = unsafe { fork() };
                match pid {
                    -1 => {
                        eprintln!("Failed to fork: {}", std::io::Error::last_os_error());
                        return Err(io::Error::new(io::ErrorKind::Other, "Fork failed"));
                    }
                    0 => {
                        // Child process
                        println!("Worker {} running", i);
                        // Keep the child process running
                        loop {
                            std::thread::sleep(Duration::from_secs(1));
                        }
                    }
                    pid => {
                        // Parent process
                        println!("Started worker process {} with pid {}", i, pid);
                    }
                }
            }

            // Parent process keeps running
            println!("Server running with {} workers", workers);
            loop {
                std::thread::sleep(Duration::from_secs(1));
            }
        }
        Commands::Run { cmd, key, value } => {
            let mut db = DB::new(DB_FILE)?;
            match cmd.as_str() {
                "get" => {
                    if let Some(value) = db.get(&key)? {
                        println!("{}", value);
                    } else {
                        println!("Key not found");
                    }
                }
                "set" => {
                    if let Some(value) = value {
                        db.set(&key, &value)?;
                        println!("Value set successfully");
                    } else {
                        println!("Value required for set command");
                    }
                }
                "delete" => {
                    db.delete(&key)?;
                    println!("Key deleted successfully");
                }
                _ => println!("Unknown command"),
            }
        }
    }

    Ok(())
}