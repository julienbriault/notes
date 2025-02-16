# pip install timeout-decorator
# does not work on windows.
import time
import timeout_decorator


@timeout_decorator.timeout(5, use_signals=False)
def mytest():
    print("Start")
    for i in range(1, 10):
        time.sleep(1)
        print("{} seconds have passed".format(i))


if __name__ == "__main__":
    mytest()