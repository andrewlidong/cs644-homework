Process basics
A program is a file on disk containing executable code. A process is a program that is loaded in memory and executing on a CPU (or waiting to execute). Every process has a parent (the process who spawned it), so the set of running processes on a system is organized as a tree.

Processes are identified by an integer called a PID. A process's PID is unique while it is running, but may eventually be reused. Two syscalls, getpid and getppid, let you discover your own and your parent's PIDs. Unusually for syscalls, they cannot return an error.

pid_t getpid(void);
pid_t getppid(void);
A process executes as a particular user (and group). In fact, it has both a real UID and an effective UID (and likewise for GIDs):

uid_t getuid(void);
gid_t getgid(void);

uid_t geteuid(void);
gid_t getegid(void);
The real UID is the "actual" user, while the effective user is the user for purposes of access control. Usually they are the same, but not always: if you run sudo sleep 30 & and then ps -eo pid,euser,ruser, you will see that the process's real UID is your UID, but its effective UID is root.

The way this works is that the file /usr/bin/sudo has a special bit set called the set-uid bit. When a program with the set-uid bit is executed, its effective UID is set to the owner of the file rather than the user who launched it. There's an analogous bit for GIDs called the set-gid bit.

Launching processes
Spawning a child process is accomplished by a pair of syscalls: fork and execve.

pid_t fork(void);
fork is the syscall that creates a new child process.

It is a very ancient Unix syscall with a clever design: when it returns, it returns into both processes. In the child process it returns 0, and in the parent process it returns the child's PID.

You can imagine that the original process has split and cloned itself: the child process is running the same program, with a complete copy of the variables, call stack, etc. of the original process.

Normally you'll see:

pid_t pid = fork();
if (pid < 0) {
    // error
} else if (pid == 0) {
    // child
} else {
    // parent
}
It's a little hard to wrap your head around.

A child process copies a lot of state from the parent. Notably:

All open file descriptors
Environment variables
Signal handlers (covered in a later week)
Regardless of if the original process was multi-threaded, the forked process will have only one thread.

Most of the time, you fork because you want to run a different program (e.g., invoke a shell command) – if you actually want to run two copies of the same program, you're probably better off using multithreading, which we'll cover later in the course.

To execute a different program, call execve in the child process:

int execve(const char* pathname, char* argv[], char* envp[]);
execve replaces the current program with the one at pathname, passing it the arguments in argv and the environment variables in envp. It does not create a new process – that's what fork did.

If execve succeeds, it will never return – the old program is "switched out" with the new one.

Usually, you'll want a way for the parent to communicate with its child. We'll cover a number of ways to do so next week; today, we'll just look at waitpid, which is how a parent waits for its child to finish.

pid_t waitpid(pid_t pid, int* wstatus, int options);
pid can be -1 to wait for any child, or a positive number to wait for a specific child. There are a few more possibilities documented in man 2 waitpid. The main useful value for the options parameter is WNOHANG, if you just want to check if a process has exited but don't want to block on waiting for it. Some information will be copied into wstatus, such as the child's exit status.

When a child exits, the kernel sends a SIGCHLD signal to the parent process – we'll talk more about signals later in the course.

If a parent exits before its child, the child process is reassigned to the process with PID 1 as its parent (a system service called init). After a child exits but before its parent calls waitpid, the process is what's called a zombie process. The OS still has to maintain some metadata about the process. So it's important to "reap" child processes to free up these resources.

The last part of a process's lifecycle is exiting:

void _exit(int status);
Calling _exit terminates the process immediately, and registers a status code that can later be retrieved by the parent process when it calls waitpid. Traditionally, an exit code of 0 indicates success and any non-zero value indicates failure.

You more likely want to use the C library function exit() (no leading underscore), which calls exit handlers, flushes output buffers, etc.

