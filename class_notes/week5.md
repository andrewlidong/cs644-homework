Week 5: Interprocess communication

Last week we learned how to create new processes. But we couldn't interact with them except to wait until they exited. This week, we'll learn some techniques for communicating between different processes – interprocess communication, or IPC. We'll cover pipes and shared memory with synchronization via semaphores. Later in the course, we'll also talk about Unix domain sockets and signals.

Pipes
A pipe lets you pass data in a one-way stream from one process to another. A shell command like ps aux | grep myprocess uses a pipe under the hood.

You call pipe2:

int pipe2(int pipefd[2], int flags);
And you get back a file descriptor for the read end in pipefd[0], and a file descriptor for the write end in pipefd[1]. Then you can use read and write as if it were an actual file.

You can use both ends of the pipe in the same process, and occasionally that is useful. But normally you want one end of the pipe in one process and the other end in another. Solution: call pipe2, then fork:

int pipefd[2];
int r = pipe2(pipefd, 0);
if (r < 0) { bail("pipe2"); }

pid_t pid = fork();
if (pid < 0) {
    bail("fork");
} else if (pid == 0) {
    // child
    close(pipefd[0]);
    // write to pipefd[1]
} else {
    // parent
    close(pipefd[1]);
    // read from pipefd[0]
}

When you fork, you end up with both file descriptors open in both processes. Each process only needs one (the read end or the write end), so it can close the one it doesn't need. File descriptors are per-process metadata, so closing one in the child process does not close it in the parent process, or vice versa.

An important limitation of pipes is that they require the two processes to be related to each other, e.g. parent and child. But they are perfect for when you want to pass data from a child to a parent or vice versa.

Shared memory and semaphores
Shared memory is an efficient form of IPC that avoids copying by mapping the same memory region into multiple processes' address spaces. This lets us transparently share in-memory data structures between unrelated processes, just as if they were threads of the same process. Of course, we'll need some way to synchronize access. For that, we can use a kernel synchronization primitive called named semaphores.

Creating a shared memory region is a three-step process. First, we need to open the shared memory object itself with shm_open, which looks very similar to open:

int fd = shm_open("/my-program-mem", O_CREAT | O_EXCL | O_RDWR, 0600);
The name is not a file path; shared memory objects have their own namespace. It should always begin with a slash and contain no other slashes.

The shared memory object starts out empty, so we must resize it:

int r = ftruncate(fd, sizeof my_data_structure);
Finally, we can map it into our process's address space:

struct my_data_structure* s = mmap(
    NULL, sizeof my_data_structure, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0,
);
if (s == MAP_FAILED) {
    // handle error
}
Now, we can use s normally, and any writes will be visible to any other process using it (and we will likewise see any other process's writes).

But we shouldn't start using it until we've set up a semaphore to synchronize access:

sem_t* sem = sem_open("/my-program-sem", O_CREAT | O_EXCL, 0600, 1);
Similar to shm_open, the name passed to sem_open is not a file path. The fourth argument is the initial value of the semaphore – setting it to 1 makes the semaphore effectively a lock that only one process can hold at a time.

We can then wait for the semaphore to be available:

int r = sem_wait(&sem);
And when we are done, release it:

int r = sem_post(&sem);
Finally, at the end of our program we should clean everything up:

close(fd);
shm_unlink("/my-program-mem");
sem_unlink("/my-program-sem");
Putting it all together:

const char* mem_pathname = "/my-program-mem";
const char* sem_pathname = "/my-program-sem";

void writer() {
    // NOTE on error handling: At various points we bail without cleaning up, e.g.
    // calling `shm_unlink`. A more robust program should still clean up resources
    // even in case of error.

    int fd = shm_open(mem_pathname, O_CREAT | O_EXCL | O_RDWR, 0600);
    if (fd < 0) { bail("shm_open"); }
    int r = ftruncate(fd, sizeof my_data_structure);
    if (r < 0) { bail("ftruncate"); }

    struct my_data_structure* s;
    s = mmap(NULL, sizeof my_data_structure, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (s == MAP_FAILED) { bail("mmap"); }

    // After we `mmap`, we can close the shared-memory file descriptor.
    r = close(fd);
    if (r < 0) { bail("close"); }

    sem_t* sem = sem_open(sem_pathname, O_CREAT | O_EXCL, 0600, 1);
    if (sem == SEM_FAILED) { bail("sem_open"); }

    r = sem_wait(&sem);
    if (r < 0) { bail("sem_wait"); }
    // ... use shared data structure ...
    r = sem_post(&sem);
    if (r < 0) { bail("sem_post"); }

    r = sem_unlink(sem_pathname)
    if (r < 0) { bail("sem_unlink"); }

    r = shm_unlink(mem_pathname);
    if (r < 0) { bail("shm_unlink"); }
}

void reader() {
    int fd = shm_open(pathname, O_RDONLY);
    if (fd < 0) { bail("shm_open"); }

    struct my_data_structure* s;
    s = mmap(NULL, sizeof my_data_structure, PROT_READ, MAP_SHARED, fd, 0);
    if (s == MAP_FAILED) { bail("mmap"); }

    // After we `mmap`, we can close the shared-memory file descriptor.
    int r = close(fd);
    if (r < 0) { bail("close"); }

    sem_t* sem = sem_open(sem_pathname, 0);
    if (sem == SEM_FAILED) { bail("sem_open"); }

    r = sem_wait(&sem);
    if (r < 0) { bail("sem_wait"); }
    // ... use shared data structure ...
    r = sem_post(&sem);
    if (r < 0) { bail("sem_post"); }
}
