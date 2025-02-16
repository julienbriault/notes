import ctypes
from datetime import datetime

# pointer to the file containing the rust function:
library_path = "target/release/libpyru.so"


# the above file is relative to where this file is located.

# loading the library:
pyru = ctypes.CDLL(library_path)


def time_rust_function_calls_from_python():
    pyru.rust_says_hello()


if __name__ == "__main__":
    # calling the Rust function:
    i = 10_000
    n = i
    start = datetime.now()
    while i > 0:
        pyru.rust_says_hello()
        i -= 1
    end = datetime.now()
    print(f"called the Rust func {n} times in {end - start}")

"""
...
Hello from the Rust universe!
Hello from the Rust universe!
called the Rust func 10000 times in 0:00:00.063125


"""
