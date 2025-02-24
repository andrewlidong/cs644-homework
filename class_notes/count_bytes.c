#include <fcntl.h>    // For open()
#include <unistd.h>   // For read(), close()
#include <stdio.h>    // For printf(), perror()

#define BUFFER_SIZE 1024  // Read in 1 KB chunks

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Usage: %s <filename>\n", argv[0]);
        return 1;
    }

    // Open the file for reading
    int fd = open(argv[1], O_RDONLY);
    if (fd == -1) {
        perror("Error opening file");
        return 1;
    }

    char buffer[BUFFER_SIZE];
    ssize_t bytesRead;
    size_t totalBytes = 0;

    // Read in chunks until EOF
    while ((bytesRead = read(fd, buffer, BUFFER_SIZE)) > 0) {
        totalBytes += bytesRead;
    }

    if (bytesRead == -1) {
        perror("Error reading file");
        close(fd);
        return 1;
    }

    // Print total file size
    printf("Total file size: %zu bytes\n", totalBytes);

    // Close file descriptor
    close(fd);
    return 0;
}
