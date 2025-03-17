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

