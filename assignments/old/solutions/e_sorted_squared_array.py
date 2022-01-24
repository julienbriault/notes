import ipdb

tests = [
    [1, 2, 3, 4, 5, 6, 7, 8, 9],
    [],
    [-2, -1],
]


def sortedSquaredArray(array):
    new_array = []
    sort = False
    for i in array:
        if i < 0:
            sort = True
        new_array.append(i * i)

    if sort:
        print("hitit")
        new_array.sort()
    return new_array


def sortedSquaredArray2(array):
    new_array = [x * x for x in array]
    new_array.sort()
    return new_array


if __name__ == "__main__":
    print("num1")
    for i, test in enumerate(tests, start=1):
        print(f"Test number {i}")
        print(sortedSquaredArray(test))

    print("num2")
    for i, test in enumerate(tests, start=1):
        print(f"Test number {i}")
        print(sortedSquaredArray2(test))
