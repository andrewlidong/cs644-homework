#include <sys/types.h>
#include <sys/resource.h>
#include <sys/time.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>

void print_usage(struct rusage *usage) {
    printf("Resource usage statistics:\n");
    printf("User CPU time: %ld.%06ld seconds\n", 
           usage->ru_utime.tv_sec, usage->ru_utime.tv_usec);
    printf("System CPU time: %ld.%06ld seconds\n",
           usage->ru_stime.tv_sec, usage->ru_stime.tv_usec);
    printf("Maximum resident set size: %ld KB\n", usage->ru_maxrss);
    printf("Page faults: %ld\n", usage->ru_majflt);
    printf("Block I/O operations: in=%ld, out=%ld\n\n", 
           usage->ru_inblock, usage->ru_oublock);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <pid>\n", argv[0]);
        exit(1);
    }

    pid_t pid = atoi(argv[1]);
    struct rusage usage;

    // Monitor process every second
    while(1) {
        if (getrusage(RUSAGE_SELF, &usage) == -1) {
            perror("getrusage failed");
            exit(1);
        }
        
        print_usage(&usage);
        sleep(1);  // Wait for 1 second before next reading
    }

    return 0;
} 