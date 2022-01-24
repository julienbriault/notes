tests = [
    "abcdcaf",
    "abcdbaf",
]


def nonRepeating(s):
    if len(s) <= 1:
        return -1
    seen = {}
    for i, c in enumerate(s):
        seen[c] = len(s) + 1 if c in seen else i
    lowest = min(seen.values())

    return -1 if lowest > len(s) else lowest


if __name__ == "__main__":
    print("num1")
    for i, test in enumerate(tests, start=1):
        print(f"Test number {i}")
        print(nonRepeating(test))
