# CS644 Homework 1

## (*) What is the `size_t` type for in C? 
`size_t` is an unsigned integer type used to represent sizes of objects in bytes.  Its commonly used for:
    - Representing sizes of memory allocations (e.g., `malloc`)
    - Loop counters when iterating over arrays or buffers.  
    - Functions like `sizeof`, `strlen`, and `memcpy` return or take `size_t`.  

Example: 
```c
#include <stdio.h>

int main() {
    size_t x = sizeof(int);
    printf("Size of int: %zu bytes\n", x);
    return 0;
}
```

## (*) True or False: Semicolons are optional in C.  
False.  In C, semicolons(;) are mandatory to mark the end of a statement.  Omitting them will result in a compilation error.  

## (**) Choose a language and think about what final project you'd like to do.  Neither choice is binding for now.  
I'm choosing Rust.  For my final project I'll make a database server.  

## (**) Does this program have a bug?  Why or why not?  

```c
#include <stdbool.h>

enum LanguageCode {
  LANG_EN, // English
  LANG_FR, // French
  LANG_RU, // Russian
};

struct Language {
  enum LanguageCode code;
  bool has_verb_conjugations;
  bool has_grammatical_gender;
  bool has_noun_declensions;
};

struct Language make_language(enum LanguageCode code) {
  struct Language lang = {
    .code = code,
    .has_verb_conjugations = false,
    .has_grammatical_gender = false,
    .has_noun_declensions = false,
  };

  switch (code) {
    case LANG_RU:
      lang.has_noun_declensions = true;
    case LANG_FR:
      lang.has_verb_conjugations = true;
      lang.has_grammatical_gender = true;
      break;
    case LANG_EN:
      lang.has_verb_conjugations = true;
      break;
  }

  return lang;
}
```
No `break` statements in the `switch` statement inside `make_language`.  This will result in fall-through behavior, meaning that execusion will continue to cascade to next cases.  

## (**) Take a look at this C function from the Python source code.  Try to understand what it does and how it works.  (Bonus: Can you explain how the second function in the file works?)

```c
/* Cross platform case insensitive string compare functions
 */

#include "Python.h"

int
PyOS_mystrnicmp(const char *s1, const char *s2, Py_ssize_t size)
{
    const unsigned char *p1, *p2;
    if (size == 0)
        return 0;
    p1 = (const unsigned char *)s1;
    p2 = (const unsigned char *)s2;
    for (; (--size > 0) && *p1 && *p2 && (Py_TOLOWER(*p1) == Py_TOLOWER(*p2));
         p1++, p2++) {
        ;
    }
    return Py_TOLOWER(*p1) - Py_TOLOWER(*p2);
}

int
PyOS_mystricmp(const char *s1, const char *s2)
{
    const unsigned char *p1 = (const unsigned char *)s1;
    const unsigned char *p2 = (const unsigned char *)s2;
    for (; *p1 && *p2 && (Py_TOLOWER(*p1) == Py_TOLOWER(*p2)); p1++, p2++) {
        ;
    }
    return (Py_TOLOWER(*p1) - Py_TOLOWER(*p2));
}
```

The first function, `PyOS_mystrnicmp`, compares two strings up to a specified number of characters.  It returns the difference in ASCII values between the first differing characters.  

The second function, `PyOS_mystricmp`, compares two strings without a specified length.  It returns the difference in ASCII values between the first differing characters.  

The second function works by comparing the strings character by character, converting them to lowercase if necessary, and returning the difference in ASCII values between the first differing characters.  

## (**) Write a C program, `redact.c`, that takes a string argument and prints out the string with all digits replaced by the character X.  (Hint: Some standard library functions might come in handy.)

```c
#include <stdio.h>
#include <string.h>

void redact(char *str) {
    for (int i = 0; str[i] != '\0'; i++) {
        if (str[i] >= '0' && str[i] <= '9') {
            str[i] = 'X';
        }
    }
}
```

## (***) Research the concept of the stack and the heap for memory allocation.  (Not to be confused with the data structures of the same name.)  When do you use one versus the other?  How do I know if a valud in a C program is allocated on the stack or the heap?  

In C, memory is managed using the stack and the heap.  They are distinct regions of memory.  

- **Stack**: 
    - Used for local variables and function call frames.  
    - Memory is allocated and freed in a last-in, first-out (LIFO) manner.  
    - Typically used for statically allocated variables and function call frames.  
    - Memory is automatically managed and freed when the function returns or the variable goes out of scope.  

- **Heap**: 
    - Used for dynamically allocated memory.  
    - Memory is allocated and freed using functions like `malloc` and `free`.  
    - Memory is managed manually by the programmer.  
    - Memory is not automatically freed when the function returns or the variable goes out of scope.  