tests = [
    [[-1, 5, 10, 20, 28, 3], [26, 134, 135, 15, 17]],
]


def smallestDifference(arrayOne, arrayTwo):

    results = []

    for indexO, valueO in enumerate(arrayOne):
        for indexT, valueT in enumerate(arrayTwo):
            diff = abs(valueO - valueT)
            curr = [indexO, indexT, diff]
            # print(curr)
            if results and diff < results[2] or len(results) <= 0:
                results = curr
    first = results[0]
    second = results[1]
    return [arrayOne[first], arrayTwo[second]]


if __name__ == "__main__":
    print("num1")
    for i, test in enumerate(tests, start=1):
        print(f"Test number {i}")
        print(smallestDifference(test[0], test[1]))
