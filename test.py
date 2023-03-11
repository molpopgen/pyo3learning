import numpy as np

import pyo3learning


def test_add():
    x = pyo3learning.add(1, 7)
    assert x == 8


def test_make_holder():
    h = pyo3learning.HoldsVec([1, 2, 3])
    # This is annoying: the repr_ns is 'builtins'
    print(h)
    assert len(h) == 3
    h.append(77)
    assert len(h) == 4

    mv = memoryview(h)
    assert mv.readonly is False

    print([i for i in mv])

    mv[1] = -99

    print([i for i in mv])

    assert mv[1] == -99

    try:
        print(h[77])
    except IndexError as e:
        print(f"got expected exception: {e}")

    array = np.array(mv)
    print(array)
    print(array.dtype)
    array[0] = 3
    print(array)
    print(h[0])
    mv[0] = 7
    print(h[0])
    print(array)

    v = memoryview(h.view())
    v[3] = 101
    print(f"view = {[i for i in v]}")
    print(f"h = {[i for i in h]}")
    del h
    print(f"view = {[i for i in v]}")


if __name__ == "__main__":
    test_add()
    test_make_holder()
