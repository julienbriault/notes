def strings_concatenate_1():
    """Concatenate strings"""
    s1 = "string 1" + "string 2"

def strings_concatenate_2():
    """Concatenate strings"""
    s1 = "string 1"
    s2 = "string 2"
    s1 = s1.join(s2)
    return s1

def strings_concatenate_3():
    """Concatenate strings"""
    return f'string 1string 2'

if __name__ == '__main__':
    import timeit
    print(timeit.timeit("strings_concatenate_1()", setup="from __main__ import strings_concatenate_1"))

    print(timeit.timeit("strings_concatenate_2()", setup="from __main__ import strings_concatenate_2"))

    print(timeit.timeit("strings_concatenate_3()", setup="from __main__ import strings_concatenate_3"))