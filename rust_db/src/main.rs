use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};
use std::env;
use std::error::Error;

/// Database file path
const DB_FILE: &str = "db.txt";

/// Append a key-value pair to the database file
fn set(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().create(true).append(true).open(DB_FILE)?;

    writeln!(file, "{}|{}", key, value)?;
    println!("Set key: {}", key);

    Ok(())
}

/// Retrieve a value by key
fn get(key: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(DB_FILE)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(2, '|').collect();

        if parts.len() == 2 && parts[0] == key {
            println!("Value: {}", parts[1]);
            return Ok(());
        }
    }

    println!("Key not found.");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("   {} set <key> <value>", args[0]);
        println!("   {} get <key>", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "set" if args.len() == 4 => set(&args[2], &args[3])?,
        "get" if args.len() == 3 => get(&args[2])?,
        _ => println!("Invalid command.  Use 'set' or 'get'."),
    }

    Ok(())
}

// use std::collections::HashMap;
// use std::fs::{OpenOptions, File};
// use std::io::{self, BufRead, Write};
// use std::sync::Mutex;
// use std::time::Instant;

// #[derive(Clone)]
// struct Entry {
//     value: String,
//     expires_at: Option<Instant>,
// }

// struct PersistentDatabase {
//     store: Mutex<HashMap<String, Entry>>,
//     file_path: String,
// }

// impl PersistentDatabase {
//     fn new(file_path: &str) -> Self {
//         let mut store = HashMap::new();
//         if let Ok(file) = File::open(file_path) {
//             for line in io::BufReader::new(file).lines() {
//                 if let Ok(entry) = line {
//                     let parts: Vec<&str> = entry.splitn(2, ',').collect();
//                     if parts.len() == 2 {
//                         store.insert(parts[0].to_string(), Entry {
//                             value: parts[1].to_string(),
//                             expires_at: None,
//                         });
//                     }
//                 }
//             }
//         }

//         PersistentDatabase {
//             store: Mutex::new(store),
//             file_path: file_path.to_string(),
//         }
//     }

//     fn set(&self, key: String, value: String) {
//         let mut db = self.store.lock().unwrap();
//         let entry = Entry {
//             value: value.clone(),
//             expires_at: None,
//         };
//         db.insert(key.clone(), entry);

//         let mut file = OpenOptions::new()
//             .append(true)
//             .create(true)
//             .open(&self.file_path)
//             .unwrap();
//         writeln!(file, "{},{}", key, value).unwrap();
//     }

//     fn get(&self, key: &str) -> Option<Entry> {
//         let db = self.store.lock().unwrap();
//         db.get(key).cloned()
//     }
// }

// fn main() {
//     let db = PersistentDatabase::new("database.txt");

//     db.set("language".to_string(), "Rust".to_string());
//     if let Some(entry) = db.get("language") {
//         println!("Stored value: {}", entry.value);
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;

//     #[test]
//     fn test_set_and_get() {
//         let test_file = "test_db.txt";
//         let db = PersistentDatabase::new(test_file);
        
//         // Test setting and getting a value
//         db.set("test_key".to_string(), "test_value".to_string());
//         assert_eq!(db.get("test_key"), Some("test_value".to_string()));
        
//         // Clean up test file
//         fs::remove_file(test_file).unwrap();
//     }

//     #[test]
//     fn test_multiple_values() {
//         let test_file = "test_db2.txt";
//         let db = PersistentDatabase::new(test_file);
        
//         // Test multiple key-value pairs
//         db.set("key1".to_string(), "value1".to_string());
//         db.set("key2".to_string(), "value2".to_string());
        
//         assert_eq!(db.get("key1"), Some("value1".to_string()));
//         assert_eq!(db.get("key2"), Some("value2".to_string()));
//         assert_eq!(db.get("nonexistent"), None);
        
//         // Clean up test file
//         fs::remove_file(test_file).unwrap();
//     }

//     #[test]
//     fn test_value_update() {
//         let test_file = "test_db3.txt";
//         let db = PersistentDatabase::new(test_file);
        
//         // Test updating an existing key
//         db.set("update_key".to_string(), "first_value".to_string());
//         db.set("update_key".to_string(), "second_value".to_string());
        
//         assert_eq!(db.get("update_key"), Some("second_value".to_string()));
        
//         // Clean up test file
//         fs::remove_file(test_file).unwrap();
//     }
// }