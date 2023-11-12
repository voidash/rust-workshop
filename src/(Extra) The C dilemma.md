Extracted from : https://google.github.io/comprehensive-rust/why-rust/an-example-in-c.html

````c
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>

int main(int argc, char* argv[]) {
	char *buf, *filename;
	FILE *fp;
	size_t bytes, len;
	struct stat st;

	switch (argc) {
		case 1:
			printf("Too few arguments!\n");
			return 1;

		case 2:
			filename = argv[argc];
			stat(filename, &st);
			len = st.st_size;
			
			buf = (char*)malloc(len);
			if (!buf)
				printf("malloc failed!\n", len);
				return 1;

			fp = fopen(filename, "rb");
			bytes = fread(buf, 1, len, fp);
			if (bytes = st.st_size)
				printf("%s", buf);
			else
				printf("fread failed!\n");

		case 3:
			printf("Too many arguments!\n");
			return 1;
	}

	return 0;
}

````

Can you find all the bugs? There are 11 of them

1. Assignment `=` instead of equality comparison `==` (line 28)
1. Excess argument to `printf` (line 23)
1. File descriptor leak (after line 26)
1. Forgotten braces in multi-line `if` (line 22)
1. Forgotten `break` in a `switch` statement (line 32)
1. Forgotten NUL-termination of the `buf` string, leading to a buffer overflow (line 29)
1. Memory leak by not freeing the `malloc`-allocated buffer (line 21)
1. Out-of-bounds access (line 17)
1. Unchecked cases in the `switch` statement (line 11)
1. Unchecked return values of `stat` and `fopen` (lines 18 and 26)

try copying and running it online on [online GDB](https://www.onlinegdb.com/online_c_compiler). It will compile fine. But these are all serious bugs, how come the compiler accepts just like that.... and these errors are serious

* Assignment `=` instead of equality comparison `==`: [The Linux Backdoor Attempt of 2003](https://freedom-to-tinker.com/2013/10/09/the-linux-backdoor-attempt-of-2003)
* Forgotten braces in multi-line `if`: [The Apple goto fail vulnerability](https://dwheeler.com/essays/apple-goto-fail.html)
* Forgotten `break` in a `switch` statement: [The break that broke sudo](https://www.lufsec.com/anatomy-of-a-security-hole-the-break-that-broke-sudo/)

### How can Rust be better here?

1. Assignments inside an `if` clause are not supported.
1. Format strings are checked at compile-time.
1. Resources are freed at the end of scope via the `Drop` trait.
1. All `if` clauses require braces.
1. `match` (as the Rust equivalent to `switch`) does not fall-through, hence you can’t accidentally forget a `break`.
1. Buffer slices carry their size and don’t rely on a NUL terminator.
1. Heap-allocated memory is freed via the `Drop` trait when the corresponding `Box` leaves the scope.
1. Out-of-bounds accesses cause a panic or can be checked via the `get` method of a slice.
1. `match` mandates that all cases are handled.
1. Fallible Rust functions return `Result` values that need to be unwrapped and thereby checked for success. Additionally, the compiler emits a warning if you miss to check the return value of a function marked with `#[must_use]`.
