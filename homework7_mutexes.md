(★) What is (a) the syscall and (b) the library function for starting a new thread?

Linux syscall: clone

The clone syscall is the low-level system call that the kernel provides to create a new process or thread. It allows fine-grained control over what is shared between the parent and the child (memory space, file descriptors, etc.).

POSIX function: pthread_create

This is the standard C library function used to create a new thread in user-space applications.

(★) What's the difference between pthread_exit and exit?

pthread_exit
Exits the calling thread only.

Other threads in the process keep running.

It allows the thread to return a specific value to pthread_join.

It cleans up resources specific to the thread, but leaves the process alive if other threads are still running.

exit
Terminates the entire process, including all threads.

Calls cleanup handlers, flushes stdio buffers, etc.

The whole process ends with the given status code.

(★) What does it mean that multithreading is preemptive?

It means the operating system's scheduler can interrupt a running thread at any time to switch to another thread — even if the current thread isn’t done or hasn’t explicitly yielded.

The OS uses timer interrupts to periodically check if a thread has used its time slice.

If so, it can preempt that thread and schedule a different one, even if the first thread is in the middle of executing.

This happens transparently — the thread doesn’t need to cooperate or call yield.

(★★) Final project (database): It's time to make the database multithreaded! In week 4, you spun up multiple processes to do database operations. Convert these to use threads instead of processes. You can also convert it to use pthreads locks instead of file locks, but think about the pros and cons. Bonus: make the socket interface multithreaded, too.

(★★) Final project (web server): It's time to make the web server multithreaded! It's a good idea for a web server to have multiple threads so it can serve more than one request simultaneously. Instead of using multiple processes, use multiple threads. You can choose whether to start a new thread for every request, or have a fixed pool of threads created at start-up.

(★★) What happens if main returns (without calling exit) while other threads are still active?

If main() returns without calling exit(), it's equivalent to calling exit(status) — which means:

The entire process terminates immediately, including all threads, no matter what they’re doing.

(★★) What syscalls do pthreads locks use under the hood? Write a program and run it under strace to find out.

```c
#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

pthread_mutex_t lock;

void* worker(void* arg) {
    pthread_mutex_lock(&lock);
    printf("Thread got the lock\n");
    sleep(1);
    pthread_mutex_unlock(&lock);
    return NULL;
}

int main() {
    pthread_t t1, t2;

    pthread_mutex_init(&lock, NULL);
    pthread_create(&t1, NULL, worker, NULL);
    pthread_create(&t2, NULL, worker, NULL);

    pthread_join(t1, NULL);
    pthread_join(t2, NULL);

    pthread_mutex_destroy(&lock);
    return 0;
}
```

🛠️ Compile it:

```bash
gcc -pthread -o mutex_test mutex_test.c
```

🔍 Run with strace:

```bash
strace ./mutex_test 2>&1 | grep futex
```

pthread_mutex_lock() and pthread_mutex_unlock() use the **futex** syscall under the hood on Linux.

futex = fast userspace mutex.

It’s designed so that if there’s no contention, locking/unlocking is entirely done in user space (fast).

(★★) Write a program that demonstrates a deadlock due to acquiring locks in different orders.

```c

#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

pthread_mutex_t mutex1 = PTHREAD_MUTEX_INITIALIZER;
pthread_mutex_t mutex2 = PTHREAD_MUTEX_INITIALIZER;

void* thread_func1(void* arg) {
    pthread_mutex_lock(&mutex1);
    printf("Thread 1 acquired mutex1\n");
    sleep(1);  // Give thread 2 time to lock mutex2
    pthread_mutex_lock(&mutex2);
    printf("Thread 1 acquired mutex2\n");

    pthread_mutex_unlock(&mutex2);
    pthread_mutex_unlock(&mutex1);
    return NULL;
}

void* thread_func2(void* arg) {
    pthread_mutex_lock(&mutex2);
    printf("Thread 2 acquired mutex2\n");
    sleep(1);  // Give thread 1 time to lock mutex1
    pthread_mutex_lock(&mutex1);
    printf("Thread 2 acquired mutex1\n");

    pthread_mutex_unlock(&mutex1);
    pthread_mutex_unlock(&mutex2);
    return NULL;
}

int main() {
    pthread_t t1, t2;

    pthread_create(&t1, NULL, thread_func1, NULL);
    pthread_create(&t2, NULL, thread_func2, NULL);

    pthread_join(t1, NULL);
    pthread_join(t2, NULL);

    return 0;
}

```

🛠️ Compile and Run:

```bash
gcc -pthread deadlock.c -o deadlock
./deadlock
```

Thread 1 acquired mutex1
Thread 2 acquired mutex2
...and then it hangs forever — that's the deadlock.

(★★★) Read the list of thread-unsafe functions in man 7 pthreads. Are there any surprises on the list? Why would a function not be thread-safe?

man 7 pthreads
Then search (/) for Thread-Safety.

1. strtok
You might assume it's safe since it's just parsing a string.

But it uses a static internal buffer to keep track of where it is between calls.

Multiple threads calling it will clobber each other's state.

2. ctime, localtime, etc.
These seem innocent — just convert time structs to strings.

But they return pointers to static memory — shared across threads!

3. rand
It keeps state in global variables (static unsigned long next or similar).

So multiple threads racing to call rand() will corrupt the RNG state.

(★★★) Many higher-level languages have restrictions on multithreading (e.g., until very recently Python code could not execute in multiple threads simultaneously). Why is this? What makes multithreading in high-level languages hard?

High-level languages often trade off raw performance for safety, simplicity, and portability — and true multithreading introduces complex problems in memory management, object models, and runtime environments.

In CPython, only one thread executes Python bytecode at a time, even on multi-core CPUs.

The GIL exists to protect internal interpreter state, especially the memory management (e.g. reference counting).

Without it, you’d need fine-grained locks all over the place — hard to get right, very easy to introduce race conditions.

🔁 So: Python has threads, but they don’t run Python code in parallel — just C extensions or I/O operations.

2. Garbage Collection (GC) and Thread Safety
High-level languages (Python, Ruby, JavaScript) use garbage collectors.

GC needs to scan, move, and collect memory — safely — even while the program is running.

Adding real multithreading means the GC must be thread-safe, which is hard and expensive to implement and maintain.

3. Complex Object Models
Languages like Python and Ruby have mutable objects, dynamic types, and duck typing.

This makes reasoning about memory and data races much harder.

Lower-level languages (like C/C++) give you full control, but also full responsibility.

4. Developer Simplicity & Safety
High-level languages are designed for rapid development and ease of use.

True parallelism often introduces:

Race conditions

Deadlocks

Subtle memory bugs

Many language designers avoid exposing full multithreading unless they can guarantee safety.

5. Portability across Platforms
High-level languages aim to run on Windows, Linux, macOS, etc.

Threading primitives and memory models vary between platforms.

A conservative threading model (like the GIL) is easier to keep consistent.

Multithreading is hard because it introduces issues with memory management, GC, shared mutable state, and runtime complexity. High-level languages often restrict it to keep things safe, portable, and easy to reason about.

