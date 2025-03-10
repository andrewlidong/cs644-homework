Types of files
Linux distinguishes between several types of files:

Regular file: A simple sequence of bytes. (Linux doesn't distinguish between binary and text files.)
Directory: A listing of other files (which of course may themselves be directories).
Symbolic link: A file that "points" to another file. Most syscalls follow symlinks by default, so if I create a symlink dir1/a that points to dir2/b, then calling open("dir1/a") will have the same effect as calling open("dir2/b"). If the original file is removed, then the symlink will be left dangling.
Hard link: Another kind of filesystem link, with three major differences from symlinks:
The hard link and the original file are completely identical, and in fact there is no "original" – the two are indistinguishable.
Hard links can't be left dangling; the file contents will be kept alive until all links to it are removed.
Creating a hard link to a directory is a bad idea.
This list is not exhaustive, but these are the key file types we are concerned with now.

The classic Unix permissions model
Linux has inherited the classic file permissions model from Unix. There are three permission bits, each of which has a different meaning for regular files versus directories:

Read
Files: can read from file
Directories: can list directory contents
Write
Files: can write to file
Directories: can create, delete, and move entries
Execute
Files: can execute as program
Directories: can "traverse", i.e. access paths within the directory
Execute for directories
The execute permission is really a misnomer for directories: it has nothing to do with execution, they just appropriated an otherwise meaningless bit.

The execute permission for directories is tricky. It really has nothing to do with execution; since the bit was meaningless for directories, they appropriated it for something unrelated. It seems similar to read, and indeed usually the read and execute bits for directories have the same value, but all four combinations of the two bits are possible:

r-x – can both list the directory and access paths within in (this is the normal case)
r-- – can list the directory, but can't access any of the paths
--x – can access paths within the directory, but can't get a list of all the paths
--- – no permission on the directory at all
Owners and groups
Every file has an owner and a group. The three permissions (read, write, execute) can be separately set for the owner, the group, and everyone else. We can write a file's permission as a nine-character string, such as rwxr-xr-x, where the first three bits are the owner's permissions, the next bits the group's, and the last bits everyone else's.

Example: Suppose I'm a professor teaching a CS course, and I have a file of homework solutions. I should have read and write access to the file. My TAs, who belong to the teaching_assistants group, should have read-only access. Everyone else should have no access at all. Therefore, my file permissions should look like:

$ ls -l solutions.txt
-rw-r----- ian teaching_assistants ... solutions.txt
(The initial dash is how ls indicates the file is not a directory.)

Octal notation
Instead of a nine-character string, we can represent permissions more concisely as a three-digit octal number, for example 755. To break it down:

755 in octal is 111 101 101 in binary.
So this is equivalent to rwx r-x r-x.
Another way to remember is that r = 4, w = 2, and x = 1, so 7 = 4 + 2 + 1 = rwx.
Some common permissions are:

755 = rwxr-xr-x = only owner can write, but anyone can read or execute (directories and executable files)
644 = rw-r--r-- = only owner can write, but anyone can read (non-executable files)
600 = rw------- = only owner can read and write, no one else can access
400 = r-------- = only owner can read, no one can write (read-only files)
Bonus: chmod abbreviations
The chmod shell command understands some abbreviations:

# give the owner ('user') write permission
$ chmod u+x myfile.txt
# remove the owner's write permission
$ chmod u-x myfile.txt
Unfortunately they are easy to get confused:

u stands for 'user'
g stands for 'group', not global
o stands for 'other', not owner
a stands for 'all', meaning user and group and other, not just other
Syscalls: File metadata and permissions
struct stat {
  mode_t st_mode;
  uid_t  st_uid;
  gid_t  st_gid;
  off_t  st_size;
  /* other fields */
};

int stat(const char* pathname, struct stat* statbuf);
int fstat(int fd, struct stat* statbuf);
stat returns metadata about a file. Notably:
the mode, which includes what type of file it is (regular file, directory, etc.) and its permissions
the owner and group
the size of the file in bytes
fstat is like stat except it takes a file descriptor instead of a path name.
int chmod(const char* pathname, mode_t mode);
int fchmod(int fd, mode_t mode);
chmod and fchmod let you change a file's permissions.
You must be the owner of the file (or root) to do this.
Syscalls: Directories
int mkdir(const char* pathname, mode_t mode);
mkdir creates a directory with the permissions specified in mode.
Unlike open, it does not return a file descriptor.
The new directory's owner will be set to the effective user ID of the process (same as for creating regular files with open).
struct linux_dirent {
    char d_name[];
    char d_type;
    /* other fields */
};

// raw syscall
ssize_t getdents64(int fd, void* dirp, size_t count);

// libc wrapper
DIR* opendir(const char* pathname);
struct dirent* readdir(DIR* dirp);
getdents64 is the raw syscall to get the entries of a directory.
Quoting man 2 getdents: "These are not the interfaces you are interested in."
You are supposed to use opendir and readdir from libc instead, and languages other than C may not provide a getdents64 interface at all (Python only has os.scandir, for instance).
Still, getdents64 isn't hard to understand. You pass in a file descriptor, an array to hold the entries, and a count (of bytes, not of array entries), and it fills the array and returns the number of bytes read.
This is because struct linux_dirent values are not fixed in size, so if it returned the number of entries read you wouldn't know where the end of your array is.
At any rate, 0 is returned at the end of the directory.
The man page has a lot of gory details about struct layout, but these only apply to kernels older than Linux 2.4, which was released in 2001.
opendir and readdir are less awkward than getdents64, but have the disadvantage of only returning one entry at a time.
struct dirent is similar to struct linux_dirent; read the man page if you're interested.
Syscalls: Moving and deleting files
int rename(const char* oldpath, const char* newpath);
rename is used both to rename and move files (from the kernel's standpoint these are the same thing).
If newpath already exists, it will be replaced atomically, meaning that there is no interval where newpath temporarily ceases to exist, and if the rename fails then newpath will be untouched.
The move itself is not atomic, i.e. there will possibly be a time when both oldpath and newpath point to the file being renamed.
Caveat: oldpath and newpath must be on the same filesystem. Otherwise, the kernel would not be able to do the rename atomically.
int unlink(const char* pathname);
unlink removes a file.
Why is it called unlink instead of remove? We'll find out in a minute when we talk about hard links and symlinks.
int rmdir(const char* pathname);
rmdir deletes an empty directory. It will not work if the directory has any files in it.
Syscalls: File locking
const int LOCK_SH = /* */;
const int LOCK_EX = /* */;
const int LOCK_UN = /* */;
const int LOCK_NB = /* */;

int flock(int fd, int op);
flock places or removes a lock on a file.
A lock can be shared (LOCK_SH) or exclusive (LOCK_EX). A file can have either N shared locks or 1 exclusive lock at a time.
Typically, shared locks are for readers and exclusive locks for writers.
Pass LOCK_UN as the operation to release the lock.
By default, flock will block until the lock is available. You can combine LOCK_NB with the lock options to make it non-blocking, in which case it will return immediately, with EWOULDBLOCK if the lock was already held.
It is an advisory lock, meaning that the kernel won't stop other processes from accessing the file unless they also try to acquire the lock.
It's good for groups of cooperating processes that all agree to use flock.
If you need to protect against uncooperative processes, use file permissions instead (or in addition).
