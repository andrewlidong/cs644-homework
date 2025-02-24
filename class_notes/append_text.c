#include <fcntl.h>    // For open(), O_APPEND
#include <unistd.h>   // For write(), close(), lseek()
#include <stdio.h>    // For printf(), perror()
#include <string.h>   // For strlen()

#define TEXT "This is an appended line.\n"

void append_with_o_append(const char *filename) {
    int fd = open(filename, O_WRONLY | O_CREAT | O_APPEND, 0644);
    if (fd == -1) {
        perror("Error opening file with O_APPEND");
        return;
    }

    if (write(fd, TEXT, strlen(TEXT)) == -1) {
        perror("Error writing to file with O_APPEND");
    }

    close(fd);
}

void append_with_lseek(const char *filename) {
    int fd = open(filename, O_WRONLY | O_CREAT, 0644);
    if (fd == -1) {
        perror("Error opening file with lseek");
        return;
    }

    if (lseek(fd, 0, SEEK_END) == -1) {
        perror("Error seeking to end of file");
        close(fd);
        return;
    }

    if (write(fd, TEXT, strlen(TEXT)) == -1) {
        perror("Error writing to file with lseek");
    }

    close(fd);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("Usage: %s <filename>\n", argv[0]);
        return 1;
    }

    printf("Appending using O_APPEND...\n");
    append_with_o_append(argv[1]);

    printf("Appending using lseek...\n");
    append_with_lseek(argv[1]);

    printf("Done. Check %s.\n", argv[1]);
    return 0;
}
