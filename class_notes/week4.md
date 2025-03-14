Week 4: Process control

Process basics
A program is a file on disk containing executable code. A process is a program that is loaded in memory and executing on a CPU (or waiting to execute). Every process has a parent (the process who spawned it), so the set of running processes on a system is organized as a tree.

Processes are identified by an integer called a PID. A process's PID is unique while it is running, but may eventually be reused. Two syscalls, getpid and getppid, let you discover your own and your parent's PIDs. Unusually for syscalls, they cannot return an error.

```c
pid_t getpid(void);
pid_t getppid(void);
```

A process executes as a particular user (and group). In fact, it has both a real UID and an effective UID (and likewise for GIDs):

```c
uid_t getuid(void);
gid_t getgid(void);

uid_t geteuid(void);
gid_t getegid(void);
```

The real UID is the "actual" user, while the effective user is the user for purposes of access control. Usually they are the same, but not always: if you run sudo sleep 30 & and then ps -eo pid,euser,ruser, you will see that the process's real UID is your UID, but its effective UID is root.

The way this works is that the file /usr/bin/sudo has a special bit set called the set-uid bit. When a program with the set-uid bit is executed, its effective UID is set to the owner of the file rather than the user who launched it. There's an analogous bit for GIDs called the set-gid bit.

