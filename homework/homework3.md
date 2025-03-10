# Homework 3

## Question 1 (★)
### Do syscalls follow hard links?

**Answer:** Yes

### Explanation
System calls treat hard links and the original file as the same entity because hard links point to the same inode on the file system.

### Technical Evidence
The `stat()` system call confirms this behavior - when called on either the original file or any of its hard links, it will return the same inode number, demonstrating that they reference the identical file system object.

## Question 2 (★)
### True or false: If I acquire an exclusive lock on a file, no other process will be able to write to it.

**Answer:** False

### Explanation
An exclusive lock prevents other processes from acquiring a lock on the same file. However, it does not prevent other processes from writing to the file if they don't respect the lock.

### Technical Evidence
The `flock()` system call with the `LOCK_EX` flag will acquire an exclusive lock on the file, but this is an advisory lock - processes that don't check for locks can still write to the file.

## Question 3 (★)
### How is the owner of a new file or directory set?

**Answer:** The owner is set to the effective user ID of the creating process

### Explanation
When a new file or directory is created, the system automatically sets its owner to the effective user ID (eUID) of the process that creates it.

### Technical Evidence
This can be verified by creating files while running under different effective user IDs using `sudo` or setuid programs.

## Question 4 (★★)
### Final project (database): Modify your program to create the database file with permissions locked down to the file's owner. Use file locking to ensure that get and set are synchronized. Test your solution by having the set command go to sleep for 5 seconds while holding the lock; in another terminal, ensure that get and set block until the first set command wakes up and releases the lock.

[Project implementation details would go here]

## Question 5 (★★)
### Final project (web server): A busy HTTP server can accumulate lots of logs. Add a rotate command which renames http.log to http.1.log, http.1.log to http.2.log, etc. Delete any log files older than http.5.log. Next, instead of running and exiting, modify the run command to loop forever and write a line to the log file each second, sleeping in between. Use file locking to ensure that count and run are synchronized, and make sure you can still run count while the server is running.

[Project implementation details would go here]

## Question 6 (★★)
### Write a simple version of rm -rf. If your language doesn't expose getdents64 directly, you can use a higher-level API. Be careful when testing it!

**Answer:** 

```c
#define _GNU_SOURCE  // Required for getdents64
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/stat.h>

// Linux dirent structure
struct linux_dirent64 {
    ino64_t        d_ino;
    off64_t        d_off;
    unsigned short d_reclen;
    unsigned char  d_type;
    char           d_name[];
};

// Function to recursively delete files and directories
void remove_recursive(const char *path) {
    struct stat path_stat;

    // Get file metadata
    if (syscall(SYS_lstat, path, &path_stat) == -1) {
        perror("lstat");
        return;
    }

    // If it's a file or symlink, delete it using unlink()
    if (S_ISREG(path_stat.st_mode) || S_ISLNK(path_stat.st_mode)) {
        if (syscall(SYS_unlink, path) == -1) {
            perror("unlink");
        }
        return;
    }

    // If it's a directory, process it
    if (S_ISDIR(path_stat.st_mode)) {
        int fd = syscall(SYS_open, path, O_RDONLY | O_DIRECTORY);
        if (fd == -1) {
            perror("open");
            return;
        }

        char buffer[1024];
        struct linux_dirent64 *entry;
        int bytes;

        // Read directory entries using getdents64
        while ((bytes = syscall(SYS_getdents64, fd, buffer, sizeof(buffer))) > 0) {
            for (int offset = 0; offset < bytes; offset += entry->d_reclen) {
                entry = (struct linux_dirent64 *)(buffer + offset);

                // Skip "." and ".." to avoid infinite loops
                if (entry->d_name[0] == '.' && 
                    (entry->d_name[1] == '\0' || 
                     (entry->d_name[1] == '.' && entry->d_name[2] == '\0'))) {
                    continue;
                }

                // Construct full path
                char full_path[1024];
                snprintf(full_path, sizeof(full_path), "%s/%s", path, entry->d_name);

                // Recursively delete file/directory
                remove_recursive(full_path);
            }
        }

        syscall(SYS_close, fd);

        // Remove the directory itself using rmdir()
        if (syscall(SYS_rmdir, path) == -1) {
            perror("rmdir");
        }
    }
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <directory or file>\n", argv[0]);
        return 1;
    }

    remove_recursive(argv[1]);
    return 0;
}

```

Uses only syscalls → No unnecessary high-level function calls (opendir(), readdir()).
getdents64() reads directory contents in a single syscall → Faster than iterating with readdir().
No extra heap allocations → Everything is done using fixed-size buffers.

### Explanation
This requires implementing recursive directory traversal and file deletion with proper error handling.

### Technical Evidence
The implementation should use appropriate system calls like `unlink()` for files and `rmdir()` for directories.

## Question 7 (★★)
### Write a program that takes in a file path and prints its permissions in rwxrwxrwx format.

**Answer:** 

```c
#include <stdio.h>
#include <sys/stat.h>

void print_permissions(mode_t mode) {
    char perms[10] = "---------";  // Default: No permissions

    // User (owner) permissions
    if (mode & S_IRUSR) perms[0] = 'r';
    if (mode & S_IWUSR) perms[1] = 'w';
    if (mode & S_IXUSR) perms[2] = 'x';

    // Group permissions
    if (mode & S_IRGRP) perms[3] = 'r';
    if (mode & S_IWGRP) perms[4] = 'w';
    if (mode & S_IXGRP) perms[5] = 'x';

    // Others permissions
    if (mode & S_IROTH) perms[6] = 'r';
    if (mode & S_IWOTH) perms[7] = 'w';
    if (mode & S_IXOTH) perms[8] = 'x';

    printf("%s\n", perms);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <file_path>\n", argv[0]);
        return 1;
    }

    struct stat file_stat;
    if (stat(argv[1], &file_stat) == -1) {
        perror("stat");
        return 1;
    }

    print_permissions(file_stat.st_mode);
    return 0;
}

```

### Explanation
This requires reading file metadata and converting the permission bits to human-readable format.

### Technical Evidence
The program should use `stat()` to get the file mode and convert the bits to rwx notation.

## Question 8 (★★)
### We have syscalls to move and delete files, but not to copy them. Why not?

**Answer:** Copying is a complex operation that can be implemented using more basic operations

### Explanation
File copying involves multiple operations (reading, writing, maintaining permissions) and can be implemented differently depending on needs.

### Technical Evidence
Copy operations can be implemented using combinations of `open()`, `read()`, `write()`, and `stat()` system calls.

## Question 9 (★★)
### What happens if you rename a file while another process is writing to it? Make a prediction, then write a pair of programs to demonstrate what happens.

**Answer:** The write continues to the renamed file

### Explanation
Renaming a file doesn't affect open file descriptors - processes that have the file open continue to write to the same inode.

### Technical Evidence
This can be demonstrated with a program that keeps a file open while another renames it.

```rs
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("testfile.txt")?;

    for i in 1..=10 {
        writeln!(file, "Writing line {}", i)?;
        file.flush()?;  // Ensure data is written to disk
        println!("Wrote line {}", i);
        sleep(Duration::from_secs(1));
    }

    Ok(())
}

```

## Question 10 (★★)
### Is getdents64 atomic? Write a pair of programs that demonstrates its behavior.

**Answer:** 

```rs
use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;
use libc::{getdents64, linux_dirent64};
use std::ptr;

const BUFFER_SIZE: usize = 1024;

fn main() -> io::Result<()> {
    let path = ".";
    let dir = File::open(path)?;
    let fd = dir.as_raw_fd();
    let mut buffer = [0u8; BUFFER_SIZE];

    println!("Reading directory entries...");

    unsafe {
        let bytes_read = getdents64(fd, buffer.as_mut_ptr() as *mut linux_dirent64, BUFFER_SIZE as u32);
        if bytes_read == -1 {
            eprintln!("Error reading directory entries");
            return Err(io::Error::last_os_error());
        }

        let mut offset = 0;
        while offset < bytes_read as usize {
            let d = buffer.as_ptr().add(offset) as *const linux_dirent64;
            let name_ptr = (*d).d_name.as_ptr();
            let name = std::ffi::CStr::from_ptr(name_ptr).to_string_lossy();
            println!("Found: {}", name);

            offset += (*d).d_reclen as usize;
        }
    }

    println!("Finished reading directory.");
    Ok(())
}

```

```rs
use std::fs::{File, remove_file};
use std::thread;
use std::time::Duration;

fn main() {
    let filename = "tempfile.txt";

    // Sleep briefly to ensure reader has started
    thread::sleep(Duration::from_secs(1));

    println!("Creating file: {}", filename);
    let _ = File::create(filename);

    thread::sleep(Duration::from_secs(2));

    println!("Deleting file: {}", filename);
    let _ = remove_file(filename);
}

```

### Explanation
This requires testing directory listing behavior with concurrent modifications.

### Technical Evidence
Programs should demonstrate whether directory entries can change during a single `getdents64` call.

getdents64 is NOT atomic. If the directory is modified during reading, results can be inconsistent.
The listing may include files that were deleted or miss files that were added.
If you need consistent snapshots, you should use locking mechanisms or temporary file indexing.


## Question 11 (★★)
### Linux file permissions are a little more complicated than what was presented here. Research the concepts of the set-uid, set-gid, and sticky bits.

**Answer:** These are special permission bits that modify standard Unix permissions

### Explanation
- Set-UID: Allows execution with owner's permissions
- Set-GID: Allows execution with group's permissions
- Sticky bit: Restricts deletion in directories

### Technical Evidence
These can be observed on system files like `/usr/bin/passwd` (setuid) and `/tmp` (sticky bit).

## Question 12 (★★★)
### Is it possible to atomically overwrite a file? First, write a pair of programs (a reader and a writer) that shows that simply overwriting with write isn't atomic. Then, find a way to do it atomically.

```rs

use std::fs::File;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let filename = "testfile.txt";

    loop {
        let mut file = File::open(filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        println!("Read: {}", content);

        // Sleep a bit to let the writer act
        thread::sleep(Duration::from_millis(100));
    }
}


```rs
use std::fs::File;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let filename = "testfile.txt";

    loop {
        let mut file = File::create(filename)?;
        file.write_all(b"AAAAAAAAAA")?;
        file.flush()?;
        
        thread::sleep(Duration::from_millis(100));

        let mut file = File::create(filename)?;
        file.write_all(b"BBBBBBBBBB")?;
        file.flush()?;
        
        thread::sleep(Duration::from_millis(100));
    }
}

```

```rs
use std::fs::{File, rename};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let filename = "testfile.txt";
    let temp_filename = "testfile.tmp";

    loop {
        let mut temp_file = File::create(temp_filename)?;
        temp_file.write_all(b"AAAAAAAAAA")?;
        temp_file.flush()?; 

        // Atomically replace the original file
        rename(temp_filename, filename)?;

        thread::sleep(Duration::from_millis(100));

        let mut temp_file = File::create(temp_filename)?;
        temp_file.write_all(b"BBBBBBBBBB")?;
        temp_file.flush()?;

        rename(temp_filename, filename)?;

        thread::sleep(Duration::from_millis(100));
    }
}

```
**Answer:** 

### Explanation
Direct overwrites aren't atomic, but atomic replacement can be achieved using rename.

### Technical Evidence
Implementation should demonstrate both non-atomic writes and atomic replacement using temporary files and rename.

## Question 13 (★★★)
### mv myfile.txt /tmp requires an additional permission beyond what we discussed. What is it? Can you explain why it's necessary?

**Answer:** Write permission on the source directory is required

### Explanation
Moving a file requires modifying both source and destination directories' contents.

### Technical Evidence
This can be demonstrated by removing write permission from the source directory and attempting the move.

## Question 14 (★★★)
### Linux file permissions are a lot more complicated than what was presented here. Research file ACLs and SELinux contexts. What syscalls do they use?

**Answer:** ACLs and SELinux add additional layers of access control

### Explanation
- ACLs provide fine-grained permissions beyond traditional Unix permissions
- SELinux adds mandatory access control based on security contexts

### Technical Evidence
ACLs use syscalls like `getxattr()`, `setxattr()`, while SELinux uses security-enhanced versions of standard syscalls.

