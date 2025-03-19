(★) List the key differences between pipes and shared memory as forms of IPC.

Pipes: 
- One way communication
- Used for communication between related processes
- Data is lost when the reader is reading from the pipe
- No explicit synchronization
- Slower due to frequent kernel-space and user-space context switches
- Limited by the pipe buffer size
- Typically used between parent and child processes
- Kernel manages access, making it more secure
- Simple, sequential way to transfer small amounts of data
- Working with parent-child process communication
- Don't require persistence of data after reading

Shared Memory: 
- Two way communication
- Used for communication between unrelated processes
- Data is not lost when the reader is reading from the shared memory
- Faster than pipes
- Explicit synchronization
- No limit on the size of the data that can be sent
- Typically used between unrelated processes
- Requires synchronization mechanisms (semaphores, mutexes, etc.) to prevent race conditions
- Can accomodate larger amounts of data
- Requires explicit access control
- Need high-speed communication with minimal overhead
- Sharing large amounts of data between multiple processes
- Can handle explicit synchronization to prevent race conditions


(★) What format should be used for the names of shared memory objects and semaphores?

1. Must begin with a slash (/)
2. Cannot contain additional /
3. Should be unqiue across the system to avoid conflicts
4. Naming should be descriptive

Shared Memory: 
- /shm_name
- /shm_name_<pid>

Semaphores: 
- /sem_name
- /sem_name_<pid>

(★) Explain how two different processes end up with opposite ends of a pipe.

1. Parent process creates the pipe
2. Parent process forks a child process
3. The child process inherits the pipe file descriptor from the parent process
4. The child process closes the read end of the pipe
5. The parent process closes the write end of the pipe
6. The child process can now read from the pipe
7. The parent process can now write to the pipe

Pipes are inherited by child processes after a fork() call.  
Each process must close the unused end to prevent blocking.  
Parent writes fd[1] -> child reads fd[0]
if reversed, another pipe would be needed
If the read end is closed, writing causes a SIGPIPE error
If the write end is closed, reading returns EOF.  

(★★) Final project (database): Let's start turning our database into a proper long-running server instead of a collection of short-lived commands. Add a serve command that forks off a few child processes. The parent process should first create a shared memory object holding a cache of key-value pairs. The children should open this cache. Write get_with_cache and set_with_cache subroutines that use the cache; in a later week, we'll see how to use advanced IPC to send get and set requests to the child processes.

(★★) Final project (web server): Our web server currently spawns a few child processes to handle requests. We may want these worker processes to share config values, and to respond to updates to them live. Create a config data structure with whatever parameters you like, e.g. int verbosity. Have the parent process create a shared memory object that the child processes open. Add a separate update-config command that lets update the config while the server is running. The child processes should loop and detect updates to the config.

(★★) Do we get any atomicity guarantees when working with pipes? Read man 7 pipe to find out.

Writes of PUP_BUF_SIZE bytes or less are atomic.  This means that a write of 1 byte will always be atomic.  A write of 2 bytes may be atomic, but it is not guaranteed.  A write of 4096 bytes is atomic.

Writes exceeding PUP_BUF_SIZE bytes are not atomic.  They are split into multiple writes of PUP_BUF_SIZE bytes.  The kernel may also split the write in the middle of a struct or other data type.

(★★) What could go wrong if we don't use semaphores to synchronize access to shared memory? Write an example program to demonstrate the problem.

1. Race condition -> multiple processes might read and write to the shared memory at the same time, leading to inconsistent data
2. Data Inconcisitency -> if one process is reading while another is writing, the data may be inconsistent Partial updates from one process might be visible to another process before completion.  
3. Lost Updates -> if a process is writing to the shared memory while another is reading, the reading process may read an incomplete or inconsistent state of the data.  
4. Read-While-Right Issues -> if a process is reading from the shared memory while another is writing, the reading process may read a partial or inconsistent state of the data.  

(★★) Linux has a concept called FIFOs (first-in, first-out) that overcome some of the limitations of pipes. Research them (man 7 pipe and man 7 fifo may help). What system calls do they use? What are the differences from regular pipes?

(★★) What is the buffering behavior of pipes? Pose a hypothesis, then write a test program to find out.

(★★) We used mmap for shared memory, but the system call is more versatile than just that. Read the man page and find out what else it can be used for.

(★★★) Shared memory is often used for high-performance concurrent applications. Implement a ring buffer in shared memory with one producer process putting work into the buffer, and multiple consumer processes reading from it.

(★★★) High-level languages have a way for a parent process to capture its child's stdout and stderr (e.g., capture_output=True in subprocess.run in Python). Use pipe2, fork, and execve to spawn an external program, e.g., echo hello world, and capture its output in a string in the parent program.