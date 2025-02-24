## What is a syscall?

A **system call** or **syscall** is how your program communicates with the operating system. Syscalls look like function calls, but instead of jumping to another point in your program, they switch out of your program entirely and into the operating systems.

Usually programming languages wrap system calls with higher-level APIs, for portability (system calls are OS-specific) and convenience. However, in this course we will be making syscalls directly[1](https://iafisher.com/cs644/spring2025/week2#fn:directly) because we want to understand exactly what we are asking the OS to do.

A syscall is the mechanism that user-space programs use to request services from the kernel (the core part of the operating system). Since user programs don’t have direct access to hardware or critical system resources for security reasons, they use syscalls to interact with the OS for tasks like:

Reading and writing files (read, write, open, close)
Creating and managing processes (fork, exec, waitpid)
Allocating memory (mmap, brk)
Communicating between processes (pipe, socket, shmget)
How a Syscall Works (Simplified)
The program places the syscall number and arguments in specific registers.
It triggers a software interrupt (int 0x80 in x86, syscall instruction in x86_64) to transition to kernel mode.
The OS executes the requested operation.
The OS switches back to user mode and returns a result.
Why Use Syscalls Directly?
Most programming languages provide standard libraries (like libc in C) that wrap syscalls in more user-friendly functions. However, making syscalls directly (e.g., using inline assembly or syscall in C) helps in understanding how OS internals work.

## Syscall error handling

If the syscall fails (because of invalid arguments, because of inadequate permissions, etc.), a negative integer is returned that indicates the specific problem. Error codes have descriptive names like `EACCES`, `EINVAL`, and `EBUSY`, but the exact meaning depends on the syscall.

Some languages handle error results differently. C sets a per-thread global variable called `errno`. Python raises an `OSError`.

Syscall Error Handling
When a system call (syscall) fails, it typically returns a negative integer, which is an error code indicating what went wrong. However, the way errors are reported depends on the programming language and system.

How Error Handling Works
1. In C (using errno)
In C, syscalls usually return -1 on failure.
The global variable errno is set with a specific error code (e.g., EACCES, EINVAL).
You can use perror() or strerror() to get a human-readable error message.
Example: Handling an Open File Error
c
Copy
Edit
#include <stdio.h>
#include <fcntl.h>
#include <errno.h>
#include <string.h>

int main() {
    int fd = open("nonexistent_file.txt", O_RDONLY);
    if (fd == -1) {
        printf("Error opening file: %s\n", strerror(errno));
        return 1;
    }
    return 0;
}
Expected Output (if file does not exist):

yaml
Copy
Edit
Error opening file: No such file or directory
2. In Python (OSError)
Python automatically raises an OSError (or a subclass like FileNotFoundError).
You can catch the exception and inspect the error.
Example: Handling a File Open Error in Python
python
Copy
Edit
try:
    with open("nonexistent_file.txt", "r") as f:
        pass
except OSError as e:
    print(f"Error opening file: {e}")
Expected Output (if file does not exist):

javascript
Copy
Edit
Error opening file: [Errno 2] No such file or directory: 'nonexistent_file.txt'
Common Syscall Error Codes
Error Code	Meaning
EACCES	Permission denied
EINVAL	Invalid argument
EBUSY	Resource busy
ENOMEM	Out of memory
EEXIST	File exists
ENOENT	No such file or directory
EPERM	Operation not permitted
These codes are OS-specific, but many Unix-based systems follow the POSIX standard.

Checking Syscall Errors in Assembly (x86-64 Linux)
In low-level syscall implementations, the return value is in the rax register. If rax is negative, an error has occurred.

Example: Using syscall in Assembly
assembly
Copy
Edit
section .data
    msg db "Hello, world!", 10
    len equ $ - msg

section .text
    global _start

_start:
    ; Syscall: write(1, msg, len)
    mov rax, 1          ; syscall number for sys_write
    mov rdi, 1          ; file descriptor (stdout)
    mov rsi, msg        ; pointer to message
    mov rdx, len        ; length of message
    syscall             ; invoke syscall

    ; Check for error (negative return value)
    cmp rax, 0
    jl error_handler

    ; Syscall: exit(0)
    mov rax, 60         ; syscall number for sys_exit
    xor rdi, rdi        ; exit code 0
    syscall             ; invoke syscall

error_handler:
    ; Syscall: exit(1) on error
    mov rax, 60
    mov rdi, 1
    syscall
If an error occurs, rax will be negative, and the program will exit with status 1.

Key Takeaways
Syscalls return negative error codes when they fail.
In C, errors are set in errno and can be checked using perror() or strerror().
In Python, errors raise an OSError exception.
In assembly, errors are indicated by a negative return value in rax.


## Linux filesystem APIs

Today, we're going to learn the basic APIs for reading and writing files in Linux.

### Opening a file

`int open(const char* pathname, int flags, mode_t mode)`

- `pathname` is the path to the file you want to open (e.g., `/usr/share/cs644/bigfile.txt`).
- `flags` control how the file should be open.
    - `O_RDONLY` to open for reading only
    - `O_WRONLY` to open for writing only
    - `O_RDWR` to open for reading and writing
    - `O_CREAT` to create if it does not exist
    - `O_APPEND` to append writes to the end of the file
    - `O_TRUNC` to truncate the file's length to 0 if it already exists
- `mode` is used for setting permissions of newly-created files. It's optional unless `O_CREAT` is passed in `flags`. We'll talk more about it next week.
- `open` returns a _file descriptor_, an integer that identifies the open file to the OS. The file descriptor itself holds no information (they just count up from 0); all the bookkeeping is done by the OS.

Linux Filesystem APIs: File Opening and Management
In Linux, interacting with files is done through system calls like open(), read(), write(), and close(). These APIs provide low-level access to files, bypassing higher-level abstractions like C’s fopen() or Python’s open().

Opening a File with open()
The open() system call is used to open a file and obtain a file descriptor.

Function Signature:
c
Copy
Edit
int open(const char *pathname, int flags, mode_t mode);
pathname: The file path (e.g., /home/user/myfile.txt).
flags: Specifies how the file should be opened.
mode: Specifies the permissions if the file is created (only used with O_CREAT).
Return Value:
On success, returns a file descriptor (a small integer ≥ 0).
On failure, returns -1 and sets errno.
Common Flags for open()
Flag	Description
O_RDONLY	Open for reading only
O_WRONLY	Open for writing only
O_RDWR	Open for reading & writing
O_CREAT	Create the file if it doesn’t exist (requires mode)
O_TRUNC	Truncate the file to length 0 if it exists
O_APPEND	Append writes to the end of the file
Examples
1. Open a file for reading (O_RDONLY)
c
Copy
Edit
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

int main() {
    int fd = open("example.txt", O_RDONLY);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }
    printf("File opened successfully, file descriptor: %d\n", fd);
    close(fd);
    return 0;
}
✅ Opens example.txt in read-only mode.
❌ Fails if the file does not exist.

2. Open a file for writing (O_WRONLY | O_CREAT | O_TRUNC)
c
Copy
Edit
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

int main() {
    int fd = open("newfile.txt", O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }
    printf("File opened successfully for writing: %d\n", fd);
    close(fd);
    return 0;
}
✅ Creates newfile.txt if it doesn’t exist.
✅ Truncates newfile.txt to 0 bytes if it exists.
❌ Fails if permissions don’t allow writing.

3. Open a file in append mode (O_WRONLY | O_APPEND)
c
Copy
Edit
#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

int main() {
    int fd = open("logfile.txt", O_WRONLY | O_APPEND);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }
    printf("File opened in append mode: %d\n", fd);
    close(fd);
    return 0;
}
✅ Writes will always go to the end of the file.
✅ Useful for log files.

File Permissions (mode_t)
When using O_CREAT, you must specify file permissions. The most common permissions are:

Mode	Octal	Meaning
S_IRUSR	0400	Read by owner
S_IWUSR	0200	Write by owner
S_IXUSR	0100	Execute by owner
S_IRGRP	0040	Read by group
S_IWGRP	0020	Write by group
S_IROTH	0004	Read by others
Example: 0644 = Owner can read/write, group and others can read.

Closing a File
After finishing with a file, always close it using:

c
Copy
Edit
int close(int fd);
Example:

c
Copy
Edit
close(fd);
Closing a file frees system resources and prevents leaks.

Checking for Errors
Always check if open() fails:

c
Copy
Edit
int fd = open("file.txt", O_RDONLY);
if (fd == -1) {
    perror("Error opening file");
    return 1;
}
perror("message") prints the error reason (e.g., No such file or directory).
errno can be checked manually (#include <errno.h>).
Summary
open() returns a file descriptor (small integer).
Flags control read/write mode (O_RDONLY, O_WRONLY, etc.).
O_CREAT requires a mode (file permissions).
close(fd) releases the file.


### Reading from a file

`ssize_t read(int fd, char* buf, size_t count)`

- `fd` is the file descriptor to read from, as returned by `open`.
- `buf` is the pointer to the array to read into.
- `count` is the maximum number of bytes to read. Make sure that `buf` is at least this long!
- The return value is the number of bytes read, or -1 on error. If you are at the end of file, 0 is returned.

Reading from a File in Linux (read() System Call)
The read() syscall allows you to read data from a file into a buffer.

Function Signature
c
Copy
Edit
ssize_t read(int fd, void *buf, size_t count);
fd: File descriptor returned by open().
buf: Pointer to the buffer where data will be stored.
count: Maximum number of bytes to read.
Return Value
Returns the number of bytes actually read.
Returns 0 if the end of the file (EOF) is reached.
Returns -1 on error (e.g., invalid file descriptor, no permission).
Example: Reading a File
This program reads up to 100 bytes from a file and prints it.

c
Copy
Edit
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>

#define BUFFER_SIZE 100

int main() {
    char buffer[BUFFER_SIZE];
    
    // Open file for reading
    int fd = open("example.txt", O_RDONLY);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }

    // Read from the file
    ssize_t bytesRead = read(fd, buffer, BUFFER_SIZE - 1);
    if (bytesRead == -1) {
        perror("Error reading file");
        close(fd);
        return 1;
    }

    // Null-terminate and print the buffer
    buffer[bytesRead] = '\0';  // Ensure it's a valid string
    printf("File contents:\n%s\n", buffer);

    // Close the file
    close(fd);
    return 0;
}
Handling read() Properly
✅ Check if read() returns -1 to handle errors.
✅ Ensure buffer is large enough for count bytes.
✅ Null-terminate buffer for string-based processing.
✅ Loop until EOF if you need to read the whole file.

Reading an Entire File
Since read() may not read the full file in one call, a loop is used to read chunk by chunk.

c
Copy
Edit
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>

#define BUFFER_SIZE 256

int main() {
    char buffer[BUFFER_SIZE];
    int fd = open("example.txt", O_RDONLY);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }

    ssize_t bytesRead;
    while ((bytesRead = read(fd, buffer, BUFFER_SIZE - 1)) > 0) {
        buffer[bytesRead] = '\0';  // Null-terminate
        printf("%s", buffer);
    }

    if (bytesRead == -1) {
        perror("Error reading file");
    }

    close(fd);
    return 0;
}
Detecting End of File (EOF)
If read() returns 0, you have reached the end of file.
Loop read() calls to process large files properly.
Handling Errors
If read() returns -1, check errno for details:

c
Copy
Edit
#include <errno.h>
#include <string.h>

if (bytesRead == -1) {
    printf("Read error: %s\n", strerror(errno));
}
Common errno values:

Error Code	Meaning
EINTR	Read was interrupted, try again.
EBADF	Invalid file descriptor.
EFAULT	Buffer is in invalid memory.
EIO	I/O error occurred.
EINVAL	Invalid argument.
Summary
read(fd, buffer, count) reads up to count bytes.
Returns the actual bytes read or 0 for EOF.
Use a loop to read large files.
Always check for errors!


### Writing to a file

`ssize_t write(int fd, const char* buf, size_t count)`

- `fd` is the file descriptor to write to, as returned by `open`.
- `buf` is the pointer to the array to write from.
- `count` is the maximum number of bytes to write. Make sure that `buf` is at least this long!
- The return value is the number of bytes written, or -1 on error. Usually it will equal `count`, but not always, for instance if your disk runs out of space.

Writing to a File in Linux (write() System Call)
The write() system call is used to write data from a buffer to a file.

Function Signature
c
Copy
Edit
ssize_t write(int fd, const void *buf, size_t count);
fd: File descriptor returned by open().
buf: Pointer to the buffer containing the data to write.
count: Number of bytes to write.
Return Value
Returns the number of bytes actually written.
Returns -1 on error (e.g., no disk space, no permission).
Might write fewer bytes than requested (count), requiring a loop to complete the write.
Example: Writing to a File
This example writes a string to a file.

c
Copy
Edit
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>

int main() {
    const char *text = "Hello, Linux filesystem!\n";

    // Open file for writing, create if it doesn’t exist, truncate it
    int fd = open("output.txt", O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }

    // Write to the file
    ssize_t bytesWritten = write(fd, text, 25);
    if (bytesWritten == -1) {
        perror("Error writing to file");
        close(fd);
        return 1;
    }

    printf("Successfully wrote %ld bytes to the file.\n", bytesWritten);

    // Close the file
    close(fd);
    return 0;
}
✅ Creates or truncates output.txt.
✅ Writes "Hello, Linux filesystem!" to it.
✅ Checks for errors.

Handling Partial Writes
write() does not guarantee writing all count bytes at once.
If fewer bytes are written, you should loop until all data is written.

c
Copy
Edit
ssize_t full_write(int fd, const char *buf, size_t count) {
    ssize_t totalWritten = 0;
    while (totalWritten < count) {
        ssize_t written = write(fd, buf + totalWritten, count - totalWritten);
        if (written == -1) {
            perror("Error writing to file");
            return -1;
        }
        totalWritten += written;
    }
    return totalWritten;
}
Use it like this:

c
Copy
Edit
full_write(fd, "Complete message\n", 17);
Appending to a File
If you want to append data instead of overwriting:

c
Copy
Edit
int fd = open("output.txt", O_WRONLY | O_CREAT | O_APPEND, 0644);
✅ Preserves existing content
✅ Writes will always be added at the end

Common Errors
Error Code	Meaning
EACCES	No permission to write.
ENOSPC	No space left on the device.
EIO	Disk I/O error.
EBADF	Invalid file descriptor.
EPIPE	Broken pipe (writing to a closed pipe/socket).
Example error handling:

c
Copy
Edit
#include <errno.h>
#include <string.h>

if (write(fd, buf, count) == -1) {
    printf("Write error: %s\n", strerror(errno));
}
Summary
write(fd, buffer, count) writes up to count bytes.
Returns the actual bytes written (may be less than count).
Use a loop for large writes.
Use O_APPEND to append data safely.
Always check for errors!


### Seeking in a file

`off_t lseek(int fd, off_t offset, int whence)`

- The kernel keeps track of "where you are" in the file, e.g., after you read 100 bytes, the next read will start 100 bytes into the file.
- `lseek` lets you explicitly control the position.
- You can probably guess what `fd` is by now.
- `offset` and `whence` together determine the behavior.
- If `whence` is `SEEK_SET`, then `offset` is a fixed offset to jump to.
- If `whence` is `SEEK_CUR`, then `offset` is relative to the current position.
- If `whence` is `SEEK_END`, then `offset` is relative to the end of the file.
- To jump to start of file: `lseek(fd, 0, SEEK_SET)`
- To jump to end of file: `lseek(fd, 0, SEEK_END)`
- The return value is either the new position, or -1 on error.

Seeking in a File with lseek()
The lseek() system call allows manual control over the file offset, which determines where the next read() or write() will occur.

Function Signature
c
Copy
Edit
off_t lseek(int fd, off_t offset, int whence);
fd: File descriptor of the open file.
offset: The number of bytes to move.
whence: Determines how offset is interpreted.
whence Options
Flag	Meaning
SEEK_SET	Move to offset bytes from the beginning.
SEEK_CUR	Move offset bytes from the current position.
SEEK_END	Move offset bytes from the end of the file.
Example: Seeking in a File
This example reads a specific part of a file using lseek().

c
Copy
Edit
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>

int main() {
    int fd = open("example.txt", O_RDONLY);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }

    // Move to byte 10 from the start
    off_t newPos = lseek(fd, 10, SEEK_SET);
    if (newPos == -1) {
        perror("Error seeking in file");
        close(fd);
        return 1;
    }

    // Read from new position
    char buffer[21];
    ssize_t bytesRead = read(fd, buffer, 20);
    if (bytesRead == -1) {
        perror("Error reading file");
        close(fd);
        return 1;
    }

    buffer[bytesRead] = '\0';
    printf("Data from offset 10: %s\n", buffer);

    close(fd);
    return 0;
}
✅ Moves 10 bytes into the file and reads 20 bytes.

Seeking to the Start or End of a File
c
Copy
Edit
lseek(fd, 0, SEEK_SET);  // Jump to the start
lseek(fd, 0, SEEK_END);  // Jump to the end
Get file size:

c
Copy
Edit
off_t fileSize = lseek(fd, 0, SEEK_END);
printf("File size: %ld bytes\n", fileSize);
✅ Moves to the end of the file and returns the total file size.

Seeking Backward or Forward
c
Copy
Edit
lseek(fd, -5, SEEK_CUR);  // Move back 5 bytes from the current position
lseek(fd, 15, SEEK_CUR);  // Move forward 15 bytes
✅ Allows stepping through a file without reading all data sequentially.

Seeking Beyond the End of File (File Hole)
If lseek() moves beyond the current file size, writing will create a file hole (gaps filled with zeros).

c
Copy
Edit
int fd = open("sparsefile.txt", O_WRONLY | O_CREAT, 0644);
lseek(fd, 1000, SEEK_END);  // Move 1000 bytes past end
write(fd, "X", 1);         // Write at new position
close(fd);
✅ Creates a 1001-byte file with 1000 zero bytes followed by X.

Checking for Errors
lseek() returns -1 on failure. Common errors:

Error Code	Meaning
EBADF	Invalid file descriptor.
EINVAL	Invalid whence or offset.
ESPIPE	fd refers to a pipe/socket, which cannot seek.
Example:

c
Copy
Edit
if (lseek(fd, -5, SEEK_SET) == -1) {
    perror("Seek failed");
}
Summary
lseek(fd, offset, SEEK_SET) → Move to absolute position.
lseek(fd, offset, SEEK_CUR) → Move relative to current position.
lseek(fd, offset, SEEK_END) → Move relative to file end.
Use lseek(fd, 0, SEEK_END) to get file size.
Writing past EOF creates "holes".


### Closing a file

`int close(int fd)`

- File descriptors are not an infinite resource: the kernel sets a maximum number of open files per process. So it's a good idea to clean them up when you're done.
- Note this important caveat from the man page: "Typically, filesystems do not flush buffers when a file is closed."
- `fd` is the file descriptor to be closed.
- There's no information to communicate back, so `close` just returns 0 on success and -1 on error.


Closing a File with close() in Linux
The close() system call is used to release a file descriptor once you're done with it. Since the number of open files is limited per process, it's important to close files to free resources.

Function Signature
c
Copy
Edit
int close(int fd);
fd: The file descriptor to close.
Return Value
0 on success.
-1 on error, setting errno.
Why Close Files?
✅ Prevents resource leaks (too many open files cause EMFILE errors).
✅ Ensures proper cleanup (some OSes don't write all data until close()).
✅ Allows other processes to access the file if locks are involved.

Example: Properly Opening, Reading, and Closing a File
c
Copy
Edit
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
        close(fd);  // Always close on failure too!
        return 1;
    }

    buffer[bytesRead] = '\0';
    printf("File contents: %s\n", buffer);

    // Close the file
    if (close(fd) == -1) {
        perror("Error closing file");
        return 1;
    }

    return 0;
}
✅ Closes the file descriptor after use.
✅ Handles errors gracefully.

Flushing Buffers on Close
Closing a file does not guarantee that all data is written to disk immediately.
If you need to ensure data is flushed to disk, use:
c
Copy
Edit
fsync(fd);  // Flushes file data
fsync() ensures data is physically written, but can be slow.
Handling close() Errors
Errors on close() are rare but can happen:

Error Code	Meaning
EBADF	Invalid file descriptor.
EIO	I/O error while closing.
Example:

c
Copy
Edit
if (close(fd) == -1) {
    perror("Close failed");
}
Closing Multiple Files
If a program opens many files, it should ensure all are closed, even on failure.

Example: Closing multiple files in a loop:

c
Copy
Edit
int fds[3] = {fd1, fd2, fd3};

for (int i = 0; i < 3; i++) {
    if (fds[i] != -1 && close(fds[i]) == -1) {
        perror("Error closing file");
    }
}
Summary
Always close files with close(fd).
Closing does not flush buffers to disk (use fsync(fd) if needed).
Handles errors (rare but possible).
Prevents "too many open files" errors (EMFILE).


## In-class exercises


