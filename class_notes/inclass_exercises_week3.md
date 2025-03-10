What permissions are required to rename a file?

Renaming a file in Linux using the rename() syscall (or mv command) does not depend on the file's permissions but rather on the permissions of the directory containing the file.  

You need to have write permission to modify the directory's contents, and execute permission to access and modify files within the directory.  

If the target name (newfile) already exists, you need write permission on the directory to remove the existing file.  

Let's rewrite our program from last week to more efficiently find a file's size.

```c
#include <stdio.h>
#include <sys/stat.h>

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <filename>\n", argv[0]);
        return 1;
    }

    struct stat file_stat;
    
    // Get file metadata using stat()
    if (stat(argv[1], &file_stat) == 0) {
        printf("File size of '%s': %ld bytes\n", argv[1], file_stat.st_size);
    } else {
        perror("stat");
        return 1;
    }

    return 0;
}
```