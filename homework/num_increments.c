/*
This program creates shared memory where two processes increment a counter without synchronization. Due to race conditions, some increments will be lost.

Expected Behavior (With Proper Sync):
Each child should increment 100 times, leading to a final value of 200.

Observed Behavior (Without Sync):
Final value is often less than 200 due to lost updates.


*/

#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#define NUM_INCREMENTS 100

int main() {
    int *shared_counter = mmap(NULL, sizeof(int), PROT_READ | PROT_WRITE,
                               MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    
    if (shared_counter == MAP_FAILED) {
        perror("mmap failed");
        exit(1);
    }

    *shared_counter = 0;  // Initialize shared counter

    pid_t pid = fork();

    if (pid < 0) {
        perror("fork failed");
        exit(1);
    }

    // Both Parent and Child Increment Counter
    for (int i = 0; i < NUM_INCREMENTS; i++) {
        (*shared_counter)++;  // Race condition here
    }

    if (pid > 0) {  // Parent Process
        wait(NULL);  // Wait for child to finish
        printf("Final Counter Value: %d (Expected: 200)\n", *shared_counter);
        
        // Cleanup
        munmap(shared_counter, sizeof(int));
    }

    return 0;
}
