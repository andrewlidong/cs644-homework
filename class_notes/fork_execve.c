#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
    pid_t pid = fork(); // create a child process

    if (pid < 0) {
        // fork failed
        perror("fork failed");
        exit(1);
    } else if (pid == 0) {
        // child process
        printf("Child process (PID: %d) executing 'ls -l'\n", getpid());

        // prepare arguments for execve
        char *args[] = {"/bin/ls", "-l", NULL};
        execve("/bin/ls", args, NULL);

        // if execve fails
        perror("execve failed");
        exit(1);
    } else {
        // parent process
        printf("Parent process (PID: %d), waiting for child (PID: %d) to finish...\n", getpid(), pid);
        wait(NULL); // wait for child to finish
        printf("Child process finished.\n");
    }

    return 0;
}

/*
The fork() call creates a child process
The child process receives 0 as the return value
The parent process receives the child's PID

The child process calls execve to replace itself with /bin/ls -l
execve() takes:
- path to the file to execute
- arguments to the file
- environment variables

The execve() call never returns if it succeeds
The path to the executable
An array of arguments where args[0] is the command name
NULL for environment variables (using default)

The parent waits for the child to complete using wait(NULL)
The parent prints messages before and after the child finishes
*/