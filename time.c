#include <sys/types.h>
#include <sys/wait.h>
#include <sys/resource.h>
#include <sys/time.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s command [args...]\n", argv[0]);
        exit(1);
    }

    struct rusage usage;
    pid_t pid = fork();

    if (pid < 0) {
        perror("fork failed");
        exit(1);
    }

    if (pid == 0) {  // Child process
        execvp(argv[1], &argv[1]);
        perror("execvp failed");
        exit(1);
    } else {  // Parent process
        int status;
        wait4(pid, &status, 0, &usage);

        // Print resource usage statistics
        printf("\nResource usage statistics:\n");
        printf("User CPU time: %ld.%06ld seconds\n", 
               usage.ru_utime.tv_sec, usage.ru_utime.tv_usec);
        printf("System CPU time: %ld.%06ld seconds\n",
               usage.ru_stime.tv_sec, usage.ru_stime.tv_usec);
        printf("Maximum resident set size: %ld KB\n", usage.ru_maxrss);
    }

    return 0;
} 