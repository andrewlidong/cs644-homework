1. (★) What's the difference between a syscall and a function call?

Function calls jump between different points in your program; syscalls switch control to the operating system.


Both syscalls (system calls) and function calls involve executing code, but they differ fundamentally in where the code runs and how they interact with the operating system.

A function call is when one function in a program calls another within the same process.  It happens entirely in user space and it direct jumps to another part of the program, and is faster than a syscall because of no privilege switch.  

A syscall is a request from user space to the kernel to perform an operation (like file I/O, memory allocation and process management).  It switches from user mode to kernel mode and uses a special instruction.  It's slower than function calls due to privilege switch.  

1. (★) How do you distinguish between an I/O error and reaching the end of the file with read?

```c
ssize_t read(int fd, void *buf, size_t count);
```

read returns 0 at end of file, and a negative number on an I/O error.


The return value tells you how many bytes were read.  
You have to check the return value to determine if you've reached EOF or encountered an I/O error.  In a normal read read() returns the number of bytes read.  The loop continues processing.  For EOF, read() returns 0 and the program prints "reached end of file".  For an I/O error, read() will return -1 and prints the error message using strerror(errno).  

3. (★) What flags do I pass to open to open a file for writing at the end?

```c
int fd = open("filename.txt", O_WRONLY | O_CREAT | O_APPEND, 0644);
```

O_WRONLY (or O_RDWR) and O_APPEND


O_WRONLY means for writing only
O_CREAT means create the file if it doesn't exist
O_APPEND moves to the end before each write.  
0644 is the file permissions (rw-r--r--) and you need it if you're using O_CREAT

4. (★★) Final project (database): The very first version of your database simply stores key-value pairs to disk. Your program should have two commands: get and set. The set command takes a key and a value and writes it to disk, and the get command takes a key and prints the value, if it exists. You should store all data in a single file (it's okay to hard-code the path – users shouldn't look at the file directly). Use whatever data format you want. It's okay to make assumptions about the data if it simplifies your program (e.g., doesn't contain the | character so you can use that as a delimiter).

(see rust_db/src/main.rs)

5. (★★) Final project (web server): Web servers commonly log some details about incoming requests to a file. We're not ready to handle network requests, so this week we'll just do the logging. Your program should have two commands: run and count. The run command will append a line to a log file and exit. The count command should read the log file and print a count of the number of lines. You can format the log lines however you like, though generally they begin with a timestamp and include a descriptive message.

(see web_logger/src/main.rs)

6. (★★) EACCES, EEXIST, and ENOENT are three common errors that open can return. Read the description of these errors in man 2 open, and write a program that demonstrates each of them.

```c
#include <assert.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>

int main() {
  // Solution to homework exercise #6
  // https://iafisher.com/cs644/spring2025/week2
  int r = open("/root/hello.txt", O_WRONLY | O_CREAT, 0644);
  assert(r < 0);
  assert(errno == EACCES);
  perror("open(\"/root/hello.txt\")");

  r = open("/usr/share/cs644/code/README.md", O_WRONLY | O_CREAT | O_EXCL, 0644);
  assert(r < 0);
  assert(errno == EEXIST);
  perror("open(\"/usr/share/cs644/code/README.md\")");

  r = open("/non/existent/path", O_RDONLY);
  assert(r < 0);
  assert(errno == ENOENT);
  perror("open(\"/non/existent/path\")");
  return 0;
}
```

There are three common errors that open() may return: 

EACCESS: Permission denied, trying to open a file without read/write permissions.  
EEXIST: File already exists, when using O_CREAT
ENOENT: No such file or directory, trying to open a non-existent file in O_RDONLY mode.  

1. (★★) Modify your program from exercise 3 to count the number of whitespace characters in the file. Try it out on /usr/share/cs644/bigfile.txt. Experiment with different chunk sizes. How does it affect the performance of your program? (Tip: Run time ./myprogram to measure the running time of your program.)

```c
#include <ctype.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void usage(char* argv[]) {
  fprintf(stderr, "usage: %s <filename> <bufsz>\n", argv[0]);
}

int is_flag(const char* arg) {
  return arg[0] == '-';
}

int main(int argc, char* argv[]) {
  // Solution to homework exercise #7
  // https://iafisher.com/cs644/spring2025/week2
  const char* filename;
  size_t bufsz = 4096;
  
  // Basic command-line validation:
  //  - first argument is filename
  //  - second argument is buffer size
  // One or both can be omitted. If any argument looks like a flag, abort.
  if (argc == 1) {
    filename = "/usr/share/cs644/bigfile.txt";
  } else if (argc == 2 && !is_flag(argv[1])) {
    filename = argv[1];
  } else if (argc == 3 && !is_flag(argv[1]) && !is_flag(argv[2])) {
    filename = argv[1];
    long long n = atoll(argv[2]);
    if (n <= 0) {
      usage(argv);
      return 1;
    }
    bufsz = (size_t)n;
  } else {
    usage(argv);
    return 1;
  }

  int fd = open(filename, O_RDONLY);
  if (fd < 0) {
    perror("open");
    return 1;
  }

  size_t nbytes = 0;
  size_t space_count = 0;

  // In classic C, we'd need to use `malloc` since the array size is not known at compile
  // time. But since C99, we can have flexible stack-allocated arrays.
  char buf[bufsz];

  while (1) {
    ssize_t bytes_read = read(fd, buf, bufsz);
    if (bytes_read == 0) {
      break;
    } else if (bytes_read < 0) {
      perror("read");
      return 1;
    } else {
      nbytes += bytes_read;
    }

    for (size_t i = 0; i < bytes_read; i++) {
      if (isspace(buf[i])) {
        space_count += 1;
      }
    }
  }

  int err = close(fd);
  if (err < 0) {
    perror("close");
    return 1;
  }

  printf("Bytes:  %lu\n", nbytes);
  printf("Spaces: %lu\n", space_count);
  return 0;
}
```

There are 1,650,564 whitespace characters in the file. Here's a program to measure it. Unsurprisingly, increasing the buffer size makes the program faster. My program took 7,500 ms with a buffer of 1, but only 70–80 ms with a buffer of 1,000. Past around 10,000 bytes, making the buffer bigger did not reliably make it faster, probably because performance became dominated by actual I/O rather than syscall overhead.


2. (★★) Modify your program from exercise 3 to read a file line-by-line.

```c
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "cs644.h"

int is_flag(const char* arg) {
  return arg[0] == '-';
}

char* readline(int fd);

int main(int argc, char* argv[]) {
  const char* filename;
  if (argc == 2 && !is_flag(argv[1])) {
    filename = argv[1];
  } else {
    fprintf(stderr, "usage: %s <filename>\n", argv[0]);
    return 1;
  }

  int fd = open(filename, O_RDONLY);
  if (fd < 0) {
    perror("open");
    return 1;
  }

  char* line;
  unsigned int i = 1;
  while ((line = readline(fd)) != NULL) {
    size_t n = strlen(line);
    printf("line %u: %lu byte(s)\n", i, n);
    printf("line: %s\n", line);
    free(line);
    i++;
  }

  int err = close(fd);
  if (err < 0) {
    perror("close");
    return 1;
  }

  return 0;
}

#define NOT_DONE ((off_t)1)
#define READSZ 16

off_t build_line(struct lpstr* s, char* buf, size_t bufsz) {
  if (bufsz == 0) {
    return 0;
  }

  lpstr_append(s, buf, bufsz);
  ssize_t newline_pos = lpstr_find(*s, '\n');
  if (newline_pos == -1) {
    return NOT_DONE;
  }

  // Make it null-terminated so it's a valid C string. (Remember that the string that
  // `read` returns is *not* null-terminated.) We overwrite the newline, which is
  // convenient because we don't want to return a string with a newline at the end anyway.
  s->data[newline_pos] = '\0';

  // Suppose we read "a\nb".
  //
  //   s->len = 3
  //   newline_pos = 1
  //
  // The file's cursor is at the character after 'b', and we want to set it back by one,
  // so this function should return -1 = (newline_pos - s->len) + 1.
  return (newline_pos - s->len) + 1;
}

char* readline(int fd) {
  struct lpstr s = lpstr_new();
  while (1) {
    char buf[READSZ];
    ssize_t nread = read(fd, buf, READSZ);
    handle_err(nread, "read");

    off_t lseek_offset = build_line(&s, buf, nread);
    if (lseek_offset == NOT_DONE) {
      continue;
    } else {
      off_t r = lseek(fd, lseek_offset, SEEK_CUR);
      handle_err(r, "lseek");
      break;
    }
  }
  return s.data;
}
```

3.  (★★) Why does read return the number of bytes read? Why doesn't it just set buf to a null-terminated string, like other C functions?k

Because files in Linux can hold arbitrary bytes, including the null byte. If read made buf null-terminated, the caller could not distinguish the null terminator from a null byte read from the file.


The read() system call in C is low-level and unbuffered, meaning it simply copies raw bytes from a file descriptor into a buffer.  It does not process or interpret the data.  

read() works with Binary Data, not just strings.  Returning the number of bytes read allows handling any data format.  

It also supports partial reads.  Returning the byte count ensures correct handling of partial reads.  

It leaves buffer management to the programmer.  

Returning bytes read keeps read() fast and efficient.  

1.  (★★) If you call write, use lseek to rewind, and call read again, are you guaranteed to see the data you just wrote? Find the place in the man pages that describes Linux's behavior. Write a program to demonstrate it.

man 2 write says: "POSIX requires that a read(2) that can be proved to occur after a write() has returned will return the new data. Note that not all filesystems are POSIX conforming." Demonstrating program: https://github.com/iafisher/cs644/tree/master/week2/solutions/read-after-write.c

```c
#include <assert.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "cs644.h"

#define BUFSZ 100000

const char* TEMPFILE = "/tmp/ian-test";

void fill_with_char(char*, size_t, char);
void check_all_char(const char*, size_t, char);
void clean_up(void);

int main(int argc, char* argv[]) {
  // Solution to homework exercise #10
  // https://iafisher.com/cs644/spring2025/week2

  int fd = open(TEMPFILE, O_CREAT | O_TRUNC | O_RDWR, 0600);
  handle_err(fd, "open");
  // Call `clean_up` (which removes the temporary file) when the program exits,
  // (`atexit` is a C standard library feature.)
  atexit(clean_up);

  // First, fill the file with spaces and flush to disk.
  char s[BUFSZ];
  fill_with_char(s, BUFSZ, ' ');
  long long r = write(fd, s, BUFSZ);
  assert(r == BUFSZ);
  r = fsync(fd);
  handle_err(r, "fsync");

  // Next, rewind and fill the file with X's, but don't flush.
  r = lseek(fd, 0, SEEK_SET);
  handle_err(r, "lseek");
  fill_with_char(s, BUFSZ, 'X');
  r = write(fd, s, BUFSZ);
  assert(r == BUFSZ);

  // Rewind again and start reading.
  r = lseek(fd, 0, SEEK_SET);
  handle_err(r, "lseek");

  char s2[BUFSZ];
  while (1) {
    ssize_t nread = read(fd, s2, BUFSZ);
    handle_err(nread, "read");
    if (nread == 0) {
      break;
    }

    check_all_char(s2, nread, 'X');
  }

  puts("read-after-write: success");

  r = close(fd);
  handle_err(r, "close");

  return 0;
}

void fill_with_char(char* s, size_t n, char ch) {
  for (size_t i = 0; i < n; i++) {
    s[i] = ch;
  }
}

void check_all_char(const char* s, size_t n, char ch) {
  for (size_t i = 0; i < n; i++) {
    if (s[i] != ch) {
      puts("read-after-write: FAILURE");
      exit(1);
    }
  }
}

void clean_up() {
  // Don't check error code here as clean-up is best-effort.
  unlink(TEMPFILE);
}
```

Not always.  File writes (write()) go through a kernel buffer before being flushed to disk.  lseek(fd, 0, SEEK_SET) moves the file offset but does not flush the buffer.  If you call read() immediately after writing, you may not see the latest data unless the buffer is flushed.  

11. (★★★) Find the location in the Linux kernel source code where a process's table of file descriptors is declared.

The field is struct files_struct *files in struct task_struct (include/sched/linux.h). struct files_struct is defined here, and the actual file representation, struct file, is defined here.


In the Linux kernel, each process maintains a table of its open file descriptors. This table is managed through the files_struct structure, which is referenced within the process's task_struct.


12.  (★★★) What happens when one program is reading from a file while another program is writing? Formulate a hypothesis, then write a pair of programs to test it.

Some plausible hypotheses:
If a program tries to read while another program is in the middle of a write, or vice-versa, the syscall will return with an error.
The OS will allow simultaneous access to a file, but writes will be atomic, so a read will never observe the partial effect of a write.
The OS will allow simultaneous access to a file, and writes will not be atomic, so a read could observe a partial write.
This program shows that it's the third possibility: there's no synchronization between reads and writes of different programs. Even a write as small as 100 bytes is not atomic. In week 3, we'll learn how we can explicitly synchronize access.



When one program is reading from a file while another is writing, the behavior depends on OS buffering, file locks, and whether the writer is appending or overwriting:

If the writer overwrites (O_TRUNC): the reader might see partial data or EOF before the file is completely rewritten.  
The read() call could return inconsistent results if read during the writing process.  
If the writer appends (O_APPEND): the reader might see the new data only after it has been wirten.  Depending on OS buffering, read() might return only old data until refreshed.  
Without synchronization the reader and writer are indpenednet meaning race conditions could cause unpredictable results. The reader might read a mix of old and new data.  