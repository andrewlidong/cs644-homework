#include <stdio.h>
#include <string.h>

void redact(char *str) {
    for (int i = 0; str[i] != '\0'; i++) {
        if (str[i] >= '0' && str[i] <= '9') {
            str[i] = 'X';
        }
    }
}

int main() {
    char str[] = "Hello, 123 World!";
    redact(str);
    printf("%s\n", str);
    return 0;
}

// #include <stdio.h>
// #include <ctype.h>

// int main() {
//     int ch;
//     while ((ch = getchar()) != EOF) {  // Read from stdin
//         putchar(isdigit(ch) ? 'X' : ch);
//     }
//     return 0;
// }
