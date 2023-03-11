import pyo3learning


def test_add():
    x = pyo3learning.add(1, 7)
    assert x == 8


def test_make_holder():
    h = pyo3learning.HoldsVec([1, 2, 3])
    # This is annoying: the repr_ns is 'builtins'
    print(h)
    assert len(h) == 3
    h.append(4)
    assert len(h) == 4


if __name__ == "__main__":
    test_add()
    test_make_holder()
