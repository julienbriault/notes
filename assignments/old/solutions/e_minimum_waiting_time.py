tests = [
    [3, 2, 1, 2, 6],
]


def minimumWaitingTime(queries):
    queries.sort()
    print(queries[:-1])
    x = 0
    z = 0
    for i in queries[:-1]:
        z += i
        x += z
    return x


def minimumWaitingTime_second(queries):
    queries.sort()

    wait = 0
    for i in queries[:-1]:
        addition = queries.pop(0) * len(queries)
        wait += addition

    return wait


def minimumWaitingTime_third(queries):
    queries.sort()

    return sum(q * (len(queries) - (i + 1)) for i, q in enumerate(queries))


if __name__ == "__main__":
    print("num1")
    for i, test in enumerate(tests, start=1):
        print(f"Test number {i}")
        print(minimumWaitingTime(test))
    print("num2")
    for i, test in enumerate(tests, start=1):
        print(f"Test number {i}")
        print(minimumWaitingTime_second(test))