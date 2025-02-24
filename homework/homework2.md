1. (★) What's the difference between a syscall and a function call?

Both syscalls (system calls) and function calls involve executing code, but they differ fundamentally in where the code runs and how they interact with the operating system.

A function call is when one function in a program calls another within the same process.  It happens entirely in user space and it direct jumps to another part of the program, and is faster than a syscall because of no privilege switch.  

A syscall is a request from user space to the kernel to perform an operation (like file I/O, memory allocation and process management).  It switches from user mode to kernel mode and uses a special instruction.  It's slower than function calls due to privilege switch.  

2. (★) How do you distinguish between an I/O error and reaching the end of the file with read?

```c
ssize_t read(int fd, void *buf, size_t count);
```

The return value tells you how many bytes were read.  
You have to check the return value to determine if you've reached EOF or encountered an I/O error.  In a normal read read() returns the number of bytes read.  The loop continues processing.  For EOF, read() returns 0 and the program prints "reached end of file".  For an I/O error, read() will return -1 and prints the error message using strerror(errno).  

3. (★) What flags do I pass to open to open a file for writing at the end?

```c
int fd = open("filename.txt", O_WRONLY | O_CREAT | O_APPEND, 0644);
```

O_WRONLY means for writing only
O_CREAT means create the file if it doesn't exist
O_APPEND moves to the end before each write.  
0644 is the file permissions (rw-r--r--) and you need it if you're using O_CREAT

4. (★★) Final project (database): The very first version of your database simply stores key-value pairs to disk. Your program should have two commands: get and set. The set command takes a key and a value and writes it to disk, and the get command takes a key and prints the value, if it exists. You should store all data in a single file (it's okay to hard-code the path – users shouldn't look at the file directly). Use whatever data format you want. It's okay to make assumptions about the data if it simplifies your program (e.g., doesn't contain the | character so you can use that as a delimiter).

(see rust_db/src/main.rs)

5. (★★) Final project (web server): Web servers commonly log some details about incoming requests to a file. We're not ready to handle network requests, so this week we'll just do the logging. Your program should have two commands: run and count. The run command will append a line to a log file and exit. The count command should read the log file and print a count of the number of lines. You can format the log lines however you like, though generally they begin with a timestamp and include a descriptive message.

(see web_logger/src/main.rs)

6. (★★) EACCES, EEXIST, and ENOENT are three common errors that open can return. Read the description of these errors in man 2 open, and write a program that demonstrates each of them.

There are three common errors that open() may return: 

EACCESS: Permission denied, trying to open a file without read/write permissions.  
EEXIST: File already exists, when using O_CREAT
ENOENT: No such file or directory, trying to open a non-existent file in O_RDONLY mode.  

7. (★★) Modify your program from exercise 3 to count the number of whitespace characters in the file. Try it out on /usr/share/cs644/bigfile.txt. Experiment with different chunk sizes. How does it affect the performance of your program? (Tip: Run time ./myprogram to measure the running time of your program.)

8. (★★) Modify your program from exercise 3 to read a file line-by-line.

9.  (★★) Why does read return the number of bytes read? Why doesn't it just set buf to a null-terminated string, like other C functions?k

The read() system call in C is low-level and unbuffered, meaning it simply copies raw bytes from a file descriptor into a buffer.  It does not process or interpret the data.  

read() works with Binary Data, not just strings.  Returning the number of bytes read allows handling any data format.  

It also supports partial reads.  Returning the byte count ensures correct handling of partial reads.  

It leaves buffer management to the programmer.  

Returning bytes read keeps read() fast and efficient.  

10. (★★) If you call write, use lseek to rewind, and call read again, are you guaranteed to see the data you just wrote? Find the place in the man pages that describes Linux's behavior. Write a program to demonstrate it.

Not always.  File writes (write()) go through a kernel buffer before being flushed to disk.  lseek(fd, 0, SEEK_SET) moves the file offset but does not flush the buffer.  If you call read() immediately after writing, you may not see the latest data unless the buffer is flushed.  

11. (★★★) Find the location in the Linux kernel source code where a process's table of file descriptors is declared.

In the Linux kernel, each process maintains a table of its open file descriptors. This table is managed through the files_struct structure, which is referenced within the process's task_struct.


12. (★★★) What happens when one program is reading from a file while another program is writing? Formulate a hypothesis, then write a pair of programs to test it.

When one program is reading from a file while another is writing, the behavior depends on OS buffering, file locks, and whether the writer is appending or overwriting:

If the writer overwrites (O_TRUNC): the reader might see partial data or EOF before the file is completely rewritten.  
The read() call could return inconsistent results if read during the writing process.  
If the writer appends (O_APPEND): the reader might see the new data only after it has been wirten.  Depending on OS buffering, read() might return only old data until refreshed.  
Without synchronization the reader and writer are indpenednet meaning race conditions could cause unpredictable results. The reader might read a mix of old and new data.  