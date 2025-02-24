use std::fs::{File, OpenOptions};
use std::os::unix::fs::PermissionsExt; // Needed for setting file permissions
use std::fs;
use std::io::{ErrorKind, Write};

const TEST_FILE: &str = "test_file.txt";
const NO_FILE: &str = "does_not_exist.txt";
const NO_PERM_FILE: &str = "no_permission.txt";

fn test_eacces() {
    println!("\nTesting EACCES (Permission Denied)...");

    // Create a file with no permissions (000)
    match File::create(NO_PERM_FILE) {
        Ok(file) => {
            drop(file);
            fs::set_permissions(NO_PERM_FILE, fs::Permissions::from_mode(0o000))
                .expect("Failed to set permissions");
        }
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            return;
        }
    }

    // Try to open the file without permissions
    match OpenOptions::new().write(true).open(NO_PERM_FILE) {
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            println!("Caught EACCES: Permission denied as expected!");
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
        Ok(_) => {
            println!("Unexpected success!");
        }
    }
}

fn test_eexist() {
    println!("\nTesting EEXIST (File Already Exists)...");

    // Create a file
    if File::create(TEST_FILE).is_err() {
        eprintln!("Error creating test file");
        return;
    }

    // Try to create it again with `create_new(true)` (equivalent to `O_EXCL`)
    match OpenOptions::new().write(true).create_new(true).open(TEST_FILE) {
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            println!("Caught EEXIST: File already exists as expected!");
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
        Ok(_) => {
            println!("Unexpected success!");
        }
    }
}

fn test_enoent() {
    println!("\nTesting ENOENT (No Such File or Directory)...");

    // Try to open a file that doesn't exist
    match File::open(NO_FILE) {
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("Caught ENOENT: No such file or directory as expected!");
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
        }
        Ok(_) => {
            println!("Unexpected success!");
        }
    }
}

fn main() {
    test_eacces();
    test_eexist();
    test_enoent();
}
