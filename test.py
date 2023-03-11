import pyo3learning


def test_add():
    x = pyo3learning.add(1, 7)
    assert x == 8


if __name__ == "__main__":
    test_add()
