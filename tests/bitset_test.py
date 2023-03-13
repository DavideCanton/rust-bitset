import re

import pytest
from rust_bitset import BitSet


@pytest.mark.parametrize(
    "size, err, msg",
    [
        (0, ValueError, "size cannot be zero"),
        (-1, OverflowError, "can't convert negative int to unsigned"),
    ],
)
def test_invalid_size(size, err, msg):
    with pytest.raises(err, match=re.escape(msg)):
        BitSet(size)


@pytest.mark.parametrize("op", ["get", "set"])
def test_invalid_index(op):
    b = BitSet(4)
    with pytest.raises(IndexError, match="index can be between 0 and 3, found 5"):
        if op == "get":
            b[5]
        else:
            b[5] = True


def test_get_set():
    b = BitSet(4)
    assert len(b) == 4

    _check_indexes(b, {})
    b[0] = True
    _check_indexes(b, {0})
    b[2] = True
    _check_indexes(b, {0, 2})
    b[0] = False
    _check_indexes(b, {2})
    b[-2] = False
    _check_indexes(b, {})


def _check_indexes(b: BitSet, inside: set[int]):
    for i in range(len(b)):
        if i in inside:
            assert b[i]
            assert i in b
        else:
            assert not b[i]
            assert i not in b

    assert repr(b) == repr(sorted(inside))
    assert b.elements() == frozenset(inside)
