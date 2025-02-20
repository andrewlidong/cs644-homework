use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, Write};
use std::sync::Mutex;
use std::time::Instant;

#[derive(Clone)]
struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

struct PersistentDatabase {
    store: Mutex<HashMap<String, Entry>>,
    file_path: String,
}

impl PersistentDatabase {
    fn new(file_path: &str) -> Self {
        let mut store = HashMap::new();
        if let Ok(file) = File::open(file_path) {
            for line in io::BufReader::new(file).lines() {
                if let Ok(entry) = line {
                    let parts: Vec<&str> = entry.splitn(2, ',').collect();
                    if parts.len() == 2 {
                        store.insert(parts[0].to_string(), Entry {
                            value: parts[1].to_string(),
                            expires_at: None,
                        });
                    }
                }
            }
        }

        PersistentDatabase {
            store: Mutex::new(store),
            file_path: file_path.to_string(),
        }
    }

    fn set(&self, key: String, value: String) {
        let mut db = self.store.lock().unwrap();
        let entry = Entry {
            value: value.clone(),
            expires_at: None,
        };
        db.insert(key.clone(), entry);

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();
        writeln!(file, "{},{}", key, value).unwrap();
    }

    fn get(&self, key: &str) -> Option<Entry> {
        let db = self.store.lock().unwrap();
        db.get(key).cloned()
    }
}

fn main() {
    let db = PersistentDatabase::new("database.txt");

    db.set("language".to_string(), "Rust".to_string());
    if let Some(entry) = db.get("language") {
        println!("Stored value: {}", entry.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_set_and_get() {
        let test_file = "test_db.txt";
        let db = PersistentDatabase::new(test_file);
        
        // Test setting and getting a value
        db.set("test_key".to_string(), "test_value".to_string());
        assert_eq!(db.get("test_key"), Some("test_value".to_string()));
        
        // Clean up test file
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_multiple_values() {
        let test_file = "test_db2.txt";
        let db = PersistentDatabase::new(test_file);
        
        // Test multiple key-value pairs
        db.set("key1".to_string(), "value1".to_string());
        db.set("key2".to_string(), "value2".to_string());
        
        assert_eq!(db.get("key1"), Some("value1".to_string()));
        assert_eq!(db.get("key2"), Some("value2".to_string()));
        assert_eq!(db.get("nonexistent"), None);
        
        // Clean up test file
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_value_update() {
        let test_file = "test_db3.txt";
        let db = PersistentDatabase::new(test_file);
        
        // Test updating an existing key
        db.set("update_key".to_string(), "first_value".to_string());
        db.set("update_key".to_string(), "second_value".to_string());
        
        assert_eq!(db.get("update_key"), Some("second_value".to_string()));
        
        // Clean up test file
        fs::remove_file(test_file).unwrap();
    }
}
