# Homework Exercises

## 1 (*) What UID is used to check file permissions? 

The effective UID (EUID) is used to determine file access permissions.  
You can receive it using `geteuid()`.  

## 2 (*) How does the caller of fork know if they are the child or parent process? 

fork() returns: 0 to the child process, the child's PID to the parent process, -1 if the fork fails.  

## 3 (*) What is a zombie process? 

A zombie process is a process that has exited but has not been "reaped" by its parent using wait() or waitpid().  

The system retains minimal information about the process until the parent retrieves the exit status.  

## 4 (**) Final project (database): Implement a delete command for your database program that accepts one or more keys to delete. Fork off a child process for each key to delete. Make sure to use file locking to prevent the child processes from accessing the database simultaneously. If you're ambitious, try using record locking (man 2 fcntl) so that multiple processes can access different parts of the file at the same time. The parent should wait for its children to exit, and print the status of each.

## 5 (★★) Final project (web server): Eventually, we'll want our web server to fork itself into several processes to serve multiple requests concurrently. Let's lay the groundwork this week. Modify the run command to fork off 4 child processes when it starts. Have these children run for variable amounts of time – you can either call a sleep() function, or execve the sleep shell command. Either way, the parent process should wait for all children to exit, and print their status codes. Rather than blocking, it should continue to run its main loop while waiting for the children.

## 6 (★★) How can a parent process view its child's resource usage (e.g., CPU time) after it exits? Find the relevant syscall, and use it to write your own version of the time command.

Use wait4() instead of waitpid().  Example: wait4(pid, &status, 0, &rusage); This gives resource usage (struct rusage), including CPU time.  

## 7 (**) fork is the classic Unix system call, but Linux also ofers something called clone.  Read man 2 clone.  What are the differences from fork? 

clone() is more flexible than fork() and allows fine-grained control over what is shared (memory, file descriptors, etc.)

fork() creates a separate process, while clone() can create threads or lightweight processes.  

## 8 (**) The C standard library offers several wrappers around execve.  Read man 3 exec and implement the wrappers in your language of choice.  

The CC standard library provides several wrappers around execve() such as execl(), execv(), execle(), etc.  

These simplify argument handling (execl() takes variable arguments instead of an array)

execl(path, arg0, arg1, ..., NULL) takes a variable number of arguments.  

execle(path, arg0, arg1, ..., NULL, envp) takes an environment array.  

execlp(file, arg0, arg1, ..., NULL) uses PATH environment variable to locate the executable

execv(path, argv) takes arguments as an array

execvp(file, argv) uses PATH lookup with an array

execve(path, argv, envp) is a raw system call allowing full control over environment variables.  

execvpe(file, argv, envp) combines execvp() and execve().  

## 9 (**) Many programming languages have a high-level way to run a child process, such as Python's subprocess.run.  Write a simple program to demonstrate it, then use strace to determine what syscalls it makes.  (The -f flag lets you see syscalls in child processes as well.)  

```rs
use std::process::{Command, ExitStatus};

fn main() {
    println!("Spawning a child process to run 'ls -l'");

    let output = Command::new("ls")
        .arg("-l")
        .spawn() // Spawn the child process
        .expect("Failed to start process");

    let status: ExitStatus = output.wait().expect("Failed to wait on child");

    println!("Child process exited with status: {}", status);
}

```

## 10 (***) Think through what happens in terms of process relationships and UIDs when you make an SSH connection to a remote server. How does a process running on your laptop (ssh) "transform" itself into a process running on the remote server (bash)? How does it end up with the correct UID?

## 11 (★★★) It's important to follow execve with a call to _exit. What could go wrong if you don't?

## 12 (★★★) Read this post on fork. Do you agree with the author?

