(★) Do syscalls follow hard links?

Yes, although to be pedantic it's misleading to speak of "following" hard links in the same way as symbolic links, since there's not really a link to be followed but a direct reference to the file's content and metadata.


(★) True or false: If I acquire an exclusive lock on a file, no other process will be able to write to it.

False. Locks are advisory, so another process that does not even attempt to acquire the lock will not be prevented from writing to it.

(★) How is the owner of a new file or directory set?

The owner of the new file is set to the effective user ID of the process that created it. We'll learn in week 4 what exactly the "effective user ID" is.

(★★) Write a simple version of rm -rf. If your language doesn't expose getdents64 directly, you can use a higher-level API. Warning: Depending on your language, listing the directory may include the special .. entry. Make sure to filter this out! Otherwise you might try to recursively delete the whole tree.

```c
#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include "cs644.h"

void remove_file(const char* pathname);
void remove_dir(const char* pathname);

// TODO: use unlinkat to avoid having to construct paths

void remove_file_or_dir(const char* pathname) {
  struct stat statbuf;
  int r = stat(pathname, &statbuf);
  cs644_bail_if_err(r, "stat");

  switch (statbuf.st_mode & S_IFMT) {
    case S_IFLNK:
    case S_IFREG:
      remove_file(pathname);
      return;
    case S_IFDIR:
      remove_dir(pathname);
      return;
    default:
      cs644_bail("not removing unknown file type");
  }
}

void remove_file(const char* pathname) {
  int r = unlink(pathname);
  cs644_bail_if_err(r, "unlink");
}

char* make_subpath(const char* parent, const char* child) {
  size_t n1 = strlen(parent);
  size_t n2 = strlen(child);

  char* r = cs644_malloc_or_bail((n1 + n2 + 2) * sizeof *r);
  strcpy(r, parent);
  r[n1] = '/';
  strcpy(r + n1 + 1, child);
  return r;
}

void remove_dir(const char* pathname) {
  DIR* dir = opendir(pathname);
  if (dir == NULL) {
    perror("opendir");
    exit(1);
  }

  struct dirent* ent;
  while (1) {
    errno = 0;
    ent = readdir(dir);
    if (ent == NULL) {
      if (errno == 0) {
        break;
      } else {
        perror("readdir");
        exit(1);
      }
    }

    if (strcmp(ent->d_name, "..") == 0 || strcmp(ent->d_name, ".") == 0) {
      continue;
    }

    char* sbp = make_subpath(pathname, ent->d_name);

    switch (ent->d_type) {
      case DT_DIR:
        remove_dir(sbp);
        break;
      case DT_LNK:
      case DT_REG:
        remove_file(sbp);
        break;
      default:
        cs644_bail("not removing unknown file type");
    }

    free(sbp);
  }

  int r = rmdir(pathname);
  cs644_bail_if_err(r, "rmdir");
}

int main(int argc, char* argv[]) {
  if (argc != 2 || argv[1][0] == '-') {
    return 1;
  }

  remove_file_or_dir(argv[1]);

  return 0;
}
```

(★★) Write a program that takes in a file path and prints its permissions in rwxrwxrwx format.

```c
#include <stdio.h>
#include <sys/stat.h>
#include "cs644.h"

void print_perms(int p) {
  char r = p & 4 ? 'r' : '-';
  char w = p & 2 ? 'w' : '-';
  char x = p & 1 ? 'x' : '-';
  printf("%c%c%c", r, w, x);
}

int main(int argc, char* argv[]) {
  if (argc != 2 || argv[1][0] == '-') {
    return 1;
  }

  const char* filename = argv[1];
  struct stat statbuf;
  int r = stat(filename, &statbuf);
  cs644_bail_if_err(r, "stat");

  print_perms(statbuf.st_mode >> 6);
  print_perms(statbuf.st_mode >> 3);
  print_perms(statbuf.st_mode);
  puts("");

  return 0;
}
```

(★★) We have syscalls to move and delete files, but not to copy them. Why not?

Copying can be accomplished using open, read, and write; a dedicated syscall is not necessary. However, this is inefficient because you have to copy every byte twice: once from the kernel into userspace on read, and once again from userspace into the kernel on write. So Linux has a sendfile syscall that lets you directly transfer bytes between two open files.

(★★) What happens if you rename a file while another process is writing to it? Make a prediction, then write a pair of programs to demonstrate what happens.

Two reasonable predictions are that (a) it will continue writing to the new path, or (b) it will start writing to the old path. If you are doing the web server final project, you can use the logging command along with mv to test this. It turns out that (a) is correct – but only if the writing process keeps the file open. If it closes and reopens each time, it will keep writing to the old path.

(★★) Is getdents64 atomic? Write a pair of programs that demonstrates its behavior.

```c
#define _GNU_SOURCE
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>
#include "cs644.h"

const char* TMPDIR = "/tmp/getdents-test";

struct linux_dirent {
  unsigned long  d_ino;
  off_t          d_off;
  unsigned short d_reclen;
  char           d_name[];
};

void* test_atomic_getdents(void* data) {
  size_t n = 50000;
  char buf[n];

  int fd = open(TMPDIR, O_RDONLY | O_DIRECTORY);
  cs644_bail_if_err(fd, "open");

  for (int i = 0; i < 200; i++) {
    if ((i + 1) % 10 == 0) {
      printf("getdents: iter %d\n", i);
    }

    ssize_t bytes = getdents64(fd, buf, n);
    cs644_bail_if_err(bytes, "getdents64");

    struct linux_dirent* ent;
    int seen[1000] = {0};
    for (size_t bpos = 0; bpos < bytes;) {
      ent = (struct linux_dirent*)(buf + bpos);
      int i = atoi(ent->d_name);
      if (i != 0) {
        if (i > 1000) {
          i -= 1000;
        }

        if (seen[i]) {
          printf("getdents: saw both old and new versions of '%s'\n", ent->d_name);
          pthread_exit(NULL);
        }
        seen[i] = 1;
      }
      bpos += ent->d_reclen;
    }
  }

  int r = close(fd);
  cs644_bail_if_err(r, "close");

  return NULL;
}

void do_one_readdir() {
  DIR* dp = opendir(TMPDIR);
  struct dirent* ent;

  int seen[1000] = {0};
  while ((ent = readdir(dp)) != NULL) {
    int i = atoi(ent->d_name);
    if (i == 0) {
      continue;
    }

    if (i >= 1000) {
      i -= 1000;
    }

    if (seen[i]) {
      printf("readdir: saw both old and new versions of '%s'\n", ent->d_name);
      pthread_exit(NULL);
    }
    seen[i] = 1;
  }
}

void* test_atomic_readdir(void* data) {
  for (int i = 0; i < 200; i++) {
    if ((i + 1) % 10 == 0) {
      printf("readdir: iter %d\n", i);
    }

    do_one_readdir();
  }

  return NULL;
}

void shuffle_entries(char* old) {
  char new[100];
  snprintf(new, 100, "%04d", atoi(old) + 1000);

  unlink(new);
  int fd = open(old, O_WRONLY | O_CREAT | O_TRUNC, 0600);
  cs644_bail_if_err(fd, "open");
  int r = close(fd);
  cs644_bail_if_err(fd, "close");

  int i = 0;
  while (1) {
    r = rename(old, new);
    if (r < 0) {
      printf("rename (old -> new): %s (e=%s, s='%s', i=%d)\n",
          strerror(errno), strerrorname_np(errno), old, i);
      break;
    }

    r = rename(new, old);
    if (r < 0) {
      printf("rename (new -> old): %s (e=%s, s='%s', i=%d)\n",
          strerror(errno), strerrorname_np(errno), old, i);
      break;
    }
    i++;
  }
}

void* shuffle_entries_thrd(void* data) {
  shuffle_entries(data);
  return NULL;
}

int main() {
  int r = mkdir(TMPDIR, 0700);
  if (r < 0 && errno != EEXIST) {
    cs644_bail_if_err(r, "mkdir");
  }
  r = chdir(TMPDIR);
  cs644_bail_if_err(r, "chdir");

  pthread_t tid1;
  pthread_create(&tid1, NULL, test_atomic_getdents, NULL);
  pthread_t tid2;
  pthread_create(&tid2, NULL, test_atomic_readdir, NULL);

  for (int i = 1; i < 1000; i++) {
    size_t bufsz = 5;
    char* buf = cs644_malloc_or_bail(bufsz);
    snprintf(buf, bufsz, "%04d", i);
    pthread_t tid;
    pthread_create(&tid, NULL, shuffle_entries_thrd, buf);
  }

  pthread_join(tid1, NULL);
  pthread_join(tid2, NULL);
  return 0;
}
```

(★★) Linux file permissions are a little more complicated than what was presented here. Research the concepts of the set-uid, set-gid, and sticky bits.

Normally, when an executable is run on Linux, it will run with the user ID and group ID of the user who started it. But if the set-uid or set-gid bit is set in the executable file's st_mode, then it will instead run as the file's owner or group, respectively. Usually this is when an executable needs root permissions but should be available to non-root users. Because they allow privilege escalation, set-uid executables are a potential security hole and must be written very carefully. See section 8.11 of Advanced Programming in the Unix Environment for an example.

The sticky bit is another bit in st_mode that changes the interpretation of directory permissions: if set, then files in the directory can only be renamed or removed by the file's owner, the directory's owner, or the superuser (instead of anyone with +w permissions). Most commonly, the sticky bit is set on /tmp so that everyone can create files but not interfere with others' files. Tony Finch's blog post goes into more detail.

(★★★) Is it possible to atomically overwrite a file? First, write a pair of programs (a reader and a writer) that shows that simply overwriting with write isn't atomic. Then, find a way to do it atomically.

We demonstrated in class with simultaneous.c that writes are not atomic. To overwrite atomically, create a temporary file, write to it, and then use rename to atomically replace the destination file. It's important to create the temporary file on the same filesystem as the destination file (e.g., in the same directory), because rename does not work across filesystems.

(★★★) mv myfile.txt /tmp requires an additional permission beyond what we discussed. (NOTE: This turns out to not be true on the shared server, but if you have a macOS computer, you might be able to replicate it there.) What is it? Can you explain why it's necessary?

It also requires +r permission on myfile.txt. The reason is that /tmp is mounted as a separate filesystem, and rename cannot move files across filesystems. So the mv command instead reads the source file and writes it out to the destination, and this requires +r on the source file.

(★★★) Linux file permissions are a lot more complicated than what was presented here. Research file ACLs and SELinux contexts. What syscalls do they use?
