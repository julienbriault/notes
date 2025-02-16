"""
On using exceptions:
- be as precise about the exception as possible
- capture the exception if you can write somthing to handle it or to log it
- use Exceptions you wrote


https://realpython.com/python-exceptions/

"""


class MyError(Exception):
    pass


class MyDetailedError(Exception):
    def __init__(self, *args: object) -> None:
        super().__init__(*args)


class MyDetailedError2(Exception):
    def __init__(self, errno, msg) -> None:
        self.args = (errno, msg)
        self.errno = errno
        self.msg = msg


def try_something():
    pass


# straightforward try/except:
try:
    try_something()
except Exception as e:
    print(e)


# handle multiple exceptions:
try:
    try_something()
except (LookupError, OSError) as e:
    print(e)

# handle multiple exceptions separately:

try:
    file = open("somefile", "r")
except EOFError as e:
    raise e
except IOError as e:
    raise e

try:
    raise ValueError("Incorrect value", "1", "2", "3")
except ValueError as err:
    print(err.args)