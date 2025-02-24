Systems Programming: writing software that is used by other software

Course Focus: 
Linux system interface
- files
- network sockets
- shared memory

Syllabus

```c
#include <stdio.h>

enum Role {
	ROLE_DEVELOPER,
	ROLE_DESIGNER,
	ROLE_MANAGER,
};



struct Employee {
	unsigned int employee_id;
	const char* name;
	enum Role role;
};

int main() {
	struct Employee employees[] = {
	 { .employee_id = 0, .name = "Dana Designer", .role = ROLE_DESIGNER },
	 { .employee_id = 1, .name = "Davy Developer", .role = ROLE_DEVELOPER },
	 { .employee_id = 1, .name = "Meredith Manager", .role = ROLE_DEVELOPER },
	};
	return 0;
}


```

In-Class Exercises

bat (better cat)
