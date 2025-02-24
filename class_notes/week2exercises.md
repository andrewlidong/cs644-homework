In-class exercises

1. Let's take a look at the APIs that your programming languages of choice expose for making system calls on Linux.

1. C: Direct System Calls (unistd.h, fcntl.h)
C provides low-level access to syscalls through <unistd.h> and <fcntl.h>.

Example: Open, Read, Write, Seek, Close
```c
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>

int main() {
    int fd = open("example.txt", O_RDONLY);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }

    char buffer[100];
    ssize_t bytesRead = read(fd, buffer, sizeof(buffer) - 1);
    if (bytesRead == -1) {
        perror("Error reading file");
        close(fd);
        return 1;
    }
    buffer[bytesRead] = '\0';
    printf("Read: %s\n", buffer);

    // Seek and write
    lseek(fd, 0, SEEK_END);
    int fd2 = open("output.txt", O_WRONLY | O_CREAT | O_TRUNC, 0644);
    write(fd2, buffer, bytesRead);

    close(fd);
    close(fd2);
    return 0;
}
```
Direct system calls
Efficient but requires manual error-handling

2. Python: os and sys Modules (Built-in Wrappers)
Python provides high-level wrappers for syscalls in os and sys.  

Example: Open, Read, Write, Seek, Close
```python
import os

fd = os.open("example.txt", os.O_RDONLY)
buffer = os.read(fd, 100)
print("Read:", buffer.decode())

# Seek and write
os.lseek(fd, 0, os.SEEK_END)
fd2 = os.open("output.txt", os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o644)
os.write(fd2, buffer)

os.close(fd)
os.close(fd2)
```
More readable than C
Automatically raises exceptions for errors

3. Rust: std::fs and nix Crate for Syscalls
Rust provides safe and ergonomic file handling using std::fs but also raw syscalls via nix.  

Example: Using std::fs (Safe API)
```rust
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};

fn main() {
    let mut file = File::open("example.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read");

    println!("Read: {}", buffer);

    let mut outfile = File::create("output.txt").expect("Failed to create file");
    outfile.write_all(buffer.as_bytes()).expect("Failed to write");
}

```
Safe and idiomatic
Rust's Result enforces error handling

2. Use man 2 read to view the manual page for the read syscall.

```bash
man 2 read
```
Purpose: Reads bytes from a file descriptor into a buffer.
Return Values:
Returns the number of bytes actually read.
Returns 0 at end-of-file (EOF).
Returns -1 on error (sets errno).
Errors:
EBADF: Invalid file descriptor.
EIO: Disk I/O error.
EINTR: Interrupted by a signal.
Behavior:
Reading advances the file offset.
Does not guarantee reading all requested bytes (loop needed).

3. Write a program that reads a file in fixed-size chunks and prints the number of bytes in the file. (Next week we'll learn a more efficient way to do this.)

(see count_bytes.c)

4. Write a program that appends a line of text to a file, creating it if it does not already exist. Do it once with O_APPEND and once with lseek.

(see append_text.c)

5. Let's use strace to see what system calls some common Linux utilities use.
